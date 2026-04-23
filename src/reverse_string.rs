#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<char>, Vec<char>)> {
    vec![
        (vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h']),
        (
            vec!['H', 'a', 'n', 'n', 'a', 'h'],
            vec!['h', 'a', 'n', 'n', 'a', 'H'],
        ),
        (vec!['h', 'o'], vec!['o', 'h']),
        (
            vec![
                'A', ' ', 'm', 'a', 'n', ',', ' ', 'a', ' ', 'p', 'l', 'a', 'n', ',', ' ', 'a',
                ' ', 'c', 'a', 'n', 'a', 'l', ':', ' ', 'P', 'a', 'n', 'a', 'm', 'a',
            ],
            vec![
                'a', 'm', 'a', 'n', 'a', 'P', ' ', ':', 'l', 'a', 'n', 'a', 'c', ' ', 'a', ' ',
                ',', 'n', 'a', 'l', 'p', ' ', 'a', ' ', ',', 'n', 'a', 'm', ' ', 'A',
            ],
        ),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: &mut Vec<char>) {
    reverse_string(arr);
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn reverse_string(s: &mut Vec<char>) {
    let size = s.len() - 1;
    for i in 0..=(size / 2) {
        s.swap(i, size - i);
    }
}
