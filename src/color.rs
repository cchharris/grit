
pub const COLOR_MAXLEN: i32 = 75;
pub const GIT_COLOR_NORMA: &str = "";
pub const GIT_COLOR_RESET: &str = "\033[m";
pub const GIT_COLOR_BOLD: &str = "\033[1m";
pub const GIT_COLOR_BLACK: &str = "\033[30m";
pub const GIT_COLOR_RED: &str = "\033[31m";
pub const GIT_COLOR_GREEN: &str = "\033[32m";
pub const GIT_COLOR_YELLOW: &str = "\033[33m";
pub const GIT_COLOR_BLUE: &str = "\033[34m";
pub const GIT_COLOR_MAGENTA: &str = "\033[35m";
pub const GIT_COLOR_CYAN: &str = "\033[36m";
pub const GIT_COLOR_WHITE: &str = "\033[37m";
pub const GIT_COLOR_DEFAULT: &str = "\033[39m";
pub const GIT_COLOR_BOLD_BLACK: &str = "\033[1;30m";
pub const GIT_COLOR_BOLD_RED: &str = "\033[1;31m";
pub const GIT_COLOR_BOLD_GREEN: &str = "\033[1;32m";
pub const GIT_COLOR_BOLD_YELLOW: &str = "\033[1;33m";
pub const GIT_COLOR_BOLD_BLUE: &str = "\033[1;34m";
pub const GIT_COLOR_BOLD_MAGENTA: &str = "\033[1;35m";
pub const GIT_COLOR_BOLD_CYAN: &str = "\033[1;36m";
pub const GIT_COLOR_BOLD_WHITE: &str = "\033[1;37m";
pub const GIT_COLOR_BOLD_DEFAULT: &str = "\033[1;39m";
pub const GIT_COLOR_FAINT_BLACK: &str = "\033[2;30m";
pub const GIT_COLOR_FAINT_RED: &str = "\033[2;31m";
pub const GIT_COLOR_FAINT_GREEN: &str = "\033[2;32m";
pub const GIT_COLOR_FAINT_YELLOW: &str = "\033[2;33m";
pub const GIT_COLOR_FAINT_BLUE: &str = "\033[2;34m";
pub const GIT_COLOR_FAINT_MAGENTA: &str = "\033[2;35m";
pub const GIT_COLOR_FAINT_CYAN: &str = "\033[2;36m";
pub const GIT_COLOR_FAINT_WHITE: &str = "\033[2;37m";
pub const GIT_COLOR_FAINT_DEFAULT: &str = "\033[2;39m";
pub const GIT_COLOR_BG_BLACK: &str = "\033[40m";
pub const GIT_COLOR_BG_RED: &str = "\033[41m";
pub const GIT_COLOR_BG_GREEN: &str = "\033[42m";
pub const GIT_COLOR_BG_YELLOW: &str = "\033[43m";
pub const GIT_COLOR_BG_BLUE: &str = "\033[44m";
pub const GIT_COLOR_BG_MAGENTA: &str = "\033[45m";
pub const GIT_COLOR_BG_CYAN: &str = "\033[46m";
pub const GIT_COLOR_BG_WHITE: &str = "\033[47m";
pub const GIT_COLOR_BG_DEFAULT: &str = "\033[49m";
pub const GIT_COLOR_FAINT: &str = "\033[2m";
pub const GIT_COLOR_FAINT_ITALIC: &str = "\033[2;3m";
pub const GIT_COLOR_REVERSE: &str = "\033[7m";
pub const GIT_COLOR_NIL: &str = "NIL";

pub const GIT_COLOR_UNKNOWN: i32 = -1;
pub const GIT_COLOR_NEVER: i32 = 0;
pub const GIT_COLOR_ALWAYS: i32 = 1;
pub const GIT_COLOR_AUTO: i32 = 2;

pub const COLUMN_COLORS_ANSI: &[&str] = &[
	GIT_COLOR_RED,
	GIT_COLOR_GREEN,
	GIT_COLOR_YELLOW,
	GIT_COLOR_BLUE,
	GIT_COLOR_MAGENTA,
	GIT_COLOR_CYAN,
	GIT_COLOR_BOLD_RED,
	GIT_COLOR_BOLD_GREEN,
	GIT_COLOR_BOLD_YELLOW,
	GIT_COLOR_BOLD_BLUE,
	GIT_COLOR_BOLD_MAGENTA,
	GIT_COLOR_BOLD_CYAN,
	GIT_COLOR_RESET,
];

const COLUMN_COLORS_ANSI_MAX :usize = COLUMN_COLORS_ANSI.len() - 1;

pub const COLOR_BACKGROUND_OFFSET:i32 = 10;
pub const COLOR_FOREGROUND_ANSI: i32 = 30;
pub const COLOR_FOREGROUND_RGB: i32 = 38;
pub const COLOR_FOREGROUND_256: i32 = 38;
pub const COLOR_FOREGROUND_BRIGHT_ANSI: i32 = 90;


struct ValueColor {
	value: u8
}

struct ValueRGB {
	red: u8,
	green: u8,
	blue: u8
}

enum Color {
	Unspecified,
	Normal,
	Ansi(ValueColor),
	U256(ValueColor), // U required since identifiers can't start with a number
	RGB(ValueRGB)
}

/*
	Is this even required??
	https://www.ibm.com/docs/en/i/7.2?topic=functions-strncasecmp-compare-strings-without-case-sensitivity

	Description:
	The strncasecmp() function compares up to count characters of string1 and string2 without sensitivity to case. All alphabetic characters in string1 and string2 are converted to lowercase before comparison.

	The strncasecmp() function operates on null terminated strings. The string arguments to the function are expected to contain a null character ('\0') marking the end of the string.

	/*
	 * "word" is a buffer of length "len"; does it match the NUL-terminated
	 * "match" exactly?
	 */
	static int match_word(const char *word, int len, const char *match)
	{
		return !strncasecmp(word, match, len) && !match[len];
	}
*/

/**
	https://github.com/git/git/commit/e49521b56d8715f46b93ee6bc95f7de9c6858365
	Git seems to use a lookup table to get the int value of a character
	We can use to_digit with a hex radix, and map Nones to the git value of (uint)-1
**/
fn try_hexval(c: char) -> Option<u32> {
	c.to_digit(16)
}

/**
Original code:
	static int get_hex_color(const char *in, unsigned char *out)
	{
		unsigned int val;
		val = (hexval(in[0]) << 4) | hexval(in[1]);
		if (val & ~0xff)
			return -1;
		*out = val;
		return 0;
	}

We swap to a char tuple, since we need two values
chars are 4 bytes though, so we'll see if this is typing we want to keep


if (val & ~0xff) is a fancy way of saying any bits set above 0b000000011111111
Aka. either of these hexvals returned (uint)-1 
We can just check for > 255 as well
And instead of returning a flag value and passing a location, return an Option
**/
fn try_get_hex_color(i: (char, char)) -> Option<u32> {
	let mut val;
	match try_hexval(i.0) {
		Some(a) => val = a << 4,
		None => return None,
	}
	match try_hexval(i.1) {
		Some(a) => val |= a,
		None => return None,
	}
	if val <= u8::MAX.into() {
		Some(val)
	}
	else {None}
}

impl ValueColor {
	fn new(val: u8)  -> Self {
		Self {
			value: val,
		}
	}
}


impl Color {

	fn try_parse_ansi_color(name: &str, len: usize) -> Option<Color> {
		/* Positions in array must match ANSI color codes */
		const COLOR_NAMES: [&str; 8]= [
			"black", "red", "green", "yellow",
			"blue", "magenta", "cyan", "white"
		];

		let mut color_offset = COLOR_FOREGROUND_ANSI;

		if name[0..usize::min(len, name.len())].eq_ignore_ascii_case("default") {
			return Some(Color::Ansi(ValueColor::new((color_offset + 9).try_into().unwrap())));
		}

		let mut color_name = &name[..];
		let mut name_len = usize::min(len, color_name.len());
		if color_name[0..usize::min(6, name_len)].eq_ignore_ascii_case("bright") {
			color_offset = COLOR_FOREGROUND_BRIGHT_ANSI;
			color_name = &name[6..];
			name_len -= 6;
		}
		for (i, color) in COLOR_NAMES.iter().enumerate() {
			if color_name[0..name_len].eq_ignore_ascii_case(color) {
				return Some(Color::Ansi(ValueColor::new((color_offset + i32::try_from(i).unwrap()).try_into().unwrap())));
			}
		}
		None
	}

}

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn test_hexval() {
		let numbers = '0'..='9';
		let numbers_result = 0..=9;
		let letters = 'a'..='z';
		let letters_upper = 'A'..='Z';
		let letters_result = [Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];

		let mut numbers_result_it = numbers_result.into_iter();
		for num in numbers {
			assert_eq!(try_hexval(num), Some(numbers_result_it.next().unwrap()));
		}

		let mut letters_result_iter = letters_result.iter();
		for letter in letters {
			assert_eq!(try_hexval(letter), *letters_result_iter.next().unwrap());
		}

		let mut letters_result_iter = letters_result.iter();
		for letter in letters_upper {
			assert_eq!(try_hexval(letter), *letters_result_iter.next().unwrap());
		}
	}

	#[test]
	fn test_get_hex_color() {
		let test_0 = ('0', '0');
		let test_0_ret = 0u32;
		assert_eq!(try_get_hex_color(test_0), Some(test_0_ret));

		let test_1 = ('z', '0');
		assert_eq!(try_get_hex_color(test_1), None);

		let test_2 = ('0', 'z');
		assert_eq!(try_get_hex_color(test_2), None);

		let test_3 = ('f', 'f');
		let test_3_ret = u8::MAX.into();
		assert_eq!(try_get_hex_color(test_3), Some(test_3_ret));

		let test_4 = ('4', '4');
		let test_4_ret = (4*16) + 4;
		assert_eq!(try_get_hex_color(test_4), Some(test_4_ret));
	}

	#[test]
	fn test_try_parse_ansi_color() {
		let default = "default";
		let default_res = Color::try_parse_ansi_color(default, default.len());
		match default_res {
			Some(Color::Ansi(inner)) => assert_eq!(inner.value, (COLOR_FOREGROUND_ANSI + 9).try_into().unwrap()),
			_ => assert!(false),
		}

		let green = "green";
		let green_res = Color::try_parse_ansi_color(green, 255);
		match green_res {
			Some(Color::Ansi(inner)) => assert_eq!(inner.value, (COLOR_FOREGROUND_ANSI + 2).try_into().unwrap()),
			_ => assert!(false),
		}

		let brightgreen = "brightgreen";
		let brightgreen_res = Color::try_parse_ansi_color(brightgreen, 255);
		match brightgreen_res {
			Some(Color::Ansi(inner)) => assert_eq!(inner.value, (COLOR_FOREGROUND_BRIGHT_ANSI + 2).try_into().unwrap()),
			_ => assert!(false),
		}

		let typogreen = "brigtgreen";
		let typogreen_res = Color::try_parse_ansi_color(typogreen, 255);
		match typogreen_res {
			None => assert!(true),
			_ => assert!(false),
		}

		let shortgreen = "brightgreen";
		let shortgreen_res = Color::try_parse_ansi_color(shortgreen, 6);
		match shortgreen_res {
			None => assert!(true),
			_ => assert!(false),
		}

		let shortergreen = "brightgreen";
		let shortergreen_res = Color::try_parse_ansi_color(shortergreen, 5);
		match shortergreen_res {
			None => assert!(true),
			_ => assert!(false),
		}
	}
}
