#[allow(dead_code)]
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();

    let mut boats = 0;
    let mut l = 0i32;
    let mut r = people.len() as i32;

    while l < r {
        if people[r as usize - 1] + people[l as usize] <= limit {
            l += 1;
        }

        boats += 1;
        r -= 1
    }

    boats
}

#[cfg(test)]
mod boats_to_save_people_tests {
    use crate::boats_to_save_people::num_rescue_boats;

    #[test]
    fn lc_case_1() {
        assert_eq!(1, num_rescue_boats(vec![1, 2], 3))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(3, num_rescue_boats(vec![3, 2, 2, 1], 3))
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(4, num_rescue_boats(vec![3, 5, 3, 4], 5))
    }

    #[test]
    fn lc_case_4() {
        assert_eq!(2, num_rescue_boats(vec![2, 4], 5))
    }

    #[test]
    fn does_pair_people() {
        assert_eq!(3, num_rescue_boats(vec![1, 1, 1, 1, 1, 1], 2))
    }

    #[test]
    fn does_handle_single_person() {
        assert_eq!(1, num_rescue_boats(vec![1], 3))
    }
}
