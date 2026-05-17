#[allow(dead_code)]
pub fn next_greater_element(nums: Vec<i32>, search_arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    // naive O(mn) approach
    for n in nums {
        let mut pair = -2;

        for m in &search_arr {
            if *m == n {
                pair = -1;
                continue;
            }

            if pair == -1 && *m > n {
                pair = *m;
                break;
            }
        }

        result.push(pair.max(-1));
    }

    result
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
