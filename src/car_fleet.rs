#[allow(dead_code)]
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut view: Vec<_> = position.iter().zip(speed.iter()).collect();
    view.sort_unstable_by_key(|x| -x.0);

    view.into_iter()
        .fold((0f32, 0i32), |(p_fleet, mut count), (pos, spd)| {
            let fleet = (target - pos) as f32 / *spd as f32;
            if fleet > p_fleet {
                count += 1;
            }

            (p_fleet.max(fleet), count)
        })
        .1
}

#[cfg(test)]
mod car_fleet_tests {
    use crate::car_fleet::car_fleet;

    #[test]
    fn lc_case_1() {
        assert_eq!(3, car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(1, car_fleet(10, vec![3], vec![2]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(1, car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]));
    }

    // requires float
    #[test]
    fn lc_case_4() {
        assert_eq!(2, car_fleet(10, vec![6, 8], vec![3, 2]));
    }
}
