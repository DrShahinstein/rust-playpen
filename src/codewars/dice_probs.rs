// https://www.codewars.com/kata/56f78a42f749ba513b00037f/train/rust

use crate::test;

fn combinations(dice_amount: i32, target_sum: i32) -> usize {
    if dice_amount == 1 {
        return if 1 <= target_sum && target_sum <= 6 { 1 } else { 0 };
    }
    
    let mut count = 0;
    
    for i in 1..=6 {
        count += combinations(dice_amount - 1, target_sum - i);
    }
    
    count
}

pub fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    let total_combinations = 6u64.pow(dice_amount as u32);
    let matching_combinations = combinations(dice_amount, sum);
    let probability = matching_combinations as f64 / total_combinations as f64;
    
    probability
}

pub fn prove() {
    test!(rolldice_sum_prob, 1.0/18.0, 11, 2);
    test!(rolldice_sum_prob, 5.0/36.0, 8, 2);
    test!(rolldice_sum_prob, 7.0/72.0, 8, 3);
    test!(rolldice_sum_prob, 0.0, 22, 3);
}
