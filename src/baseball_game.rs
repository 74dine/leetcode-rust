#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<String>, i32)> {
    vec![
        (
            vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string(),
            ],
            30,
        ),
        (
            vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string(),
            ],
            27,
        ),
        (vec!["1".to_string(), "C".to_string()], 0),
        (vec!["-1".to_string(), "C".to_string()], 0),
        (vec!["-2".to_string(), "D".to_string()], -6),
        (vec!["0".to_string(), "3".to_string(), "+".to_string()], 6),
        // (vec!["1".to_string(), "+".to_string()], 2),         // invalid test case - LC constraints suggest "+" is guaranteed to have two previous entries
        // (vec!["-1".to_string(), "+".to_string()], -2),       // invalid test case - LC constraints suggest "+" is guaranteed to have two previous entries
        (vec!["0".to_string(), "D".to_string()], 0),
        (vec!["0".to_string()], 0),
        (vec!["1".to_string()], 1),
    ]
}

#[allow(dead_code)]
pub fn solve(ops: Vec<String>) -> i32 {
    cal_points(ops)
}

#[allow(dead_code)]
pub fn organize_result() {}

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut history: Vec<i32> = Vec::with_capacity(operations.len());

    for n in &operations {
        match n as &str {
            "+" => history.push(history[history.len() - 2] + history[history.len() - 1]),
            "D" => history.push(history[history.len() - 1] * 2),
            "C" => {
                history.pop();
            }
            _ => history.push(n.parse().unwrap()),
        }
    }

    history.into_iter().sum::<i32>()
}
