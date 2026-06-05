use std::collections::HashMap;

#[allow(dead_code)]
pub fn total_waviness(num1: i64, num2: i64) -> i64 {
    fn f(mut n: i64) -> i64 {
        if n < 0 {
            return 0;
        }

        let mut digits = Vec::with_capacity(16);
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.reverse();

        fn g(
            i: usize,
            tight: bool,
            start: bool,
            i2: i8,
            i1: i8,
            digits: &[u8],
            memo: &mut HashMap<(usize, bool, bool, i8, i8), (i64, i64)>,
        ) -> (i64, i64) {
            if i == digits.len() {
                return (0, 1);
            }

            let key = (i, tight, start, i2, i1);

            if !tight {
                if let Some(&cache) = memo.get(&key) {
                    return cache;
                }
            }

            let limit = if tight { digits[i] } else { 9 };

            let mut total_waviness = 0;
            let mut total_count = 0;

            for d in 0..=limit {
                let next_tight = tight && d == limit;

                if !start && d == 0 {
                    let (wav, cnt) = g(i + 1, next_tight, false, -1, -1, digits, memo);

                    total_waviness += wav;
                    total_count += cnt;
                    continue;
                }

                if !start {
                    let (wav, cnt) = g(i + 1, next_tight, true, -1, d as i8, digits, memo);

                    total_waviness += wav;
                    total_count += cnt;
                    continue;
                }

                let contribution =
                    if i2 != -1 && ((i2 < i1 && i1 > d as i8) || (i2 > i1 && i1 < d as i8)) {
                        1
                    } else {
                        0
                    };

                let (wav, cnt) = g(i + 1, next_tight, true, i1, d as i8, digits, memo);

                total_waviness += wav + contribution * cnt;
                total_count += cnt;
            }

            let ans = (total_waviness, total_count);

            if !tight {
                memo.insert(key, ans);
            }

            ans
        }

        let mut memo = HashMap::new();

        g(0, true, false, -1, -1, &digits, &mut memo).0
    }

    f(num2) - f(num1 - 1)
}

#[cfg(test)]
mod total_waviness_of_numbers_in_range_ii {
    use crate::total_waviness_of_numbers_in_range_ii::total_waviness;

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
        assert_eq!(7360000000000005, total_waviness(1, 1e15 as i64));
    }
}
