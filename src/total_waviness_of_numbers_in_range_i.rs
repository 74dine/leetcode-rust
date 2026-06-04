#[allow(dead_code)]
pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    let num1 = num1.max(100);

    let mut sum = 0;
    let mut digits = Vec::with_capacity(6);
    for mut n in num1..=num2 {
        digits.clear();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let mut count = digits
            .windows(3)
            .filter(|d| d.len() == 3 && d[1] < d[0] && d[1] < d[2])
            .count();
        count += digits
            .windows(3)
            .filter(|d| d.len() == 3 && d[1] > d[0] && d[1] > d[2])
            .count();

        sum += count;
    }

    sum as i32
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
