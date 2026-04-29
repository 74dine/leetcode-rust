#[macro_export]
macro_rules! run_cases {
    ($cases:expr, $runner:expr) => {
        let mut all_passed = true;

        for (input, expected) in $cases {
            let result = $runner(input.to_owned());

            if result == expected {
                println!("[passed] Case {:?} => {:?}", input, expected);
            } else {
                println!(
                    "[failed] Case {:?} => {:?} | Returned: {:?}",
                    input, expected, result
                );
                all_passed = false;
            }
        }

        assert!(all_passed, "Some test cases failed");
    };
}
