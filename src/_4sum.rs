#[allow(dead_code)]
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();

    let mut result = vec![];

    // println!("{:?}", nums);
    for (i, n) in nums.iter().enumerate() {
        if i > 0 && *n == nums[i - 1] {
            continue;
        }

        // for j in i + 1..nums.len() {
        for (j, n_2) in nums.iter().skip(i + 1).enumerate() {
            if j > 0 && n_2 == &nums[i + j] {
                continue;
            }

            let mut left = i + j + 2;
            let mut right = nums.len() - 1;

            while left < right {
                // let sum = n + n_2 + nums[left] + nums[right];
                let sum = match n.checked_add(*n_2) {
                    Some(sum) => match sum.checked_add(nums[left]) {
                        Some(sum) => sum.checked_add(nums[right]).unwrap_or_else(|| sum),
                        None => sum,
                    },
                    None => *n,
                };

                if sum > target {
                    right -= 1;
                    continue;
                }
                if sum < target {
                    left += 1;
                    continue;
                }

                // println!("i: {} j: {} p: {} k: {}", i, i + j, left, right);
                result.push(vec![*n, *n_2, nums[left], nums[right]]);
                left += 1;

                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod _4sum_tests {
    use crate::_4sum::four_sum;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(vec![vec![2, 2, 2, 2]], four_sum(vec![2, 2, 2, 2, 2], 8))
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]],
            four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0)
        )
    }

    #[test]
    fn lc_case_4() {
        assert_eq!(
            vec![vec![-1000000000, -1000000000, 1000000000, 1000000000]],
            four_sum(
                vec![
                    1000000000,
                    1000000000,
                    1000000000,
                    1000000000,
                    -1000000000,
                    -1000000000,
                    -1000000000,
                    -1000000000
                ],
                0
            )
        );
    }

    #[test]
    fn does_handle_long_sum() {
        assert_eq!(
            vec![] as Vec<Vec<i32>>,
            four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            )
        );
    }

    #[test]
    fn does_handle_long_sum_2() {
        assert_eq!(
            vec![vec![2, 2, 2, 1000000000]],
            four_sum(
                vec![1000000000, 2, 2, 1000000000, 1000000000, 2],
                1000000006
            )
        );
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn does_handle_neg_long_sum() {
        assert_eq!(
            vec![] as Vec<Vec<i32>>,
            four_sum(
                vec![-1000000000, -1000000000, -1000000000, -1000000000],
                294967296
            )
        );
    }

    #[test]
    fn does_handle_neg_long_sum_2() {
        assert_eq!(
            vec![vec![-2, -2, -2, 0]],
            four_sum(
                vec![-1000000000, 0, -2, -2, 1000000000, -1000000000, -2],
                -6
            )
        );
    }
}
