pub trait ListExt {
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
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_iter(iter: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for i in iter {
            *tail = Some(Box::new(ListNode::new(*i)));

            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}
