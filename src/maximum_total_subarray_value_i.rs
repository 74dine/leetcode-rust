#[allow(dead_code)]
pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();

    k as i64 * (max - min) as i64
}

#[cfg(test)]
mod maximum_total_subarray_value_i {
    use super::*;

    #[test]
    fn lc_case_1() {
        assert_eq!(4, max_total_value(vec![1, 3, 2], 2))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(12, max_total_value(vec![4, 2, 5, 1], 3))
    }

    #[test]
    fn handles_min_zero() {
        assert_eq!(6, max_total_value(vec![0, 3, 2], 2))
    }

    #[test]
    fn handles_max_zero() {
        assert_eq!(0, max_total_value(vec![0, 0, 0], 2))
    }

    #[test]
    fn handles_max_i_value() {
        assert_eq!(
            i32::MAX as i64 * 2i64,
            max_total_value(vec![0, i32::MAX], 2)
        )
    }
}
