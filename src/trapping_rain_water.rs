#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }

    let mut sum = 0;

    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_left = height[left];
    let mut max_right = height[right];

    while left < right {
        let capacity = if max_left <= max_right {
            max_left - height[left]
        } else {
            max_right - height[right]
        };

        max_left = max_left.max(height[left]);
        max_right = max_right.max(height[right]);

        if max_left <= max_right {
            left += 1;
        } else {
            right -= 1;
        }

        if capacity > 0 {
            sum += capacity;
        }
    }

    sum
}

#[cfg(test)]
mod trapping_rain_water_tests {
    use crate::trapping_rain_water::trap;

    #[test]
    fn lc_case_1() {
        assert_eq!(6, trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(9, trap(vec![4, 2, 0, 3, 2, 5]));
    }

    #[test]
    fn does_handle_flat_height() {
        assert_eq!(0, trap(vec![0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn does_handle_ascending_height() {
        assert_eq!(3, trap(vec![0, 1, 0, 2, 0, 3]));
    }

    #[test]
    fn does_handle_descending_height() {
        assert_eq!(3, trap(vec![3, 0, 2, 0, 1, 0]));
    }

    #[test]
    fn does_handle_single_elem() {
        assert_eq!(0, trap(vec![3]));
    }

    #[test]
    fn does_handle_double_elem() {
        assert_eq!(0, trap(vec![3, 0]));
    }
}
