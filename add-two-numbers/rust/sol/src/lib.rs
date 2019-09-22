// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      val,
      next: None,
    }
  }
}

pub struct Solution ;

impl ListNode {
    fn cons(self, val: i32) -> Self {
        ListNode {
            next: Some(Box::new(self)),
            val
        }
    }

    fn from_vec(mut v: Vec<i32>) -> Option<Box<Self>> {
        let mut out: Option<Self> = None;
        while let Some(n) = v.pop() {
            if let Some(list) = out {
                out = Some(list.cons(n));
            } else {
                out = Some(ListNode::new(n))
            }
        }
        out.and_then(|l| Some(Box::new(l)))
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut nums = vec![];
        let mut carry = false;

        loop {
            match (l1.take(), l2.take()) {
                (None, None) => {
                    if carry {
                        nums.push(1);
                    }
                    break;
                },
                (Some(b), None) | (None, Some(b)) => {
                    let mut res = b.val;
                    if carry {
                        res += 1;
                        let q = res / 10;
                        res %= 10;
                        if q > 0 {
                            carry = true;
                        } else {
                            carry = false;
                        };
                    }
                    nums.push(res);
                    l1 = b.next;
                    l2 = None;
                },
                (Some(b1), Some(b2)) => {
                    let mut res = b1.val + b2.val;
                    if carry {
                        res += 1;
                    }
                    let q = res / 10;
                    res %= 10;
                    if q > 0 {
                        carry = true;
                    } else {
                        carry = false;
                    };
                    nums.push(res);
                    l1 = b1.next;
                    l2 = b2.next;
                },
            };
        }

        ListNode::from_vec(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn example() {
        let a = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        let b = Some(Box::new(
            ListNode::new(4)
                .cons(6)
                .cons(5)
        ));
        let expected = Some(Box::new(
            ListNode::new(8)
                .cons(0)
                .cons(7)
        ));
        assert_eq!(Solution::add_two_numbers(a, b), expected);
    }

    #[test]
    fn no_numbers() {
        assert_eq!(Solution::add_two_numbers(None, None), None);
    }

    #[test]
    fn one_number_left() {
        let a = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        let expected = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        assert_eq!(Solution::add_two_numbers(a, None), expected);
    }

    #[test]
    fn one_number_right() {
        let a = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        let expected = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        assert_eq!(Solution::add_two_numbers(None, a), expected);
    }

    #[test]
    fn last_number_carry() {
        let a = Some(Box::new(
            ListNode::new(3)
                .cons(4)
                .cons(2)
        ));
        let b = Some(Box::new(
            ListNode::new(7)
                .cons(4)
                .cons(2)
        ));
        let expected = Some(Box::new(
            ListNode::new(1)
                .cons(0)
                .cons(8)
                .cons(4)
        ));
        assert_eq!(Solution::add_two_numbers(a, b), expected);
    }
}
