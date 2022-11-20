use crate::leetcode::Solution;
use ds::list::ListNode;
/*
 * https://leetcode.com/problems/add-two-numbers/
 */
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut round: i32 = 0;
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = head.as_mut();
        let mut l1 = l1;
        let mut l2 = l2;
        while l1.is_some() || l2.is_some() || round != 0 {
            let i1 = if let Some(l) = l1 {
                l1 = l.next;
                l.val
            } else {
                0
            };
            let i2 = if let Some(l) = l2 {
                l2 = l.next;
                l.val
            } else {
                0
            };
            let current = Some(Box::new(ListNode::new((i1 + i2 + round) % 10)));
            round = (i1 + i2 + round) / 10;
            if tail.is_none() {
                head = current;
                tail = head.as_mut();
            } else {
                if let Some(t) = tail {
                    t.next = current;
                    tail = t.next.as_mut();
                }
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_0() {
        let l1 = ListNode::vec_to_list(vec![2, 4, 3]);
        let l2 = ListNode::vec_to_list(vec![5, 6, 4]);
        let sum = Solution::add_two_numbers(l1, l2);
        let sum = ListNode::list_to_vec(sum);
        assert_eq!(sum, vec![7, 0, 8]);
    }

    #[test]
    fn test_two_sum_1() {
        let l1 = ListNode::vec_to_list(vec![0]);
        let l2 = ListNode::vec_to_list(vec![0]);
        let sum = Solution::add_two_numbers(l1, l2);
        let sum = ListNode::list_to_vec(sum);
        assert_eq!(sum, vec![0]);
    }

    #[test]
    fn test_two_sum_2() {
        let l1 = ListNode::vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::vec_to_list(vec![9, 9, 9, 9]);
        let sum = Solution::add_two_numbers(l1, l2);
        let sum = ListNode::list_to_vec(sum);
        assert_eq!(sum, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
