// https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    type ArrayPartition = (Vec<u8>, Vec<u8>);
    let (non_zeros, zeros): ArrayPartition = arr.iter().partition(|&x| *x != 0);
    non_zeros.into_iter().chain(zeros.into_iter()).collect()
}

fn test(arg: &[u8], expected: &[u8]) {
    let result = move_zeros(arg);
    println!("{:?} -> {:?}", result, expected);
    assert!(result == expected);
}

pub fn prove() {
    let arr1 = &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1];
    let arr2 = &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9];
    let arr3 = &[0, 0];
    let arr4 = &[0];
    let arr5 = &[];

    test(arr1, &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
    test(arr2, &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
    test(arr3, &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
    test(arr4, &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
    test(arr5, &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
}
