use crate::list_node::ListNode;

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
    use crate::list_node::ListExt;
    use crate::reverse_linked_list::{ListNode, reverse_list};

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
