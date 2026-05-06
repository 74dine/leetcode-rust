struct MyQueue {
    a: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue { a: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let mut temp = Vec::with_capacity(self.a.len());

        for _ in 0..self.a.len() {
            temp.push(self.a.pop().unwrap());
        }

        self.a.push(x);

        for _ in 0..temp.len() {
            self.a.push(temp.pop().unwrap());
        }
    }

    fn pop(&mut self) -> i32 {
        self.a.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.a.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.a.is_empty()
    }
}

#[cfg(test)]
mod implement_queue_using_stacks_tests {
    use super::MyQueue;

    #[test]
    fn lc_case_1() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);

        assert_eq!(queue.peek(), 1);

        assert_eq!(queue.pop(), 1);

        assert_eq!(queue.empty(), false);
    }

    #[test]
    fn does_initialize() {
        MyQueue::new();
    }

    #[test]
    fn does_push() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);
    }

    #[test]
    fn does_pop() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), 1);
    }

    #[test]
    fn does_validate_empty() {
        let mut queue = MyQueue::new();

        assert_eq!(queue.empty(), true);

        queue.push(1);

        assert_eq!(queue.empty(), false);
    }
}
