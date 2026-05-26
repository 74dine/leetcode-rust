pub fn my_sqrt(x: i32) -> i32 {
    if x == 1 {
        return x;
    }

    let (mut l, mut r) = (1, x / 2);

    while l <= r {
        let num = l + (r - l) / 2;

        let num_square = num as f64 * num as f64;
        if num_square == x as f64 {
            return num;
        }

        if num_square > x as f64 {
            r = num - 1;
        } else {
            l = num + 1;
        }
    }

    r
}

#[allow(dead_code)]
//noinspection SpellCheckingInspection
mod sqrtx_tests {
    use crate::sqrtx::my_sqrt;

    #[test]
    fn lc_case_1() {
        assert_eq!(2, my_sqrt(4));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(2, my_sqrt(8));
    }

    #[test]
    fn does_handle_zero() {
        assert_eq!(0, my_sqrt(0));
    }

    #[test]
    fn does_handle_one() {
        assert_eq!(1, my_sqrt(1));
    }

    #[test]
    fn does_handle_float32_overflow() {
        assert_eq!(46340, my_sqrt(2147483647));
    }
}
