#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(String, bool)> {
    vec![
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from("race a car"), false),
        (String::from(" "), true),
        (String::from("1"), true),
        (String::from("11"), true),
        (String::from("112"), false),
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
    ]
}

#[allow(dead_code)]
pub fn solve(s: String) -> bool {
    is_palindrome(s)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn is_palindrome(s: String) -> bool {
    /*
    phrase
    + consists of alphanumeric characters
    + case does not matter
    + palindrome phrase = palindrome phrase reversed
    - spaces, symbols
    */

    let bytes = s.as_bytes();

    let mut i = 0;
    let mut j = bytes.len() - 1;

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
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}
