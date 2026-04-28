#[allow(dead_code)]
pub fn get_test_cases() {}

#[allow(dead_code)]
pub fn solve() {}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();

    nums.len() as i32
}
