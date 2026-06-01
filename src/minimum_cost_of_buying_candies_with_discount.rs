#[allow(dead_code)]
pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
    cost.sort_unstable();
    cost.rchunks(3)
        .flat_map(|chunk| chunk.iter().rev().take(2))
        .sum::<i32>()
}

#[cfg(test)]
mod minimum_cost_of_buying_candies_with_discount_tests {
    use crate::minimum_cost_of_buying_candies_with_discount::minimum_cost;

    #[test]
    fn lc_case_1() {
        assert_eq!(5, minimum_cost(vec![1, 2, 3]));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(23, minimum_cost(vec![6, 5, 7, 9, 2, 2]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(10, minimum_cost(vec![5, 5]));
    }
}
