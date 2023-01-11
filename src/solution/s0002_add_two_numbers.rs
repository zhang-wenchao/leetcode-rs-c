#![allow(dead_code)]

use crate::utils::list::ListNode;

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        carried(l1, l2, 0)
    }
}

pub fn carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    Some(Box::new(ListNode {
        next: carried(
            l1.and_then(|x| {
                carry += x.val;
                x.next
            }),
            l2.and_then(|x| {
                carry += x.val;
                x.next
            }),
            carry / 10,
        ),
        val: carry % 10,
    }))
}

#[cfg(test)]
mod tests {
    use crate::utils::list::to_list;
    use super::Solution;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
