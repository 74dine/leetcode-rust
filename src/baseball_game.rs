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
        (vec!["1".to_string(), "+".to_string()], 2),
        (vec!["-1".to_string(), "+".to_string()], -2),
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
    let mut history: Vec<i32> = vec![];

    for n in &operations {
        match n as &str {
           "+" => {
                let last = history.last().unwrap();
                let second_last = history.iter().nth(history.len() - 2).unwrap();

                let sum_of_last_two = *second_last + *last;

                history.push(sum_of_last_two);
            }
            "D" => {
                let previous = history.last();
                if previous != None {
                    let double = previous.unwrap() * 2;
                    history.push(double);
                }
            }
            "C" => {
                history.pop();
            }
            _ => {
                let int = n.parse::<i32>().unwrap();

                history.push(int);
            }
        }
    }

    history.iter().sum::<i32>()
}
