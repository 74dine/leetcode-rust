#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(&'static str, bool)> {
    vec![
        ("", true), // invalid TC for LC (1 <= len(s) <= 10e4)
        ("()", true),
        ("[]", true),
        ("{}", true),
        ("()[]{}", true),
        ("(}", false),
        ("([)]", false),
        ("((", false),
        ("(", false),
    ]
}

#[allow(dead_code)]
pub fn solve(str: &str) -> bool {
    is_valid(str.to_string())
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }

    let bytes = s.as_bytes();
    let mut history = Vec::with_capacity(bytes.len() / 2);

    for n in bytes {
        match *n {
            b'(' => {
                history.push(*n);
            }
            b'[' => {
                history.push(*n);
            }
            b'{' => {
                history.push(*n);
            }

            b')' => {
                if history.pop_if(|x| *x == 40).is_none() {
                    return false;
                }
            }
            b']' => {
                if history.pop_if(|x| *x == 91).is_none() {
                    return false;
                }
            }
            b'}' => {
                if history.pop_if(|x| *x == 123).is_none() {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    history.is_empty()
}
