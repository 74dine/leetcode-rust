#[allow(dead_code)]
pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut prefix = Vec::with_capacity(nums.len());

    for n in &nums {
        prefix.push(sum);
        sum += n;
    }

    sum = 0;
    for (i, n) in nums.iter().enumerate().rev() {
        prefix[i] = (prefix[i] - sum).abs();
        sum += n;
    }

    prefix
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
