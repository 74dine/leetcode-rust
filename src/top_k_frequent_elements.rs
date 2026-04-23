use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_test_cases() -> Vec<((Vec<i32>, i32), Vec<i32>)> {
    vec![
        ((vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]),
        ((vec![1], 1), vec![1]),
        ((vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2), vec![1, 2]),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: Vec<i32>, k: i32) -> Vec<i32> {
    top_k_frequent(arr, k)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn top_k_frequent(arr: Vec<i32>, _k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, u32> = HashMap::new();

    for n in arr {
        let prev = freq.entry(n).or_insert(1);
        *prev += 1;
    }

    let result = vec![];
    result
}
