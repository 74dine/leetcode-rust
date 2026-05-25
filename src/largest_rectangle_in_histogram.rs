#[allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<(usize, &i32)> = vec![];

    for i in 0..heights.len() {
        let mut min = heights[i];
        let mut p = i;

        while let Some(&(_, last)) = stack.last() {
            if last <= &heights[i] {
                break;
            }

            let (j, &last) = stack.pop().unwrap();
            if last < min {
                min = last;
            }

            let area = (i - j) as i32 * last;
            if area > max_area {
                max_area = area;
            }

            p = j;
        }

        stack.push((p, &heights[i]));
    }

    while let Some((j, last)) = stack.pop() {
        let area = (heights.len() - j) as i32 * last;
        if area > max_area {
            max_area = area;
        }
    }

    max_area
}

#[cfg(test)]
mod largest_rectangle_in_histogram_tests {
    use crate::largest_rectangle_in_histogram::largest_rectangle_area;

    #[test]
    fn lc_case_1() {
        assert_eq!(10, largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(4, largest_rectangle_area(vec![2, 4]));
    }

    #[test]
    fn does_handle_min() {
        assert_eq!(3, largest_rectangle_area(vec![2, 1, 2]));
    }

    #[test]
    fn does_handle_zero() {
        assert_eq!(0, largest_rectangle_area(vec![0]));
    }

    #[test]
    fn does_handle_zero_2() {
        assert_eq!(0, largest_rectangle_area(vec![0, 0, 0]));
    }

    #[test]
    fn does_handle_zero_3() {
        assert_eq!(2, largest_rectangle_area(vec![0, 1, 1]));
    }
}
