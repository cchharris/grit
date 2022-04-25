//!
//! string_list is a C-like structure to hold a list of strings
//! We may or may dupe strings and thus own their memory, depending on a flag set at construction, and they may or may not be sorted
//! This flag appears to be set at initialization time - so we'll use generics/specializaiton and swap out the implementation.
//!
//! I'm going to model this by using String vs. &str, which are owned versus unowned strings, appropriately.
//! I'm trusting that Rust will steer us right if we try and use StringListRef and it becomes unsafe.
//!

///
/// Base class for string_lists.  We specialize for Strings, which are Duped, and &strs, which are referencing existing strings.
///
///
pub struct StringListBase<'a, S>
where
    S: AsRef<str> + From<&'a str> + Ord,
{
    list: Vec<S>,
    ///
    /// This is (&str, &str) to be unified whether we're Ref or Dupe
    /// We can drop Strings to &strs, and beyond exotic sorting scenarios, we can expect similar behavior.
    ///
    compare: Option<&'a dyn Fn(&str, &str) -> std::cmp::Ordering>,
}

///
/// Replaces string_lists initialized with string_list_init_nodup
///
pub type StringListRef<'a> = StringListBase<'a, &'a str>;
///
/// Replaces string_lists initialized with string_list_init_dup
///
pub type StringListDupe<'a> = StringListBase<'a, String>;

/// The function we're calling will not own the strings.
/// The git implementation has a void* cb_data.  Not filling this in until I know what it's for.
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

    ///
    /// Port of string_list_clear
    ///
    pub fn clear(&mut self) {
        self.list.clear();
    }

    ///
    /// Port of for_each_string_list
    ///
    pub fn for_each(&self, func: &StringListEachFunc) -> i32 {
        for item in self.list.iter() {
            let ret = func(item.as_ref());
            if ret != 0 {
                return ret;
            }
        }
        return 0;
    }

    ///
    /// Port of filter_string_list
    ///
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

    ///
    /// Port of string_list_sort
    ///
    pub fn sort(&mut self) {
        self.list.sort_by(|a: &S, b: &S| {
            self.compare
                .unwrap_or(&(str::cmp as fn(&str, &str) -> std::cmp::Ordering))(
                a.as_ref(),
                b.as_ref(),
            )
        });
    }

    ///
    /// Port of unsorted_string_list_lookup
    /// C version returns a pointer - we'll see if we need to change our return type
    ///
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

    ///
    /// Port of unsorted_string_list_has_string
    ///
    pub fn unsorted_has_string(&self, lookup: &str) -> bool {
        self.unsorted_lookup(&lookup).is_some()
    }

    ///
    /// Port of unsorted_string_list_delete_item
    ///
    pub fn unsorted_delete(&mut self, index: usize) {
        self.list.swap_remove(index); // Pretty much exactly the by-hand implementation
    }

    ///
    /// New Addition
    ///
    pub fn iter(&self) -> std::slice::Iter<'_, S> {
        self.list.iter()
    }
}

///
/// Specialized impls for those methods which only work in no-dupe land
///
///  I don't know if it's possible to test for compile_fail outside a doctest
///
/// ```compile_fail
/// let mut list_ref = StringListRef::new();
/// {
///   list_ref.append(&String::from("text"))
/// }
/// ```
impl<'a> StringListRef<'a> {
    ///  Port of string_list_append
    ///
    pub fn append(&mut self, str_in: &'a str) {
        self.list.push(str_in)
    }

    ///
    /// Port of string_list_split_in_place
    ///
    /// CAUTION: We don't actually modify [string]
    ///
    pub fn split_in_place(&mut self, string: &'a mut str, delim: char, max_split: i32) -> usize {
        let mut substr = &string[0..];
        let mut count: usize = 0;
        loop {
            count += 1;
            // TODO
            if max_split >= 0 && count > max_split.try_into().unwrap() {
                self.append(substr);
                return count;
            }
            match substr.find(delim) {
                Some(i) => {
                    let copy = &substr[0..i];
                    self.append(copy);
                    //substr[i] = char::default();
                    match substr.get(i + 1..) {
                        Some(s) => substr = s,
                        None => return count,
                    }
                }
                None => {
                    self.append(substr);
                    return count;
                }
            }
        }
    }
}

///
/// Specialized imples for those methods which only work in dupe land
///
impl<'a> StringListDupe<'a> {
    ///
    /// Port of string_list_append
    ///
    /// This is more permissive than StringListRef::apend, since we can accept a different lifetime than our own (generally smaller), since we duplicate it
    ///
    pub fn append<'b>(&mut self, str_in: &'b str) {
        self.list.push(String::from(str_in))
    }

    ///
    /// Port of string_list_split
    ///
    pub fn split(&mut self, string: &str, delim: char, max_split: i32) -> usize {
        let mut substr = &string[0..];
        let mut count: usize = 0;
        loop {
            count += 1;
            // TODO
            if max_split >= 0 && count > max_split.try_into().unwrap() {
                self.append(substr);
                return count;
            }
            match substr.find(delim) {
                Some(i) => {
                    let copy = &substr[0..i];
                    self.append(copy);
                    match substr.get(i + 1..) {
                        Some(s) => substr = s,
                        None => return count,
                    }
                }
                None => {
                    self.append(substr);
                    return count;
                }
            }
        }
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

    #[test]
    fn keep_filtered_ref() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let filter = "dropme";

        let mut list_ref = StringListRef::new();
        list_ref.append(&keep1);
        list_ref.append(&filter);
        list_ref.append(&keep2);

        let mut ref_prefilter_iter = list_ref.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&filter, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        list_ref.keep_filtered(&((|s: &str| s.ends_with("1")) as fn(&str) -> bool));

        let mut ref_postfilter_iter = list_ref.iter();
        assert_eq!(2, list_ref.list.len());
        assert_eq!(&keep1, ref_postfilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_postfilter_iter.next().unwrap());
    }

    #[test]
    fn keep_filtered_dupe() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let filter = "dropme";

        let mut list_dupe = StringListDupe::new();
        list_dupe.append(&keep1);
        list_dupe.append(&filter);
        list_dupe.append(&keep2);

        let mut ref_prefilter_iter = list_dupe.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&filter, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        list_dupe.keep_filtered(&((|s: &str| s.ends_with("1")) as fn(&str) -> bool));

        let mut ref_postfilter_iter = list_dupe.iter();
        assert_eq!(2, list_dupe.list.len());
        assert_eq!(&keep1, ref_postfilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_postfilter_iter.next().unwrap());
    }

    #[test]
    fn delete_ref() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let delete = "dropme";

        let mut list_ref = StringListRef::new();
        list_ref.append(&keep1);
        list_ref.append(&delete);
        list_ref.append(&keep2);

        let mut ref_prefilter_iter = list_ref.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&delete, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        list_ref.unsorted_delete(1);

        let mut ref_postfilter_iter = list_ref.iter();
        assert_eq!(2, list_ref.list.len());
        assert_eq!(&keep1, ref_postfilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_postfilter_iter.next().unwrap());
    }

    #[test]
    fn delete_dupe() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let delete = "dropme";

        let mut list_dupe = StringListDupe::new();
        list_dupe.append(&keep1);
        list_dupe.append(&delete);
        list_dupe.append(&keep2);

        let mut ref_prefilter_iter = list_dupe.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&delete, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        list_dupe.unsorted_delete(1);

        let mut ref_postfilter_iter = list_dupe.iter();
        assert_eq!(2, list_dupe.list.len());
        assert_eq!(&keep1, ref_postfilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_postfilter_iter.next().unwrap());
    }

    #[test]
    fn unsorted_lookup_ref() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let filter = "dropme";

        let mut list_ref = StringListRef::new();
        list_ref.append(&keep1);
        list_ref.append(&filter);
        list_ref.append(&keep2);

        let mut ref_prefilter_iter = list_ref.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&filter, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        let find = list_ref.unsorted_lookup("dropme");
        assert_eq!(&filter, find.unwrap());
    }

    #[test]
    fn unsorted_lookup_dupe() {
        let keep1 = "keep1";
        let keep2 = "keep_number_2_1";
        let filter = "dropme";

        let mut list_dupe = StringListDupe::new();
        list_dupe.append(&keep1);
        list_dupe.append(&filter);
        list_dupe.append(&keep2);

        let mut ref_prefilter_iter = list_dupe.iter();
        assert_eq!(&keep1, ref_prefilter_iter.next().unwrap());
        assert_eq!(&filter, ref_prefilter_iter.next().unwrap());
        assert_eq!(&keep2, ref_prefilter_iter.next().unwrap());

        let find = list_dupe.unsorted_lookup("dropme");
        assert_eq!(&filter, find.unwrap());
    }

    #[test]
    fn split_in_place_ref() {
        let split_me_base = "I should be split on spaces";
        let mut split_me = String::from(split_me_base);
        let split_1 = "I";
        let split_2 = "should";
        let split_3 = "be";
        let split_4 = "split";
        let split_5 = "on";
        let split_6 = "spaces";

        let mut list_ref = StringListRef::new();
        list_ref.split_in_place(split_me.as_mut_str(), ' ', i32::MAX);

        assert_eq!(list_ref.list.len(), 6);

        let mut iter = list_ref.iter();
        // assert_ne!(split_me.as_str(), split_me_base); // Can't actually compare this - since we modified split_me, we can't borrow it again
        assert_eq!(&split_1, iter.next().unwrap());
        assert_eq!(&split_2, iter.next().unwrap());
        assert_eq!(&split_3, iter.next().unwrap());
        assert_eq!(&split_4, iter.next().unwrap());
        assert_eq!(&split_5, iter.next().unwrap());
        assert_eq!(&split_6, iter.next().unwrap());
    }

    #[test]
    fn split_dupe() {
        let split_me = "I should be split on spaces";
        let split_1 = "I";
        let split_2 = "should";
        let split_3 = "be";
        let split_4 = "split";
        let split_5 = "on";
        let split_6 = "spaces";

        let mut list_dupe = StringListDupe::new();
        list_dupe.split(&split_me, ' ', i32::MAX);

        assert_eq!(list_dupe.list.len(), 6);

        let mut iter = list_dupe.iter();
        assert_eq!(&split_1, iter.next().unwrap());
        assert_eq!(&split_2, iter.next().unwrap());
        assert_eq!(&split_3, iter.next().unwrap());
        assert_eq!(&split_4, iter.next().unwrap());
        assert_eq!(&split_5, iter.next().unwrap());
        assert_eq!(&split_6, iter.next().unwrap());
    }
}
