use std::collections::HashSet;

#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![100, 4, 200, 1, 3, 2], 4),
        (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        (vec![1, 0, 1, 2], 3),
        (vec![1, 1, 1], 1),
        (vec![-1000000000, 1000000000], 1),
        (vec![], 0),
    ]
}

#[allow(dead_code)]
pub fn solve(nums: Vec<i32>) -> i32 {
    longest_consecutive(nums)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut distinct: HashSet<i32> = HashSet::from_iter(nums.clone().into_iter());

    let mut max_len = 0;

    for num in nums {
        if distinct.contains(&(num - 1)) {
            continue;
        }

        let mut cur_len = 1;
        while distinct.remove(&(num + cur_len)) {
            cur_len += 1;
        }

        if cur_len > max_len {
            max_len = cur_len
        }
    }

    max_len
}
