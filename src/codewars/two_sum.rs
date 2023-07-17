// https://www.codewars.com/kata/52c31f8e6605bcc646000082/train/rust

use crate::test;

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let len = numbers.len();
    let mut indices = (0..len).flat_map(|first_index| {
        (first_index + 1..len).map(move |second_index| (first_index, second_index))
    });

    // Find the first pair of indices where the sum of the corresponding numbers equals the target
    indices
        .find(|(i, j)| numbers[*i] + numbers[*j] == target)
        .unwrap()
}

pub fn prove() {
    let numbers = &[2, 7, 11, 15];
    let target = 9;
    test!(two_sum, (0, 1), numbers, target);
}

/*
core::iter::traits::iterator::Iterator
pub fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
where
    Self: Sized,
    U: IntoIterator,
    F: FnMut(Self::Item) -> U,

Creates an iterator that works like map, but flattens nested structure.
The [map] adapter is very useful, !!but only when the closure argument produces values.
If it produces an iterator instead, there's an extra layer of indirection!!
flat_map() will remove this extra layer on its own.
You can think of flat_map(f) as the semantic equivalent of [map]ping, and then [flatten]ing as in map(f).flatten().
Another way of thinking about flat_map(): [map]'s closure returns one item for each element,
and flat_map()'s closure returns an iterator for each element.
 */
