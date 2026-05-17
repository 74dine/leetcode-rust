#[allow(dead_code)]
pub fn next_greater_element(nums: Vec<i32>, search_arr: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|x| {
            let Some(idx) = search_arr.iter().position(|&y| y == *x) else {
                return -1;
            };

            *search_arr[idx + 1..]
                .iter()
                .find(|&&y| y > *x)
                .unwrap_or(&-1)
        })
        .collect()
}

#[cfg(test)]
mod next_greater_element_i_tests {
    use crate::next_greater_element_i::next_greater_element;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![-1, 3, -1],
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(
            vec![3, -1],
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
        );
    }
}
