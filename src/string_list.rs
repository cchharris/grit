/**
	string_list is a C-like structure to hold a list of strings
	We may or may dupe strings and thus own their memory, depending on a flag set at construction, and they may or may not be sorted
	This flag appears to be set at initialization time - so we'll use generics/specializaiton and swap out the implementation.

	I'm going to model this by using String vs. &str, which are owned versus unowned strings, appropriately.
	I'm trusting that Rust will steer us right if we try and use StringListRef and it becomes unsafe.
**/

struct StringListBase<'a,S> where S: AsRef<str> {
	list: Vec<&'a S>,
	compare: Option<&'a dyn Fn(&str, &str) -> u32>,
}

type StringListRef<'a> = StringListBase<'a, &'a str>;
type StringListDupe<'a> = StringListBase<'a, String>;

// The function we're calling will not own the strings.
// The git implementation has a void* cb_data.  Not filling this in until I know what it's for.'
type StringListEachFunc = dyn Fn(&str) -> u32; 

impl <'a, S: std::convert::AsRef<str>> StringListBase<'a, S> {
	fn new() -> StringListBase<'a, S> {
		Self {
			list: Vec::new(),
			compare: None
		}
	}

	fn clear(&mut self) {
		self.list.clear();
		self.list.truncate(0);
	}

	fn forEach(&self, func: &StringListEachFunc) -> u32 {
		for item in self.list.iter() {
			let ret = func(item.as_ref());
			if ret != 0 {
				return ret;
			}
		}
		return 0;
	}
}

impl <'a> StringListRef<'a> {
}

impl <'a> StringListDupe<'a> {
}