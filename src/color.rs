
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

enum ColorType {
	ColorUnspecified = 0,
	ColorNormal = 1,
	ColorAnsi = 2, /* basic 0-7 ANSI colors + "default" (value = 9) */
	Color256 = 3,
	ColorRGB = 4,
}

pub struct Color {
	color_type: ColorType, 
	value: u8,
	red: u8,
	green: u8,
	blue: u8
}

/**
	https://github.com/git/git/commit/e49521b56d8715f46b93ee6bc95f7de9c6858365
	Git seems to use a lookup table to get the int value of a character
	We can use to_digit with a hex radix, and map Nones to the git value of (uint)-1
	TODO: We might want to look for None instead of a (uint)-1 flag.
**/
fn hexval(c: char) -> u32 {
	c.to_digit(16).unwrap_or_else(u32::max_value)
}

impl Color {
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

}
