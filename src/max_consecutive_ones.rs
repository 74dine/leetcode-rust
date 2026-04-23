#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![1, 1, 0, 1, 1, 1], 3),
        (vec![1, 0, 1, 1, 0, 1], 2),
        (vec![0], 0),
        (vec![1], 1),
        (vec![], 0),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: Vec<i32>) -> i32 {
    find_max_consecutive_ones(arr)
}

#[allow(dead_code)]
pub fn organize_result() {}

fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_len = 0;
    let mut cur_len = 0;

    for n in nums {
        if n != 1 {
            max_len = max_len.max(cur_len);
            cur_len = 0;

            continue;
        }

        cur_len += 1;
    }

    max_len.max(cur_len)
}
