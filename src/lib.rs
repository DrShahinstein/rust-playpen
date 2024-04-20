pub mod snippets {
    pub mod concurrency;
    pub mod generic_methods;
    pub mod guess_number;
    pub mod lifetimes;
    pub mod median_and_mode;
    pub mod smart_pointers;
    pub mod structs;
    pub mod temperature_converter;
    pub mod using_json;
    pub mod employee_automation {
        pub mod automation;
        pub mod employee;
        pub mod json_handlers;
    }
}

pub mod codewars {
    pub mod alphanumeric;
    pub mod dice_probs;
    pub mod increment_string;
    pub mod longest_repetition_char;
    pub mod move_zeros_to_end;
    pub mod spin_words;
    pub mod two_sum;

    #[macro_export]
    macro_rules! test {
    ($solution:expr, $expected:expr, $($args:expr),*) => {
        {
            let result = $solution($($args),*);
            if result == $expected {
                println!("Passed!");
            } else {
                println!("Failed => expected: {:?} but given: {:?}", $expected, result);
            }
        }
    };
}
}
