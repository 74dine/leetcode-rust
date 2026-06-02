#[allow(dead_code)]
pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let mut min = 3000usize;

    let water_rides = water_start_time.iter().zip(water_duration.iter());

    for (land_start, land_end) in land_start_time.iter().zip(land_duration.iter()) {
        for (water_start, water_end) in water_rides.to_owned() {
            let a = (land_start + land_end).max(*water_start) + *water_end;
            let b = (*water_start + *water_end).max(*land_start) + land_end;

            let l_min = a.min(b);

            if (l_min as usize) < min {
                min = l_min as usize;
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
