#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![2, 0, 2, 1, 1, 0], vec![0, 0, 1, 1, 2, 2]),
        (vec![2, 0, 1], vec![0, 1, 2]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1, 2]),
        (vec![2, 1], vec![1, 2]),
    ]
}

#[allow(dead_code)]
#[inline]
pub fn solve(arr: &mut Vec<i32>) {
    sort_colors(arr)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut offset = 0;
    let mut cursor = 0;
    let mut tail = nums.len();

    while cursor < tail {
        match nums[cursor] {
            0 => {
                nums.swap(cursor, offset);
                offset += 1;
                cursor += 1;
            }
            2 => {
                tail -= 1;
                nums.swap(cursor, tail);
            }
            _ => {
                cursor += 1;
            }
        }
    }
}
