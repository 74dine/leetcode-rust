#[allow(dead_code)]
//noinspection SpellCheckingInspection
pub fn remove_kdigits(num: String, k: i32) -> String {
    if k as usize >= num.len() {
        return "0".into();
    }

    let mut k = k as usize;
    let mut digits = Vec::with_capacity(num.len() - k);
    let bytes = num.into_bytes();

    for n in bytes {
        while k > 0 && !digits.is_empty() && digits.last().unwrap() > &n {
            digits.pop();
            k -= 1;
        }

        digits.push(n);
    }

    // digits.push(bytes[0]);
    //
    // for cur in &bytes[1..] {
    //     let last_idx = digits.len() - 1;
    //
    //     if &digits[last_idx] > cur && k > 0 {
    //         digits[last_idx] = *cur;
    //
    //         k -= 1;
    //     } else {
    //         digits.push(*cur);
    //     }
    // }

    while k > 0 && !digits.is_empty() {
        digits.pop();
        k -= 1;
    }

    let mut start = 0;
    while start < digits.len() && digits[start] == b'0' {
        start += 1;
    }

    if start == digits.len() {
        return "0".into();
    }

    String::from_utf8(digits[start..].to_vec()).unwrap()
}

#[cfg(test)]
mod remove_k_digits_tests {
    use crate::remove_k_digits::remove_kdigits;

    #[test]
    fn lc_case_1() {
        assert_eq!("1219", remove_kdigits("1432219".to_owned(), 3))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!("200", remove_kdigits("10200".to_owned(), 1))
    }

    #[test]
    fn lc_case_3() {
        assert_eq!("0", remove_kdigits("10".to_owned(), 2))
    }

    #[test]
    fn lc_case_4() {
        assert_eq!("11", remove_kdigits("112".to_owned(), 1))
    }

    #[test]
    fn lc_case_5() {
        assert_eq!("0", remove_kdigits("10001".to_owned(), 4))
    }

    #[test]
    fn lc_case_6() {
        assert_eq!("0", remove_kdigits("33526221184202197273".to_owned(), 19))
    }
}
