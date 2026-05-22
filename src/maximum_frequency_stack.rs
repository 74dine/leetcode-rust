use std::collections::HashMap;

struct FreqStack {
    freq_history: HashMap<usize, Vec<i32>>,
    freq: HashMap<i32, usize>,
    max_freq: usize,
}

#[allow(dead_code)]
impl FreqStack {
    fn new() -> Self {
        FreqStack {
            max_freq: 0,
            freq: HashMap::new(),
            freq_history: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let freq = match self.freq.insert(val, 1) {
            None => 1,
            Some(prev) => {
                self.freq.insert(val, prev + 1);
                prev + 1
            }
        };

        self.max_freq = self.max_freq.max(freq);

        match self.freq_history.get_mut(&freq) {
            None => {
                self.freq_history.insert(freq, vec![val]);
            }
            Some(arr) => {
                arr.push(val);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        let history = self.freq_history.get_mut(&self.max_freq).unwrap();
        let n = history.pop().unwrap();

        if history.is_empty() {
            self.max_freq -= 1;
        }

        match self.freq.get_mut(&n) {
            None => unreachable!(),
            Some(n_try) => {
                *n_try -= 1;
            }
        };

        n
    }
}

#[cfg(test)]
mod maximum_frequency_stack_tests {
    use crate::maximum_frequency_stack::FreqStack;

    #[test]
    fn lc_case_1() {
        let mut s = FreqStack::new();

        s.push(5);
        s.push(7);
        s.push(5);
        s.push(7);
        s.push(4);
        s.push(5);

        assert_eq!(5, s.pop());
        assert_eq!(7, s.pop());
        assert_eq!(5, s.pop());
        assert_eq!(4, s.pop());
    }

    #[test]
    fn does_init() {
        FreqStack::new();
    }

    #[test]
    fn does_push() {
        let mut s = FreqStack::new();

        s.push(1)
    }

    #[test]
    fn does_pop() {
        let mut s = FreqStack::new();

        s.push(1);
        s.push(2);
        s.push(2);
        s.push(3);

        assert_eq!(2, s.pop());
        assert_eq!(3, s.pop());
    }

    #[test]
    fn does_remove_in_order() {
        let mut s = FreqStack::new();

        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(3, s.pop());
        assert_eq!(2, s.pop());
        assert_eq!(1, s.pop());
    }

    #[test]
    fn does_prioritize_freq_then_order() {
        let mut s = FreqStack::new();

        s.push(1);
        s.push(2);
        s.push(3);
        s.push(2);

        assert_eq!(2, s.pop());
        assert_eq!(3, s.pop());
        assert_eq!(2, s.pop());
        assert_eq!(1, s.pop());
    }
}
