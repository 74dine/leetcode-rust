#[allow(dead_code)]
pub fn get_test_cases() -> Vec<((String, String), String)> {
    vec![
        (
            (String::from("abc"), String::from("pqr")),
            String::from("apbqcr"),
        ),
        (
            (String::from("ab"), String::from("pqrs")),
            String::from("apbqrs"),
        ),
        (
            (String::from("abcd"), String::from("pq")),
            String::from("apbqcd"),
        ),
        ((String::from(""), String::from("")), String::from("")),
        ((String::from("a"), String::from("")), String::from("a")),
        ((String::from(""), String::from("p")), String::from("p")),
    ]
}

#[allow(dead_code)]
pub fn solve(left: String, right: String) -> String {
    merge_alternately(left, right)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let left_bytes = word1.as_bytes();
    let right_bytes = word2.as_bytes();

    let mut result: Vec<char> = Vec::with_capacity(left_bytes.len() + right_bytes.len());

    let mut i = 0;
    let mut j = 0;

    while i < left_bytes.len() || j < right_bytes.len() {
        if left_bytes.len() > 0 && i < left_bytes.len() {
            result.push(left_bytes[i] as char);
            i += 1;
        }

        if right_bytes.len() > 0 && j < right_bytes.len() {
            result.push(right_bytes[j] as char);
            j += 1;
        }
    }

    result.into_iter().collect()
}
