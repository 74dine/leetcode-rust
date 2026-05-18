#[derive(Default)]
#[allow(dead_code)]
struct StockSpanner {
    history: Vec<(usize, usize)>,
}

#[allow(dead_code)]
impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut count = 1;

        while let Some(prev) = self.history.pop_if(|(m, _)| *m <= price as usize) {
            count += prev.1;
        }

        self.history.push((price as usize, count));

        count as i32
    }
}

#[cfg(test)]
mod online_stock_span_tests {
    use crate::online_stock_span::StockSpanner;

    #[test]
    fn lc_case_1() {
        let mut spanner = StockSpanner::new();
        assert_eq!(1, spanner.next(100));
        assert_eq!(1, spanner.next(80));
        assert_eq!(1, spanner.next(60));
        assert_eq!(2, spanner.next(70));
        assert_eq!(1, spanner.next(60));
        assert_eq!(4, spanner.next(75));
        assert_eq!(6, spanner.next(85));
    }

    #[test]
    fn lc_case_2() {
        let mut spanner = StockSpanner::new();

        assert_eq!(1, spanner.next(28));
        assert_eq!(1, spanner.next(14));
        assert_eq!(3, spanner.next(28));
        assert_eq!(4, spanner.next(35));
        assert_eq!(5, spanner.next(46));
        assert_eq!(6, spanner.next(53));
        assert_eq!(7, spanner.next(66));
        assert_eq!(8, spanner.next(80));
        assert_eq!(9, spanner.next(87));
        assert_eq!(10, spanner.next(88));
    }

    #[test]
    fn does_init() {
        StockSpanner::new();
    }

    #[test]
    fn does_add_to_history_using_next() {
        let mut spanner = StockSpanner::new();
        spanner.next(1);
    }

    #[test]
    fn does_calculate_span() {
        let mut spanner = StockSpanner::new();

        assert_eq!(1, spanner.next(1));
        assert_eq!(2, spanner.next(2));
        assert_eq!(3, spanner.next(3));
        assert_eq!(4, spanner.next(4));
        assert_eq!(5, spanner.next(5));
        assert_eq!(6, spanner.next(6));
        assert_eq!(7, spanner.next(7));
        assert_eq!(8, spanner.next(8));
        assert_eq!(9, spanner.next(9));
        assert_eq!(10, spanner.next(10));
        assert_eq!(1, spanner.next(1));
        assert_eq!(2, spanner.next(2));
        assert_eq!(3, spanner.next(3));
        assert_eq!(4, spanner.next(4));
        assert_eq!(5, spanner.next(5));
    }
}
