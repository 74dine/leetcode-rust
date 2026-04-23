use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_test_cases() -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    vec![
        (
            vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string(),
            ],
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
            ],
        ),
        (vec!["".to_string()], vec![vec!["".to_string()]]),
        (vec!["a".to_string()], vec![vec!["a".to_string()]]),
    ]
}

#[allow(dead_code)]
pub fn group_anagrams(str_arr: Vec<String>) -> Vec<Vec<String>> {
    let mut bucket_map: HashMap<[u8; 28], Vec<String>> = HashMap::new();

    for str in str_arr {
        let mut bucket: [u8; 28] = [0; 28];
        for char in str.chars() {
            bucket[char as usize - 97] += 1;
        }

        let known_bucket = bucket_map.get_mut(&bucket);

        match known_bucket {
            Some(cur_value) => {
                cur_value.push(str.to_owned());
            }
            None => {
                bucket_map.insert(bucket, vec![str]);
            }
        }
    }

    bucket_map.values().cloned().collect()
}

#[allow(dead_code)]
pub fn organize_result(mut result: Vec<Vec<String>>) -> Option<Vec<Vec<String>>> {
    for item in &mut result {
        item.sort();
    }

    result.sort();

    Some(result)
}
