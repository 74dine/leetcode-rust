#[allow(dead_code)]
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut result = Vec::new();

    for (i, n) in nums.iter().enumerate() {
        if i > 0 && *n == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = n + nums[left] + nums[right];

            if sum > 0 {
                right -= 1;
                continue;
            }
            if sum < 0 {
                left += 1;
                continue;
            }

            result.push(vec![*n, nums[left], nums[right]]);
            left += 1;

            while left < right && nums[left] == nums[left - 1] {
                left += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod _3sum_tests {
    use crate::_3sum::three_sum;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>)
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
    }

    #[test]
    fn does_return_empty() {
        assert_eq!(three_sum(vec![1, 2, 3]), vec![] as Vec<Vec<i32>>)
    }

    #[test]
    fn does_handle_negative() {
        assert_eq!(three_sum(vec![-1, -1, 2]), vec![vec![-1, -1, 2]])
    }

    #[test]
    fn does_return_distinct() {
        assert_eq!(three_sum(vec![0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]])
    }
}
