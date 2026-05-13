#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let (mut m_1, mut c_1) = (0, 0);
    let (mut m_2, mut c_2) = (0, 0);

    for &num in &nums {
        if num == m_1 {
            c_1 += 1;
            continue;
        }

        if num == m_2 {
            c_2 += 1;
            continue;
        }

        if c_1 == 0 {
            m_1 = num;
            c_1 = 1;
            continue;
        }

        if c_2 == 0 {
            m_2 = num;
            c_2 = 1;
            continue;
        }

        c_1 -= 1;
        c_2 -= 1;
    }

    c_1 = 0;
    c_2 = 0;

    for &num in &nums {
        if num == m_1 {
            c_1 += 1;
        } else if num == m_2 {
            c_2 += 1;
        }
    }

    let mut result = vec![];
    let min_count = nums.len() / 3;
    //
    // println!("min: {} nums: {:?}", min_count, nums);
    // println!("m_1: {} c_1: {}", m_1, c_1);
    // println!("m_2: {} c_2: {}", m_2, c_2);

    if c_1 > min_count {
        result.push(m_1);
    }
    if c_2 > min_count {
        result.push(m_2);
    }

    result
}

#[cfg(test)]
mod majority_element_ii_tests {
    use crate::majority_element_ii::majority_element;

    #[test]
    fn lc_case_1() {
        assert_eq!(vec![3], majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(vec![1], majority_element(vec![1]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(vec![1, 2], majority_element(vec![1, 2]));
    }

    #[test]
    fn lc_case_4() {
        assert_eq!(vec![1], majority_element(vec![2, 1, 1, 3, 1, 4, 5, 6]));
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn does_return_empty_if_no_matching_elements() {
        assert_eq!(vec![] as Vec<i32>, majority_element(vec![3, 3, 2, 2, 1, 1]));
    }

    #[test]
    fn does_pick_single_majority() {
        assert_eq!(vec![3], majority_element(vec![3, 3, 3, 2, 2, 1]));
    }

    #[test]
    fn does_pick_double_majority() {
        assert_eq!(vec![3, 2], majority_element(vec![3, 3, 3, 2, 2, 2, 1]));
    }
}
