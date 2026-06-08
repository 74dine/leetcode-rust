#[allow(dead_code)]
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    result.extend(nums.iter().filter(|&&n| n < pivot));
    result.extend(nums.iter().filter(|&&n| n == pivot));
    result.extend(nums.iter().filter(|&&n| n > pivot));

    result
}

#[cfg(test)]
mod partition_array_according_to_given_pivot_tests {
    use crate::partition_array_according_to_given_pivot::pivot_array;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![9, 5, 3, 10, 10, 12, 14],
            pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(vec![-3, 2, 4, 3], pivot_array(vec![-3, 4, 3, 2], 2))
    }
}
