use crate::list_node::ListNode;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut counter = HashMap::new();

    let mut current = list1;
    let (mut min, mut max) = (0, 0);

    while let Some(mut node) = current {
        current = node.next.take();

        counter.insert(node.val, counter.get(&node.val).unwrap_or(&0) + 1);

        if node.val > max {
            max = node.val;
        } else if node.val < min {
            min = node.val;
        }
    }

    current = list2;

    while let Some(mut node) = current {
        current = node.next.take();

        if let Some(prev) = counter.insert(node.val, 1) {
            counter.insert(node.val, prev + 1);
        }

        if node.val > max {
            max = node.val;
        } else if node.val < min {
            min = node.val;
        }
    }

    let mut head = None;
    let mut tail = &mut head;

    for n in min..=max {
        let count = counter.get(&n).unwrap_or(&0);

        for _ in 0..*count {
            *tail = Some(Box::new(ListNode::new(n)));

            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    head
}

#[cfg(test)]
mod merge_two_sorted_lists {
    use crate::list_node::{ListExt, ListNode};
    use crate::merge_two_sorted_lists::merge_two_lists;

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn lc_case_1() {
        assert_eq!(
            vec![1, 1, 2, 3, 4, 4] as Vec<i32>,
            merge_two_lists(
                ListNode::from_iter(&[1, 2, 4]),
                ListNode::from_iter(&[1, 3, 4])
            )
            .to_vec()
        )
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn lc_case_2() {
        assert_eq!(
            vec![] as Vec<i32>,
            merge_two_lists(ListNode::from_iter(&[]), ListNode::from_iter(&[])).to_vec()
        )
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn lc_case_3() {
        assert_eq!(
            vec![0] as Vec<i32>,
            merge_two_lists(ListNode::from_iter(&[]), ListNode::from_iter(&[0])).to_vec()
        )
    }

    #[test]
    fn does_handle_negative() {
        assert_eq!(
            vec![-3, -2, -1],
            merge_two_lists(ListNode::from_iter(&[-3, -2, -1]), ListNode::from_iter(&[])).to_vec()
        )
    }

    #[test]
    fn does_handle_negative_2() {
        assert_eq!(
            vec![-3, -2, -1],
            merge_two_lists(ListNode::from_iter(&[-3, -2]), ListNode::from_iter(&[-1])).to_vec()
        )
    }
}
