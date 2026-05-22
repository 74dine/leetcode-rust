#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Err(i) => i as i32,
        Ok(i) => i as i32,
    }
}

#[cfg(test)]
mod search_insert_position_tests {
    use crate::search_insert_position::search_insert;

    #[test]
    fn lc_case_1() {
        assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(4, search_insert(vec![1, 3, 5, 6], 7));
    }

    #[test]
    fn does_handle_smaller_than_first() {
        assert_eq!(0, search_insert(vec![1, 3, 5, 6], 0));
    }
}
