/**
	string_list is a C-like structure to hold a list of strings
	We may or may dupe strings and thus own their memory, depending on a flag set at construction, and they may or may not be sorted
	This flag appears to be set at initialization time - so we'll use generics/specializaiton and swap out the implementation.

	I'm going to model this by using String vs. &str, which are owned versus unowned strings, appropriately.
	I'm trusting that Rust will steer us right if we try and use StringListRef and it becomes unsafe.
**/

pub struct StringListBase<'a, S> where S: AsRef<str> + From<&'a str> + Ord {
	list: Vec<S>,
	compare: Option<&'a dyn Fn(&S, &S) -> std::cmp::Ordering>,
}

pub type StringListRef<'a> = StringListBase<'a, &'a str>;
pub type StringListDupe<'a> = StringListBase<'a, String>;

// The function we're calling will not own the strings.
// The git implementation has a void* cb_data.  Not filling this in until I know what it's for.'
type StringListEachFunc = fn(&str) -> i32; 
type StringListKeepFunc = fn(&str) -> bool;

impl <'a, S: std::convert::AsRef<str> + std::convert::From<&'a str> + std::default::Default + std::cmp::Ord> StringListBase<'a, S> {
	fn new() -> StringListBase<'a, S> {
		Self {
			list: Vec::new(),
			compare: None
		}
	}

	fn clear(&mut self) {
		self.list.clear();
	}

	fn for_each(&self, func: &StringListEachFunc) -> i32 {
		for item in self.list.iter() {
			let ret = func(item.as_ref());
			if ret != 0 {
				return ret;
			}
		}
		return 0;
	}

	fn filter(&mut self, func: &StringListKeepFunc) {
		let mut count_dest = 0;
		for i in 0..self.list.len() {
			if func(&self.list[i].as_ref()) {
				if count_dest != i {
					self.list[count_dest] = std::mem::take(&mut self.list[i]);
				}
				count_dest+=1;
			}
		}
		self.list.truncate(count_dest);
	}

	fn sort(&mut self) {
		self.list.sort_by(self.compare.unwrap_or(&(S::cmp as fn(&S, &S)->std::cmp::Ordering)))
	}

	fn unsorted_lookup(&self) {
		let comparer = self.compare.unwrap_or(&(S::cmp as fn(&S, &S)->std::cmp::Ordering));
	}

	fn iter(&self) -> std::slice::Iter<'_, S> {
		self.list.iter()
	}
}

impl <'a> StringListRef<'a> {
	fn append(&mut self, str_in: &'a str) {
		self.list.push(str_in)
	}
}

impl <'a> StringListDupe<'a> {
	fn append<'b>(&mut self, str_in: &'b str) {
		self.list.push(String::from(str_in))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ref() {
		let longer = "test0";
		let mut list_ref = StringListRef::new();
		list_ref.append(&longer);
		list_ref.append("test1");
		list_ref.append("test2");
		let other = "test3";
		list_ref.append(&other);

		/* Won't compile due to borrowing from a not long enough lifetime
		{
			let me = String::from("text");
			list_ref.append(&me);
		}
		*/

		let mut iter = list_ref.iter();
		assert_eq!(&longer, iter.next().unwrap());
		assert_eq!(&"test1", iter.next().unwrap());
		assert_eq!(&"test2", iter.next().unwrap());
		assert_eq!(&other, iter.next().unwrap());
	}

	#[test]
	fn test_dupe() {
		let longer = "test0";
		let mut list_dupe = StringListDupe::new();
		list_dupe.append(&longer);
		list_dupe.append("test1");
		list_dupe.append("test2");
		let other = "test3";
		list_dupe.append(&other);

		{ // OK - we copy the string, so me can Drop and 
			let will_drop = String::from("test4");
			list_dupe.append(&will_drop);
		}

		let mut iter = list_dupe.iter();
		assert_eq!(&longer, iter.next().unwrap());
		assert_eq!(&"test1", iter.next().unwrap());
		assert_eq!(&"test2", iter.next().unwrap());
		assert_eq!(&other, iter.next().unwrap());
		assert_eq!(&"test4", iter.next().unwrap());
	}

	#[test]
	fn sort() {
		let first = "item1";
		let second = "item1";
		let third = "item1";

		let mut list_ref = StringListRef::new();
		let mut list_dupe = StringListDupe::new();

		list_ref.append(&third);
		list_ref.append(&first);
		list_ref.append(&second);

		let mut refUnsortedIter = list_ref.iter();
		assert_eq!(&third, refUnsortedIter.next().unwrap());
		assert_eq!(&first, refUnsortedIter.next().unwrap());
		assert_eq!(&second, refUnsortedIter.next().unwrap());

		list_dupe.append(&third);
		list_dupe.append(&first);
		list_dupe.append(&second);
		let mut dupeUnsortedIter = list_dupe.iter();
		assert_eq!(&third, dupeUnsortedIter.next().unwrap());
		assert_eq!(&first, dupeUnsortedIter.next().unwrap());
		assert_eq!(&second, dupeUnsortedIter.next().unwrap());

		list_ref.sort();
		let mut refSortedIter = list_ref.iter();
		assert_eq!(&first, refSortedIter.next().unwrap());
		assert_eq!(&second, refSortedIter.next().unwrap());
		assert_eq!(&third, refSortedIter.next().unwrap());

		list_dupe.sort();
		let mut dupeSortedIter = list_dupe.iter();
		assert_eq!(&first, dupeSortedIter.next().unwrap());
		assert_eq!(&second, dupeSortedIter.next().unwrap());
		assert_eq!(&third, dupeSortedIter.next().unwrap());
	}

	#[test]
	fn sort_user_compare() {
		let first = "item1";
		let second = "item2";
		let third = "item3";

		let mut list_ref = StringListRef::new();
		let list_ref_ordering: fn(&&str, &&str) -> std::cmp::Ordering = |a: &&str, b: &&str| { b.cmp(a) };
		list_ref.compare = Some(&list_ref_ordering);
		let mut list_dupe = StringListDupe::new();
		let list_dupe_ordering:fn(&String, &String) -> std::cmp::Ordering = |a: &String, b: &String| { b.cmp(a) };
		list_dupe.compare = Some(&list_dupe_ordering);
		

		list_ref.append(&third);
		list_ref.append(&first);
		list_ref.append(&second);

		let mut refUnsortedIter = list_ref.iter();
		assert_eq!(&third, refUnsortedIter.next().unwrap());
		assert_eq!(&first, refUnsortedIter.next().unwrap());
		assert_eq!(&second, refUnsortedIter.next().unwrap());

		list_dupe.append(&third);
		list_dupe.append(&first);
		list_dupe.append(&second);
		let mut dupeUnsortedIter = list_dupe.iter();
		assert_eq!(&third, dupeUnsortedIter.next().unwrap());
		assert_eq!(&first, dupeUnsortedIter.next().unwrap());
		assert_eq!(&second, dupeUnsortedIter.next().unwrap());

		list_ref.sort();
		let mut refSortedIter = list_ref.iter();
		assert_eq!(&third, refSortedIter.next().unwrap());
		assert_eq!(&second, refSortedIter.next().unwrap());
		assert_eq!(&first, refSortedIter.next().unwrap());

		list_dupe.sort();
		let mut dupeSortedIter = list_dupe.iter();
		assert_eq!(&third, dupeSortedIter.next().unwrap());
		assert_eq!(&second, dupeSortedIter.next().unwrap());
		assert_eq!(&first, dupeSortedIter.next().unwrap());
	}
}