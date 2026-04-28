#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![1, 1, 2], 2),
        (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5),
        (vec![-4, -3, -3, -2, -2, -1, -1, -1, 0, 0], 5),
        (vec![-1, 1, 2], 3),
        (vec![-1], 1),
        (vec![-1, -1], 1),
        (vec![-1, 0, 0, 0, 3], 3),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: &mut Vec<i32>) -> i32 {
    remove_duplicates(arr)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }

    let mut tail = 0;

    for head in 0..nums.len() {
        if nums[head] > nums[tail] {
            tail += 1;
            nums.swap(tail, head);
        }
    }

    // convert 0 based indexing to 1 based index for len
    (tail + 1) as i32
}
