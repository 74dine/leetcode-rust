#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    fn parse(idx: usize, bytes: &[u8]) -> String {
        let mut result = String::new();
        if idx >= bytes.len() {
            return result;
        }

        let mut i = idx;

        while i < bytes.len() {
            match bytes[i] {
                b'0'..=b'9' => {
                    let start = i;

                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        i += 1;
                    }

                    let k: usize = std::str::from_utf8(&bytes[start..i])
                        .unwrap()
                        .parse()
                        .unwrap();

                    let mut count = 0;
                    let mut j = i;
                    let mut start = i;

                    while j < bytes.len() {
                        match bytes[j] {
                            b'[' => {
                                if count == 0 {
                                    start = j + 1;
                                }
                                count += 1;
                            }
                            b']' => {
                                count -= 1;
                                if count == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        j += 1;
                    }

                    let inner = parse(0, &bytes[start..j]);

                    for _ in 0..k {
                        result.push_str(&inner);
                    }

                    i = j;
                }
                _ => result.push(bytes[i] as char),
            }
            i += 1;
        }

        result
    }

    parse(0, &s.into_bytes())
}

#[cfg(test)]
mod decode_string_tests {
    use crate::decode_string::decode_string;

    #[test]
    fn lc_case_1() {
        assert_eq!("aaabcbc", decode_string("3[a]2[bc]".to_owned()));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!("accaccacc", decode_string("3[a2[c]]".to_owned()));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!("abcabccdcdcdef", decode_string("2[abc]3[cd]ef".to_owned()));
    }
}
