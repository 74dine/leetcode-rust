#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut buy, mut max_profit) = (0, 0);

    for i in buy..prices.len() {
        if prices[i] > prices[buy] {
            let profit = prices[i] - prices[buy];
            if profit > max_profit {
                max_profit = profit;
            }
            continue;
        }

        buy = i;
    }

    max_profit
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_tests {
    use crate::best_time_to_buy_and_sell_stock::max_profit;

    #[test]
    fn lc_case_1() {
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]))
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(0, max_profit(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(9, max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 9]));
    }

    #[test]
    fn does_handle_single_elem() {
        assert_eq!(0, max_profit(vec![1]));
    }
}
