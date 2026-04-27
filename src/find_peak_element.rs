#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![1], 0),
        (vec![1, 2, 3, 1], 2),
        (vec![-1, -2, -3, -1], 0),
        (vec![1, 2, 1, 3, 5, 6, 4], 5),
        (vec![-1, -2, -1, -3, -5, -6, -4], 2),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: Vec<i32>) -> i32 {
    find_peak_element(arr)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (left + right) / 2;

        if nums[mid] > nums[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}
