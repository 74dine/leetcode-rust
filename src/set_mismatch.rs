#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![1, 2, 2, 4], vec![2, 3]),
        (vec![1, 1], vec![1, 2]),
        (vec![2, 2], vec![2, 1]),
        (vec![3, 3, 2, 3], vec![2, 1]),
        (vec![3, 2, 2], vec![2, 1]),
    ]
}

#[allow(dead_code)]
pub fn solve(nums: Vec<i32>) -> Vec<i32> {
    find_error_nums(nums)
}

#[allow(dead_code)]
pub fn organize_result() {}

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = nums[0];
    let mut idx = 1;
    for i in 1..nums.len() {
        sum += nums[i];
        idx += 1;

        let expected_sum = (idx * (idx + 1)) / 2;
        if sum != expected_sum {
            println!("expected: {} at: [{i}]", expected_sum - (sum - nums[i]));
            return vec![nums[i], expected_sum - (sum - nums[i])];
        }
    }

    vec![]
}
