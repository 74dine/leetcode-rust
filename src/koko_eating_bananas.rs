#[allow(dead_code)]
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = piles.iter().max().unwrap();

    let (mut low, mut high) = (1, *max);

    while low <= high {
        let speed = low + (high - low) / 2;

        let total_hours = piles.iter().fold(0i64, |total, pile| {
            total + (*pile as f64 / speed as f64).ceil() as i64
        });

        if total_hours > h as i64 {
            low = speed + 1;
        } else {
            high = speed - 1;
        }
    }

    low
}

#[cfg(test)]
mod koko_eating_bananas_tests {
    use crate::koko_eating_bananas::min_eating_speed;

    #[test]
    fn lc_case_1() {
        assert_eq!(4, min_eating_speed(vec![3, 6, 7, 11], 8))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(30, min_eating_speed(vec![30, 11, 23, 4, 20], 5))
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(23, min_eating_speed(vec![30, 11, 23, 4, 20], 6))
    }

    #[test]
    // total hours exceed i32 limit
    fn lc_case_4() {
        assert_eq!(
            3,
            min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000)
        )
    }
}
