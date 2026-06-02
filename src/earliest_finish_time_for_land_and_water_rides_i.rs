#[allow(dead_code)]
pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let mut min = 3000usize;

    for i in 0..land_start_time.len() {
        for j in 0..water_start_time.len() {
            let a = (land_start_time[i] + land_duration[i]).max(water_start_time[j])
                + water_duration[j];
            let b = (water_start_time[j] + water_duration[j]).max(land_start_time[i])
                + land_duration[i];

            let cur_min = a.min(b) as usize;
            if cur_min < min {
                min = cur_min;
            }
        }
    }

    min as i32
}

#[cfg(test)]
mod earliest_finish_time_for_land_and_water_rides_i_tests {
    use crate::earliest_finish_time_for_land_and_water_rides_i::earliest_finish_time;

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
            3000,
            earliest_finish_time(vec![1000], vec![1000], vec![1000], vec![1000])
        )
    }
}
