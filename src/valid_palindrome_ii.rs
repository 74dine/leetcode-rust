#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(String, bool)> {
    vec![
        (String::from("aba"), true),
        (String::from("abca"), true),
        (String::from("abc"), false),
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from("race a car"), true),
        (String::from(" "), true),
        (String::from("1"), true),
        (String::from("11"), true),
        (String::from("112"), true),
        (String::from("121"), true),
        (String::from("1$2-2+1"), true),
        (
            String::from("Marge, let's \"[went].\" I await {news} telegram."),
            true,
        ),
        (
            String::from("Marge, let's \\\"[went].\\\" I await {news} telegram."),
            true,
        ),
        (String::from("abaca"), false),
        (String::from("dee"), true),
        (String::from("deeee"), true),
    ]
}

#[allow(dead_code)]
pub fn solve(s: String) -> bool {
    valid_palindrome(s)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn valid_palindrome(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut skip = 1;

    let mut i = 0;
    let mut j = bytes.len() - 1;

    let mut is_palindrome = true;

    while i < j {
        if !bytes[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }

        if !bytes[j].is_ascii_alphanumeric() {
            j -= 1;
            continue;
        }

        if bytes[i].to_ascii_lowercase() != bytes[j].to_ascii_lowercase() {
            if skip > 0 {
                skip -= 1;

                j -= 1;
                continue;
            }

            is_palindrome = false;
            break;
        }

        i += 1;
        j -= 1;
    }

    if !is_palindrome {
        is_palindrome = true;

        i = 0;
        j = bytes.len() - 1;
        skip = 1;

        while i < j {
            if !bytes[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }

            if !bytes[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }

            if bytes[i].to_ascii_lowercase() != bytes[j].to_ascii_lowercase() {
                if skip > 0 {
                    skip -= 1;

                    i += 1;
                    continue;
                }

                is_palindrome = false;
                break;
            }

            i += 1;
            j -= 1;
        }
    }

    is_palindrome
}
