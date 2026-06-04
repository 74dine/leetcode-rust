#[allow(dead_code)]
pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    (num1..=num2)
        .map(|mut d| {
            let mut p1 = d % 10;
            d /= 10;

            let mut p2 = d % 10;
            d /= 10;

            let mut sum = 0;
            while d > 0 {
                let p3 = d % 10;
                d /= 10;

                if (p1 < p2 && p2 > p3) || (p1 > p2 && p2 < p3) {
                    sum += 1;
                }

                p1 = p2;
                p2 = p3;
            }

            sum
        })
        .sum()
}

#[cfg(test)]
mod total_waviness_of_numbers_in_range_i {
    use crate::total_waviness_of_numbers_in_range_i::total_waviness;

    #[test]
    fn lc_case_1() {
        assert_eq!(3, total_waviness(120, 130))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(3, total_waviness(198, 202))
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(2, total_waviness(4848, 4848))
    }

    #[test]
    fn does_handle_flat() {
        assert_eq!(0, total_waviness(1, 1));
    }

    #[test]
    fn does_handle_flat_2() {
        assert_eq!(0, total_waviness(111, 111));
    }

    #[test]
    fn does_handle_flat_3() {
        assert_eq!(0, total_waviness(1111, 1111));
    }

    #[test]
    fn does_handle_range() {
        assert_eq!(5, total_waviness(121232, 121233));
    }
}
