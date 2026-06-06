#[allow(dead_code)]
pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut suffix = nums.iter().sum::<i32>();
    let mut prefix = 0;

    nums.iter()
        .map(|n| {
            suffix -= n;
            let diff = (prefix - suffix).abs();
            prefix += n;

            diff
        })
        .collect()
}

#[cfg(test)]
mod left_and_right_sum_difference_tests {
    use crate::left_and_right_sum_differences::left_right_difference;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![15, 1, 11, 22],
            left_right_difference(vec![10, 4, 8, 3])
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(vec![0], left_right_difference(vec![1]))
    }
}
