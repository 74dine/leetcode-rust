use std::collections::HashMap;

#[allow(dead_code)]
fn main() {
    let test_cases = [
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
        (vec![-3, -3], -6, vec![0, 1]),
        (vec![3, -3], 0, vec![0, 1]),
        (vec![0, 0], 0, vec![0, 1]),
    ];

    for (params, target, expected_result) in &test_cases {
        let result = two_sum(params.clone(), *target);

        if result == *expected_result {
            println!("[passed] Case {:?} => {:?}", params, expected_result);
        } else {
            println!(
                "[failed] Case {:?} => {:?} | Returned: {:?}",
                params, expected_result, result
            );
        }
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let prev_idx = seen.get(&(target - nums[i])).unwrap_or(&-1);

        if prev_idx != &-1 {
            return vec![*prev_idx, i as i32];
        }

        seen.insert(nums[i], i as i32);
    }

    eprintln!("Failed to match!");
    vec![]
}
