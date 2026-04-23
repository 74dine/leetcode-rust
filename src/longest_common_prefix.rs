#[allow(dead_code)]
fn main() {
    let test_cases = get_test_cases();

    for (input, expected) in &test_cases {
        let result = longest_common_prefix(input.clone());

        if result == *expected {
            println!("[passed] Case {:?} => {:?}", input, expected);
        } else {
            println!(
                "[failed] Case {:?} => {:?} | Returned: {:?}",
                input, expected, result
            );
        }
    }
}

pub fn get_test_cases() -> [(Vec<String>, String); 6] {
    [
        (
            vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ],
            "fl".to_string(),
        ),
        (
            vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
            "".to_string(),
        ),
        (
            vec!["she".to_string(), "shot".to_string(), "shake".to_string()],
            "sh".to_string(),
        ),
        (vec!["".to_string()], "".to_string()),
        (vec!["ab".to_string(), "a".to_string()], "a".to_string()),
        (vec!["cir".to_string(), "car".to_string()], "c".to_string()),
    ]
}

pub fn longest_common_prefix(str_arr: Vec<String>) -> String {
    let mut prefix_len = str_arr[0].len();

    for str in str_arr.iter().skip(1) {
        if str.len() < prefix_len {
            prefix_len = str.len();
        }

        for char_idx in 0..prefix_len {
            if str.chars().nth(char_idx) == str_arr[0].chars().nth(char_idx) {
                continue;
            }

            prefix_len = char_idx;
            break;
        }

        if prefix_len <= 0 {
            return "".to_string();
        }
    }

    str_arr[prefix_len][..prefix_len].to_owned()
}

/*
// 3ms runtime, 2.36 MB memory usage

// Beats: 05.07%  (time complexity)
// Beats: 11.94% (space complexity)

fn longest_common_prefix(input: Vec<String>) -> String {
    let mut prefix_len = input[0].len();

    for s in input.iter().skip(1) {
        let mut len = 0;
        loop {
            let cur_char_in_s = s.chars().nth(len).unwrap_or('\0');
            let cur_char_in_prefix = input[0].chars().nth(len).unwrap_or('\0');

            if cur_char_in_s == '\0'
                || cur_char_in_prefix == '\0'
                || cur_char_in_s != cur_char_in_prefix
            {
                break;
            }
            len += 1;
        }

        prefix_len = min(prefix_len, len);
    }

    input[0][..prefix_len].to_owned()
}
 */
