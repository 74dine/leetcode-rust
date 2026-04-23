#[allow(dead_code)]
pub fn get_test_cases() -> Vec<((Vec<i32>, i32), Vec<i32>)> {
    vec![
        ((vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]),
        (
            (vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1],
        ),
        ((vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]),
        // ((vec![], 0), vec![]),
    ]
}

#[allow(dead_code)]
pub fn solve(nums: Vec<i32>, n: i32) -> Vec<i32> {
    shuffle(nums, n)
}

#[allow(dead_code)]
pub fn organize_result() {}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut arr = Vec::with_capacity(nums.len());

    for i in 0..n {
        arr.push(nums[i as usize]);
        arr.push(nums[(i + n) as usize]);
    }

    arr
}
