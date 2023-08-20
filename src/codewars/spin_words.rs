// https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust

use crate::test;

pub fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<String>()
            } else {
                String::from(word)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn prove() {
    test!(spin_words, "emocleW", "Welcome");
    test!(spin_words, "Hey wollef sroirraw", "Hey fellow warriors");
    test!(spin_words, "This is a test", "This is a test");
    test!(spin_words, "This is rehtona test", "This is another test");
    test!(
        spin_words,
        "You are tsomla to the last test",
        "You are almost to the last test"
    );
    test!(
        spin_words,
        "Just gniddik ereht is llits one more",
        "Just kidding there is still one more"
    );
    test!(
        spin_words,
        "ylsuoireS this is the last one",
        "Seriously this is the last one"
    );
}
