#[test]
pub fn solve() {
    let cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ];

    crate::run_cases!(cases, |input| max_profit(input));
}

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2).map(|pair| 0.max(pair[1] - pair[0])).sum()
}
