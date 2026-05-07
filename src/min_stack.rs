#[derive(Default)]
struct MinStack {
    stack: Vec<i32>,
    min_history: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.min_history.is_empty() || self.min_history.last().unwrap() >= &val {
            self.min_history.push(val);
        }
    }

    fn pop(&mut self) {
        self.min_history.pop_if(|x| *x == self.stack.pop().unwrap());
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_history.last().unwrap()
    }
}

#[cfg(test)]
mod min_stack_tests {
    use crate::min_stack::MinStack;

    #[test]
    fn lc_case_1() {
        let mut stack = MinStack::new();

        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        assert_eq!(stack.get_min(), -3);

        stack.pop();

        assert_eq!(stack.top(), 0);

        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn lc_case_2() {
        let mut stack = MinStack::new();

        stack.push(0);
        stack.push(1);
        stack.push(0);

        assert_eq!(stack.get_min(), 0);

        stack.pop();

        assert_eq!(stack.get_min(), 0);

        stack.pop();

        assert_eq!(stack.get_min(), 0);

        stack.pop();

        stack.push(-2);
        stack.push(-1);
        stack.push(-2);

        assert_eq!(stack.get_min(), -2);

        stack.pop();

        assert_eq!(stack.top(), -1);

        assert_eq!(stack.get_min(), -2);

        stack.pop();

        assert_eq!(stack.get_min(), -2);

        stack.pop();
    }

    #[test]
    fn does_initialize() {
        MinStack::new();
    }

    #[test]
    fn does_push() {
        let mut stack = MinStack::new();

        stack.push(-2);
    }

    #[test]
    #[should_panic]
    fn does_pop() {
        let mut stack = MinStack::new();

        stack.push(-2);
        stack.pop();

        stack.top();
    }

    #[test]
    fn does_top() {
        let mut stack = MinStack::new();

        stack.push(-2);
        stack.push(0);
        stack.push(4);

        assert_eq!(stack.top(), 4);
    }

    #[test]
    fn does_get_min() {
        let mut stack = MinStack::new();

        stack.push(-78);
        stack.push(0);
        stack.push(10);

        assert_eq!(stack.get_min(), -78);
    }
}
