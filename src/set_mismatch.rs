use std::collections::HashSet;

#[allow(dead_code)]
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let expected_sum = (nums.len() * (nums.len() + 1) / 2) as i32;
    let nums_sum = nums.iter().sum::<i32>();

    let distinct: HashSet<i32> = nums.into_iter().collect();
    let distinct_sum = distinct.into_iter().sum::<i32>();

    vec![
        nums_sum - distinct_sum,
        expected_sum - distinct_sum,
    ]
}

#[cfg(test)]
mod set_mismatch_tests {
    use crate::set_mismatch::find_error_nums;

    #[test]
    fn lc_case_1() {
        assert_eq!(find_error_nums(vec![3, 2, 3, 4, 6, 5]), vec![3, 1]);
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(find_error_nums(vec![3, 2, 2]), vec![2, 1]);
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn lc_case_4() {
        assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn lc_case_5() {
        assert_eq!(find_error_nums(vec![2, 2]), vec![2, 1]);
    }

    #[test]
    fn lc_case_6() {
        assert_eq!(find_error_nums(vec![3, 3, 1]), vec![3, 2]);
    }
}
