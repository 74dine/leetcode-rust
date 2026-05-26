pub struct Solution;

static mut PICK: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    unsafe {
        if num == PICK {
            0
        } else if num < PICK {
            1
        } else {
            -1
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    unsafe fn guess_number(n: i32) -> i32 {
        unsafe {
            let (mut i, mut max) = (1, n);

            while i <= max {
                let mid = i + (max - i) / 2;

                match guess(mid) {
                    -1 => max = mid - 1,
                    1 => i = mid + 1,
                    0 => return mid,
                    _ => unreachable!(),
                }
            }

            unreachable!()
        }
    }
}

#[cfg(test)]
mod guess_number_higher_or_lower_tests {
    use super::*;

    #[test]
    fn lc_case_1() {
        unsafe {
            PICK = 6;
        }

        unsafe {
            assert_eq!(6, Solution::guess_number(10));
        }
    }

    #[test]
    fn lc_case_2() {
        unsafe {
            PICK = 1;
        }

        unsafe {
            assert_eq!(1, Solution::guess_number(1));
        }
    }

    #[test]
    fn lc_case_3() {
        unsafe {
            PICK = 1;
        }

        unsafe {
            assert_eq!(1, Solution::guess_number(2));
        }
    }
}
