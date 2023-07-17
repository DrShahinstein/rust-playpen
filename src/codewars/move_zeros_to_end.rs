// https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust

use crate::test;

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    type ArrayPartition = (Vec<u8>, Vec<u8>);
    let (non_zeros, zeros): ArrayPartition = arr.iter().partition(|&x| *x != 0);
    non_zeros.into_iter().chain(zeros.into_iter()).collect()
}

pub fn prove() {
    test!(
        move_zeros,
        &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
        &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]
    );
    test!(
        move_zeros,
        &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9]
    );
    test!(move_zeros, &[0, 0], &[0, 0]);
    test!(move_zeros, &[0], &[0]);
}
