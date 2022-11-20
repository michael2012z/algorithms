#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for i in v.iter().rev() {
            let mut new_node = Box::new(ListNode::new(*i));
            new_node.next = head;
            head = Some(new_node);
        }
        head
    }

    pub fn list_to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut l: &Option<Box<ListNode>> = &l;
        let mut v: Vec<i32> = vec![];
        while !l.is_none() {
            v.push(l.as_ref().unwrap().val);
            l = &l.as_ref().unwrap().next;
        }
        v
    }
}
