// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

use crate::list_node::ListNode;

#[allow(dead_code)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);
    None
}

#[cfg(test)]
mod add_two_numbers_tests {
    use crate::add_two_numbers::add_two_numbers;
    use crate::list_node::ListNode;

    #[test]
    #[ignore]
    fn lc_case_1() {
        assert_eq!(
            ListNode::from_iter(&[7, 0, 8].to_owned()),
            add_two_numbers(
                ListNode::from_iter(&[2, 4, 3]),
                ListNode::from_iter(&[5, 6, 4])
            )
        )
    }

    #[test]
    #[ignore]
    fn lc_case_2() {
        assert_eq!(
            ListNode::from_iter(&[0]),
            add_two_numbers(ListNode::from_iter(&[0]), ListNode::from_iter(&[0]))
        )
    }

    #[test]
    #[ignore]
    fn lc_case_3() {
        assert_eq!(
            ListNode::from_iter(&[8, 9, 9, 9, 0, 0, 0, 1]),
            add_two_numbers(
                ListNode::from_iter(&[9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_iter(&[9, 9, 9, 9])
            )
        )
    }

    // #[test]
    // fn does_handle_max_range() {
    //     assert_eq!(
    //         ListNode::from_iter(&std::iter::repeat_n(9, 99).chain([8]).collect::<Vec<_>>()),
    //         add_two_numbers(
    //             ListNode::from_iter(&std::iter::repeat_n(9, 98).collect::<Vec<_>>()),
    //             ListNode::from_iter(&[1]),
    //         )
    //     )
    // }
}
