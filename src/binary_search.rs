pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if target < nums[0] || target > nums[nums.len() - 1] {
        return -1;
    }

    let (mut l, mut r) = (0, nums.len() - 1);

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m] == target {
            return m as i32;
        }

        if nums[m] > target {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    -1
}

#[cfg(test)]
mod binary_search_tests {
    use crate::binary_search::search;

    #[test]
    fn lc_case_1() {
        assert_eq!(4, search(vec![-1, 0, 3, 5, 9, 12], 9));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(-1, search(vec![-1, 0, 3, 5, 9, 12], 2));
    }
}
