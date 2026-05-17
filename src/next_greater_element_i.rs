use std::collections::HashMap;

#[allow(dead_code)]
pub fn next_greater_element(nums: Vec<i32>, search_arr: Vec<i32>) -> Vec<i32> {
    let mut queue = vec![];
    let mut known = HashMap::new();

    for &num in &search_arr {
        while let Some(&next) = queue.last() {
            if next < num {
                known.insert(next, num);
                queue.pop();
                continue;
            }

            break;
        }

        queue.push(num);
    }

    nums.iter()
        .map(|x| *known.get(&x).unwrap_or(&-1))
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

    #[test]
    fn lc_case_3() {
        assert_eq!(
            vec![-1, 3, 3],
            next_greater_element(vec![3, 1, 2], vec![2, 1, 3])
        )
    }
}
