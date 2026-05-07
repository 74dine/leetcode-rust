#[allow(dead_code)]
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if mid == 0
            || mid == nums.len() - 1
            || (nums[mid] != nums[mid - 1] && nums[mid] != nums[mid + 1])
        {
            return nums[mid];
        }

        let left_size = mid - left - (nums[mid] == nums[mid - 1]) as usize;

        if left_size % 2 == 0 {
            left = mid + 1 + (nums[mid] == nums[mid + 1]) as usize;

            continue;
        }

        right = mid - 1 - (nums[mid] == nums[mid - 1]) as usize;
    }

    unreachable!();
}

#[cfg(test)]
mod single_element_in_a_sorted_array_tests {
    use crate::single_element_in_a_sorted_array::single_non_duplicate;

    #[test]
    fn lc_case_1() {
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
    }

    #[test]
    fn sol_at_start() {
        assert_eq!(single_non_duplicate(vec![1, 2, 2]), 1);
    }

    #[test]
    fn sol_in_middle() {
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3]), 2);
    }

    #[test]
    fn sol_at_end() {
        assert_eq!(single_non_duplicate(vec![1, 1, 2]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(single_non_duplicate(vec![1]), 1);
    }
}
