pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(i) => i as i32,
        _ => -1,
    }
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
