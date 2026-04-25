#[allow(dead_code)]
pub fn get_test_cases() -> Vec<((Vec<i32>, i32, Vec<i32>, i32), Vec<i32>)> {
    vec![
        (
            (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
            vec![1, 2, 2, 3, 5, 6],
        ),
        ((vec![1], 1, vec![0], 0), vec![1]),
        ((vec![0], 0, vec![1], 1), vec![1]),
    ]
}

#[allow(dead_code)]
pub fn solve(left: &mut Vec<i32>, left_size: i32, right: &mut Vec<i32>, right_size: i32) {
    merge(left, left_size, right, right_size)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn merge(left: &mut Vec<i32>, take_left: i32, right: &mut Vec<i32>, take_right: i32) {
    for (j, i) in (take_left..take_left + take_right).enumerate() {
        left[i as usize] = right[j];
    }

    left.sort()
}
