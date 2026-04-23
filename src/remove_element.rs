#[allow(dead_code)]
pub fn get_test_cases() -> Vec<((Vec<i32>, i32), i32)> {
    vec![
        ((vec![3, 2, 2, 3], 3), 2),
        ((vec![0, 1, 2, 2, 3, 0, 4, 2], 3), 7),
        ((vec![], 3), 0),
        ((vec![3], 3), 0),
        ((vec![3, 3], 3), 0),
        ((vec![3, 0, 3], 3), 0),
    ]
}

#[allow(dead_code)]
pub fn solve(nums: &mut Vec<i32>, val: i32) -> i32 {
    remove_element(nums, val)
}

pub fn remove_element(nums: &mut Vec<i32>, target: i32) -> i32 {
    let mut tail = 0;

    for i in 0..nums.len() {
        if nums[i] != target {
            nums.swap(i, tail);
            tail += 1;
        }
    }

    tail as i32
}

#[allow(dead_code)]
pub fn organize_result(result: i32) -> Option<i32> {
    Some(result)
}
