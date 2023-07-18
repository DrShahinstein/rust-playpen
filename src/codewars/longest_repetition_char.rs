// https://www.codewars.com/kata/586d6cefbcc21eed7a001155/train/rust

use crate::test;
use itertools::Itertools;

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.is_empty() {
        return None;
    }

    let (max_char, max_count) = s
        .chars()
        .group_by(|&c| c) // Group consecutive characters
        .into_iter() // Convert result into a regular iterator
        .map(|(chr, group)| (chr, group.count())) // Transform each group into a tuple of character and count
        .max_by_key(|&(_, count)| count) // Find the pair with the maximum count
        .unwrap();

    Some((max_char, max_count))
}

pub fn prove() {
    test!(longest_repetition, Some(('a', 4)), "aaaabbb");
    test!(longest_repetition, Some(('b', 5)), "abbbbb");
    test!(longest_repetition, Some(('a', 4)), "bbbaaabaaaa");
    test!(longest_repetition, Some(('u', 3)), "cbdeuuu900");
}

/*
itertools::Itertools
pub fn group_by<K, F>(self, key: F) -> GroupBy<K, Self, F>
where
    Self: Sized,
    F: FnMut(&Self::Item) -> K,
    K: PartialEq,

Returns an iterable that can group iterator elements.
Consecutive elements that map to the same key (“runs”), are assigned to the same group.
GroupBy is the storage for the lazy grouping operation.

If the groups are consumed in order, or if each group's iterator is dropped without keeping it around,
then GroupBy uses no allocations. It needs allocations only if several group iterators are alive at the same time.

This type implements IntoIterator (it is not an iterator itself),
because the group iterators need to borrow from this value.
It should be stored in a local variable or temporary and iterated.
Iterator element type is (K, Group): the group's key and the group iterator.
*/
