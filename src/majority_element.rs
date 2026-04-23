#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)]
}

#[allow(dead_code)]
pub fn solve(num_arr: Vec<i32>) -> i32 {
    majority_element(num_arr)
}

#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut vote = 0;

    for num in nums {
        if vote == 0 {
            candidate = num;
        }

        if num == candidate {
            vote += 1;
        } else {
            vote -= 1;
        }
    }

    candidate
}

#[allow(dead_code)]
pub fn organize_result(value: i32) -> Option<i32> {
    Some(value)
}
