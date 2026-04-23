#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![5, 2, 3, 1], vec![1, 2, 3, 5]),
        (vec![3, 2, 1], vec![1, 2, 3]),
        (vec![4, 6, 2, 4, 6], vec![2, 4, 4, 6, 6]),
        (vec![-1, -5, -7], vec![-7, -5, -1]),
        (vec![-1, 0, -6, 10], vec![-6, -1, 0, 10]),
        (vec![-1], vec![-1]),
        (vec![1], vec![1]),
        (vec![], vec![]),
        (vec![1, 2, 3], vec![1, 2, 3]),
    ]
}

#[allow(dead_code)]
pub fn solve(arr: Vec<i32>) -> Vec<i32> {
    sort_array(arr)
}

#[allow(dead_code)]
pub fn organize_result(arr: Vec<i32>) -> Option<Vec<i32>> {
    let mut owned_arr = arr.clone();
    owned_arr.sort();
    Some(arr)
}

#[allow(dead_code)]
pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let mut arr = nums.to_owned();

    let mut min = nums[0];
    let mut max = nums[0];
    for num in nums.iter() {
        if *num < min {
            min = *num;
        }

        if *num > max {
            max = *num;
        }
    }

    let mut pos: Vec<u32> = vec![0; i32::abs(max) as usize + 1];
    let mut neg: Vec<i32> = vec![0; i32::abs(min) as usize + 1];
    for num in nums.iter() {
        if *num < 0 {
            neg[i32::abs(*num) as usize] += 1;
        } else {
            pos[*num as usize] += 1;
        }
    }

    let mut idx = 0;
    for num in min..0 {
        for _ in 0..neg[-num as usize] {
            arr[idx] = num;
            idx += 1;
        }
    }

    for num in 0..=max {
        for _ in 0..pos[num as usize] {
            arr[idx] = num;
            idx += 1;
        }
    }

    arr
}
