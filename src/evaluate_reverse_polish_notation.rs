#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut history = vec![];

    for n in tokens {
        match n.as_str() {
            "*" => {
                let rhs = history.pop().unwrap();
                let lhs = history.pop().unwrap();

                history.push(lhs * rhs);
            }
            "/" => {
                let rhs = history.pop().unwrap();
                let lhs = history.pop().unwrap();

                history.push(lhs / rhs);
            }
            "+" => {
                let rhs = history.pop().unwrap();
                let lhs = history.pop().unwrap();

                history.push(lhs + rhs);
            }
            "-" => {
                let rhs = history.pop().unwrap();
                let lhs = history.pop().unwrap();

                history.push(lhs - rhs);
            }
            _ => {
                history.push(n.parse::<i32>().unwrap());
            }
        }
    }

    history[0]
}

#[cfg(test)]
mod evaluate_reverse_polish_notation_tests {
    use crate::evaluate_reverse_polish_notation::eval_rpn;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            9,
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        );
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(
            6,
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ])
        );
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(
            22,
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ])
        )
    }
}
