// https://www.codewars.com/kata/526dbd6c8c0eb53254000110/train/rust

use crate::test;

pub fn alphanumeric(password: &str) -> bool {
    if password.is_empty() {
        return false; // At least one character is required
    }

    for c in password.chars() {
        if !c.is_ascii_alphanumeric() {
            return false; // Only alphanumeric characters are allowed
        }
    }

    true
}

pub fn prove() {
    test!(alphanumeric, false, "hello world_");
    test!(alphanumeric, true, "PassW0rd");
    test!(alphanumeric, false, "     ");
}
