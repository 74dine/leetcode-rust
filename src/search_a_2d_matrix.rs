#[allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let r_len = matrix.len();
    let c_len = matrix[0].len();

    let (mut l, mut r) = (0, r_len * c_len - 1);

    while l <= r {
        let mid = l + (r - l) / 2;

        let row = mid / c_len;
        let col = mid % c_len;

        if matrix[row][col] == target {
            return true;
        }

        if matrix[row][col] < target {
            l = mid + 1;
        } else {
            if mid == 0 {
                break;
            }

            r = mid - 1;
        }
    }

    false
}

#[cfg(test)]
mod search_a_2d_matrix_tests {
    use crate::search_a_2d_matrix::search_matrix;

    #[test]
    fn lc_case_1() {
        assert_eq!(
            true,
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            )
        );
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(
            false,
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            )
        )
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(false, search_matrix(vec![vec![1]], 0))
    }

    #[test]
    fn does_handle_negative() {
        assert_eq!(
            false,
            search_matrix(vec![vec![-7, -7, -7], vec![-2, -1, 0], vec![0, 0, 0]], -3)
        )
    }

    #[test]
    fn does_handle_negative_2() {
        assert_eq!(
            true,
            search_matrix(vec![vec![-7, -7, -7], vec![-2, -1, 0], vec![0, 0, 0]], -1)
        )
    }

    #[test]
    fn does_handle_empty() {
        assert_eq!(false, search_matrix(vec![], 0))
    }

    #[test]
    fn does_handle_single_elem() {
        assert_eq!(true, search_matrix(vec![vec![0]], 0))
    }

    #[test]
    fn does_handle_single_elem_2() {
        assert_eq!(false, search_matrix(vec![vec![0]], 1))
    }

    #[test]
    fn does_handle_single_row() {
        assert_eq!(true, search_matrix(vec![vec![1]], 1))
    }

    #[test]
    fn does_handle_single_row_2() {
        assert_eq!(true, search_matrix(vec![vec![1, 3]], 3))
    }

    #[test]
    fn does_handle_single_row_3() {
        assert_eq!(false, search_matrix(vec![vec![1, 3]], 2))
    }
}
