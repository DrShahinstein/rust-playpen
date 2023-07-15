// https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    type ArrayPartition = (Vec<u8>, Vec<u8>);
    let (non_zeros, zeros): ArrayPartition = arr.iter().partition(|&x| *x != 0);
    non_zeros.into_iter().chain(zeros.into_iter()).collect()
}

pub fn prove() {
    let arr1 = &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1];
    let arr2 = &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9];
    let arr3 = &[0, 0];
    let arr4 = &[0];
    let arr5 = &[];

    let result1 = move_zeros(arr1);
    let result2 = move_zeros(arr2);
    let result3 = move_zeros(arr3);
    let result4 = move_zeros(arr4);
    let result5 = move_zeros(arr5);

    println!("{:?}", result1); // Output: [1, 2, 1, 1, 3, 1, 0, 0, 0, 0]
    println!("{:?}", result2); // Output: [9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("{:?}", result3); // Output: [0, 0]
    println!("{:?}", result4); // Output: [0]
    println!("{:?}", result5); // Output: []
}
