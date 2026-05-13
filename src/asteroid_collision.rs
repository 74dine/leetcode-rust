#[allow(dead_code)]
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut space = vec![];

    for n in asteroids {
        if n >= 0 {
            space.push(n);
            continue;
        }

        let mut add = true;
        while let Some(last) = space.last() {
            if *last < 0 {
                break;
            }

            if *last == n.abs() {
                space.pop();
                add = false;
                break;
            }

            if *last < n.abs() {
                space.pop();
                continue;
            } else {
                add = false;
                break;
            }
        }

        if add {
            space.push(n);
        }
    }

    space
}

#[cfg(test)]
mod asteroid_collision_tests {
    use crate::asteroid_collision::asteroid_collision;

    #[test]
    fn lc_case_1() {
        assert_eq!(vec![5, 10], asteroid_collision(vec![5, 10, -5]));
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn lc_case_2() {
        assert_eq!(vec![] as Vec<i32>, asteroid_collision(vec![8, -8]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(vec![10], asteroid_collision(vec![10, 2, -5]));
    }

    #[test]
    fn lc_case_4() {
        assert_eq!(vec![-6, 2, 4], asteroid_collision(vec![3, 5, -6, 2, -1, 4]));
    }

    #[test]
    fn lc_case_5() {
        assert_eq!(vec![-2, -2, -2], asteroid_collision(vec![-2, -2, 1, -2]));
    }

    #[test]
    fn does_not_eliminate_negative_asteroids() {
        assert_eq!(vec![-4, -4], asteroid_collision(vec![3, -4, -4]));
    }

    #[test]
    fn does_not_eliminate_larger_positive_asteroids() {
        assert_eq!(vec![8], asteroid_collision(vec![8, -7]));
    }

    #[test]
    fn does_no_op_all_pos() {
        assert_eq!(vec![1, 2, 3], asteroid_collision(vec![1, 2, 3]));
    }

    #[test]
    fn does_no_op_all_neg() {
        assert_eq!(vec![-1, -2, -3], asteroid_collision(vec![-1, -2, -3]));
    }
}
