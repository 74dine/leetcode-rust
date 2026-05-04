use std::collections::VecDeque;

struct MyStack {
    a: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack { a: VecDeque::new() }
    }

    fn push(&mut self, num: i32) {
        self.a.push_back(num);

        let count = self.a.len() - 1;

        for _i in 0..count {
            let val = self.a.pop_front().unwrap();
            self.a.push_back(val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.a.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.a.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.a.is_empty()
    }
}

#[cfg(test)]
mod implement_stack_using_queues_tests {
    use crate::implement_stack_using_queues::MyStack;

    //noinspection SpellCheckingInspection
    #[test]
    fn mystack_1_case() {
        let mut s = MyStack::new();

        s.push(1);
        s.push(2);

        let result = s.top();
        assert_eq!(result, 2);

        let result = s.pop();
        assert_eq!(result, 2);

        let result = s.empty();
        assert_eq!(result, false);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn mystack_push() {
        let mut s = MyStack::new();

        s.push(1);
        s.push(2);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn mystack_pop() {
        let mut s = MyStack::new();

        s.push(1);
        s.push(2);

        println!("{:?}", s.a);

        let result = s.pop();
        assert_eq!(result, 2);

        let result = s.pop();
        assert_eq!(result, 1);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn mystack_does_fill() {
        let mut s = MyStack::new();

        s.push(1);

        assert_eq!(s.empty(), false);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn mystack_does_empty() {
        let mut s = MyStack::new();

        s.push(1);
        s.push(2);
        s.push(3);

        let result = s.top();
        assert_eq!(result, 3);

        let result = s.empty();
        assert_eq!(result, false);

        for _i in 0..s.a.len() {
            s.pop();
        }

        let result = s.empty();
        assert_eq!(result, true);
    }
}
