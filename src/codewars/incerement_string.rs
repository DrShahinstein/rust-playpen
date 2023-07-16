// https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust

pub fn increment_string(s: &str) -> String {
    let rev_num: String = s.chars().rev().take_while(|c| c.is_numeric()).collect();
    if rev_num.is_empty() {
        return format!("{}1", s);
    }
    let num: String = rev_num.chars().rev().collect();
    let increased_num = increment_with_leading_zeros(&num);
    let suffix_string = s.trim_end_matches(|c: char| c.is_numeric());
    format!("{}{}", suffix_string, increased_num)
}

fn increment_with_leading_zeros(s: &str) -> String {
    let digits: Vec<char> = s.chars().collect();
    let (new_digits, carry) =
        digits
            .iter()
            .rev()
            .fold((Vec::new(), true), |(mut new_digits, carry), digit| {
                if carry {
                    if *digit == '9' {
                        new_digits.push('0');
                    } else {
                        new_digits.push((*digit as u8 + 1) as char);
                        return (new_digits, false);
                    }
                } else {
                    new_digits.push(*digit);
                }
                (new_digits, carry)
            });
    let incremented_digits = if carry {
        let mut incremented_digits = vec!['1'];
        incremented_digits.extend(new_digits.into_iter().rev());
        incremented_digits
    } else {
        new_digits.into_iter().rev().collect()
    };
    incremented_digits.into_iter().collect()
}

fn test(arg: &str, expected: &str) {
    let result = increment_string(arg);
    println!("{result} = {expected}");
    assert!(result == expected);
}

pub fn prove() {
    test("foo", "foo1");
    test("foobar001", "foobar002");
    test("foobar1", "foobar2");
    test("foobar00", "foobar01");
    test("foobar99", "foobar100");
    test("foobar099", "foobar100");
    test("", "1");
}

/*
  In a real world program, I would've add +1 to the last number part in the given string within this way:
    let increased_num = num.parse::<i32>().unwrap() + 1;
    let result = format!("{:0>width$}", increased_num, width = num.to_string().len());
  But, in codewars, optimization matters since katas literally give whopping large numbers.
*/

/*
.trim_end_matches examples:
  assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
  assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");
*/
