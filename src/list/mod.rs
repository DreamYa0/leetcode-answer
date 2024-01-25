pub mod add_two_numbers;
pub mod remove;
pub mod linked_list;
pub mod reverse;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}