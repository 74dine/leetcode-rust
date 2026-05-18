#[allow(dead_code)]
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut view = position
        .iter()
        .zip(speed.iter())
        .collect::<Vec<(&i32, &i32)>>();
    view.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    // println!("{:?}", view);

    let mut fleets = vec![];

    for (pos, spd) in view {
        let fleet = (target - pos) as f32 / *spd as f32;
        // println!("{}", fleet);

        if fleets.is_empty() {
            fleets.push(fleet);
            continue;
        }

        let p_fleet = fleets.last().unwrap();
        if fleet > *p_fleet {
            fleets.push(fleet);
        }
    }

    fleets.len() as i32
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

    #[test]
    fn lc_case_4() {
        assert_eq!(2, car_fleet(10, vec![6, 8], vec![3, 2]));
    }
}
