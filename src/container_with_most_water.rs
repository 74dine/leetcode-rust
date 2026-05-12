#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);

    let mut max_area = height[l].min(height[r]) * (r - l) as i32;
    while l < r {
        let area = height[l].min(height[r]) * (r - l) as i32;

        if area > max_area {
            max_area = area;
        }

        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod container_with_most_water_tests {
    use super::max_area;

    #[test]
    fn lc_case_1() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(1, max_area(vec![1, 1]));
    }

    #[test]
    fn does_return_highest() {
        assert_eq!(2, max_area(vec![1, 2, 1]))
    }
}
