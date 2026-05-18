#[allow(dead_code)]
pub fn daily_temperatures(nums: Vec<i32>) -> Vec<i32> {
    let mut history = Vec::with_capacity(nums.len());
    let mut result = vec![0; nums.len()];

    for i in 0..nums.len() {
        while let Some(j) = history.pop_if(|x: &mut usize| nums[*x] < nums[i]) {
            result[j] = (i - j) as i32;
        }

        history.push(i);
    }

    result
}

// Naive: O(n^2) - TLE (47/48 cases)
// pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//     temperatures
//         .iter()
//         .enumerate()
//         .map(|(i, n)| {
//             return match temperatures.iter().skip(i).position(|m| *m > *n) {
//                 Some(j) => j as i32,
//                 None => 0,
//             };
//         })
//         .collect()
// }

#[cfg(test)]
mod daily_temperatures_tests {
    use crate::daily_temperatures::daily_temperatures;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(vec![1, 1, 1, 0], daily_temperatures(vec![30, 40, 50, 60]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(vec![1, 1, 0], daily_temperatures(vec![30, 60, 90]));
    }

    #[test]
    fn does_handle_flat_history() {
        assert_eq!(vec![0, 0, 0], daily_temperatures(vec![30, 30, 30]));
    }

    #[test]
    fn does_handle_desc_temp() {
        assert_eq!(vec![0, 0, 0], daily_temperatures(vec![50, 40, 30]));
    }

    #[test]
    fn does_handle_asc_temp() {
        assert_eq!(vec![1, 1, 0], daily_temperatures(vec![30, 40, 50]));
    }
}
