#[allow(dead_code)]
pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    fn calculate(
        land_start_time: &Vec<i32>,
        land_duration: &Vec<i32>,
        water_start_time: &Vec<i32>,
        water_duration: &Vec<i32>,
    ) -> i32 {
        let mut min = 300000;
        let mut min_start = 200000;
        for i in 0..land_start_time.len() {
            let duration = land_start_time[i] + land_duration[i];

            if duration < min_start {
                min_start = duration;
            }
        }

        for j in 0..water_start_time.len() {
            let duration = water_start_time[j].max(min_start) + water_duration[j];

            if duration < min {
                min = duration;
            }
        }

        min
    }

    calculate(
        &land_start_time,
        &land_duration,
        &water_start_time,
        &water_duration,
    )
    .min(calculate(
        &water_start_time,
        &water_duration,
        &land_start_time,
        &land_duration,
    ))
}

#[cfg(test)]
mod earliest_finish_time_for_land_and_water_rides_ii_tests {
    use crate::earliest_finish_time_for_land_and_water_rides_ii::earliest_finish_time;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            9,
            earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3])
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(
            14,
            earliest_finish_time(vec![5], vec![3], vec![1], vec![10])
        )
    }

    #[test]
    fn does_handle_gap_between_rides() {
        assert_eq!(11, earliest_finish_time(vec![3], vec![4], vec![8], vec![3]))
    }

    #[test]
    fn does_handle_max_constraint() {
        assert_eq!(
            300000,
            earliest_finish_time(vec![100000], vec![100000], vec![100000], vec![100000])
        )
    }
}
