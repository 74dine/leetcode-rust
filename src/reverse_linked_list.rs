trait ListExt {
    fn to_vec(&self) -> Vec<i32>;
}

impl ListExt for Option<Box<ListNode>> {
    fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.as_deref();

        while let Some(node) = current {
            values.push(node.val);
            current = node.next.as_deref();
        }

        values
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_iter(iter: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for i in iter {
            *tail = Some(Box::new(ListNode::new(*i)));

            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

#[allow(dead_code)]
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut previous = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();

        node.next = previous;

        previous = Some(node);
    }

    previous
}

#[cfg(test)]
mod reverse_linked_list_tests {
    use crate::reverse_linked_list::{ListExt, ListNode, reverse_list};

    #[test]
    fn lc_case_1() {
        assert_eq!(
            vec![5, 4, 3, 2, 1],
            reverse_list(ListNode::from_iter(&[1, 2, 3, 4, 5])).to_vec()
        )
    }

    #[test]
    fn lc_case_2() {
        assert_eq!(
            vec![1, 2],
            reverse_list(ListNode::from_iter(&[2, 1])).to_vec()
        )
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn lc_case_3() {
        assert_eq!(
            vec![] as Vec<i32>,
            reverse_list(ListNode::from_iter(&[])).to_vec()
        )
    }

    #[test]
    fn does_handle_single_elem() {
        assert_eq!(vec![1], reverse_list(ListNode::from_iter(&[1])).to_vec())
    }
}
