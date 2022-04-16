/**
    string_list is a C-like structure to hold a list of strings
    We may or may dupe strings and thus own their memory, depending on a flag set at construction, and they may or may not be sorted
    This flag appears to be set at initialization time - so we'll use generics/specializaiton and swap out the implementation.

    I'm going to model this by using String vs. &str, which are owned versus unowned strings, appropriately.
    I'm trusting that Rust will steer us right if we try and use StringListRef and it becomes unsafe.
**/

/**
    Base class for string_lists.  We specialize for Strings, which are Duped, and &strs, which are referencing existing strings.

**/
pub struct StringListBase<'a, S>
where
    S: AsRef<str> + From<&'a str> + Ord,
{
    list: Vec<S>,
    /**
     This is (&str, &str) to be unified whether we're Ref or Dupe
     We can drop Strings to &strs, and beyond exotic sorting scenarios, we can expect similar behavior.
    **/
    compare: Option<&'a dyn Fn(&str, &str) -> std::cmp::Ordering>,
}

/**
    Replaces string_lists initialized with string_list_init_nodup
**/
pub type StringListRef<'a> = StringListBase<'a, &'a str>;
/**
    Replaces string_lists initialized with string_list_init_dup
**/
pub type StringListDupe<'a> = StringListBase<'a, String>;

// The function we're calling will not own the strings.
// The git implementation has a void* cb_data.  Not filling this in until I know what it's for.
pub type StringListEachFunc = fn(&str) -> i32;
pub type StringListKeepFunc = fn(&str) -> bool;

impl<
        'a,
        S: std::convert::AsRef<str>
            + std::convert::From<&'a str>
            + std::default::Default
            + std::cmp::Ord,
    > StringListBase<'a, S>
{
    pub fn new() -> StringListBase<'a, S> {
        Self {
            list: Vec::new(),
            compare: None,
        }
    }

    /**
        Port of string_list_clear
    **/
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /**
        Port of for_each_string_list
    **/
    pub fn for_each(&self, func: &StringListEachFunc) -> i32 {
        for item in self.list.iter() {
            let ret = func(item.as_ref());
            if ret != 0 {
                return ret;
            }
        }
        return 0;
    }

    /**
        Port of filter_string_list
    **/
    pub fn keep_filtered(&mut self, func: &StringListKeepFunc) {
        let mut count_dest = 0;
        for i in 0..self.list.len() {
            if func(&self.list[i].as_ref()) {
                if count_dest != i {
                    self.list[count_dest] = std::mem::take(&mut self.list[i]);
                }
                count_dest += 1;
            }
        }
        self.list.truncate(count_dest);
    }

    /**
        Port of string_list_sort
    **/
    pub fn sort(&mut self) {
        self.list.sort_by(|a: &S, b: &S| {
            self.compare
                .unwrap_or(&(str::cmp as fn(&str, &str) -> std::cmp::Ordering))(
                a.as_ref(),
                b.as_ref(),
            )
        });
    }

    /**
        Port of unsorted_string_list_lookup
    **/
    pub fn unsorted_lookup(&self, lookup: &str) -> Option<&S> {
        for item in self.iter() {
            if self
                .compare
                .unwrap_or(&(str::cmp as fn(&str, &str) -> std::cmp::Ordering))(
                &item.as_ref(),
                &lookup,
            ) == std::cmp::Ordering::Equal
            {
                return Some(&item);
            }
        }
        return None;
    }

    /**
        New Addition
    **/
    pub fn iter(&self) -> std::slice::Iter<'_, S> {
        self.list.iter()
    }
}

/**
    Specialized impls for those methods which only work in no-dupe land
**/
impl<'a> StringListRef<'a> {
    /**
        Port of string_list_append
    **/
    pub fn append(&mut self, str_in: &'a str) {
        self.list.push(str_in)
    }
}

/**
    Specialized imples for those methods which only work in dupe land
**/
impl<'a> StringListDupe<'a> {
    /**
        Port of string_list_append

        This is more permissive than StringListRef::apend, since we can accept a different lifetime than our own (generally smaller), since we duplicate it
    **/
    pub fn append<'b>(&mut self, str_in: &'b str) {
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

        {
            // OK - we copy the string, so me can Drop and
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

        let mut ref_unsorted_iter = list_ref.iter();
        assert_eq!(&third, ref_unsorted_iter.next().unwrap());
        assert_eq!(&first, ref_unsorted_iter.next().unwrap());
        assert_eq!(&second, ref_unsorted_iter.next().unwrap());

        list_dupe.append(&third);
        list_dupe.append(&first);
        list_dupe.append(&second);
        let mut dupe_unsorted_iter = list_dupe.iter();
        assert_eq!(&third, dupe_unsorted_iter.next().unwrap());
        assert_eq!(&first, dupe_unsorted_iter.next().unwrap());
        assert_eq!(&second, dupe_unsorted_iter.next().unwrap());

        list_ref.sort();
        let mut ref_sorted_iter = list_ref.iter();
        assert_eq!(&first, ref_sorted_iter.next().unwrap());
        assert_eq!(&second, ref_sorted_iter.next().unwrap());
        assert_eq!(&third, ref_sorted_iter.next().unwrap());

        list_dupe.sort();
        let mut dupe_sorted_iter = list_dupe.iter();
        assert_eq!(&first, dupe_sorted_iter.next().unwrap());
        assert_eq!(&second, dupe_sorted_iter.next().unwrap());
        assert_eq!(&third, dupe_sorted_iter.next().unwrap());
    }

    #[test]
    fn sort_user_compare() {
        let first = "item1";
        let second = "item2";
        let third = "item3";

        let reverse_ordering: fn(&str, &str) -> std::cmp::Ordering = |a: &str, b: &str| b.cmp(a);
        let mut list_ref = StringListRef::new();
        list_ref.compare = Some(&reverse_ordering);
        let mut list_dupe = StringListDupe::new();
        list_dupe.compare = Some(&reverse_ordering);

        list_ref.append(&third);
        list_ref.append(&first);
        list_ref.append(&second);

        let mut ref_unsorted_iter = list_ref.iter();
        assert_eq!(&third, ref_unsorted_iter.next().unwrap());
        assert_eq!(&first, ref_unsorted_iter.next().unwrap());
        assert_eq!(&second, ref_unsorted_iter.next().unwrap());

        list_dupe.append(&third);
        list_dupe.append(&first);
        list_dupe.append(&second);
        let mut dupe_unsorted_iter = list_dupe.iter();
        assert_eq!(&third, dupe_unsorted_iter.next().unwrap());
        assert_eq!(&first, dupe_unsorted_iter.next().unwrap());
        assert_eq!(&second, dupe_unsorted_iter.next().unwrap());

        list_ref.sort();
        let mut ref_sorted_iter = list_ref.iter();
        assert_eq!(&third, ref_sorted_iter.next().unwrap());
        assert_eq!(&second, ref_sorted_iter.next().unwrap());
        assert_eq!(&first, ref_sorted_iter.next().unwrap());

        list_dupe.sort();
        let mut dupe_sorted_iter = list_dupe.iter();
        assert_eq!(&third, dupe_sorted_iter.next().unwrap());
        assert_eq!(&second, dupe_sorted_iter.next().unwrap());
        assert_eq!(&first, dupe_sorted_iter.next().unwrap());
    }
}
