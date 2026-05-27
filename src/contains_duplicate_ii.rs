use std::collections::HashSet;

#[allow(dead_code)]
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;

    if k > 100 {
        let mut set = HashSet::with_capacity(k);

        for i in 0..nums.len() {
            if !set.insert(nums[i]) {
                return true;
            }

            if set.len() > k {
                set.remove(&nums[i - k]);
            }
        }

        return false;
    }

    for i in 0..nums.len() {
        if nums[i.saturating_sub(k)..i].contains(&nums[i]) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod contains_duplicate_ii_tests {
    use crate::contains_duplicate_ii::contains_nearby_duplicate;

    #[test]
    fn lc_case_1() {
        assert_eq!(true, contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(true, contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(false, contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
