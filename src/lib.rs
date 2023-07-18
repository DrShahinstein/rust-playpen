pub mod snippets {
    pub mod generic_methods;
    pub mod guess_number;
    pub mod lifetimes;
    pub mod median_and_mode;
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
    pub mod increment_string;
    pub mod longest_repetition_char;
    pub mod move_zeros_to_end;
    pub mod two_sum;

    #[macro_export]
    macro_rules! test {
    ($solution:expr, $expected:expr, $($args:expr),*) => {
        {
            let result = $solution($($args),*);
            println!("{:?} => {:?}", result, $expected);
            assert_eq!(result, $expected);
        }
    };
}
}
