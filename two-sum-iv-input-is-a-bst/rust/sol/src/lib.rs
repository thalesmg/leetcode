struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

impl TreeNode {
    pub fn merge(a: Option<Rc<RefCell<Self>>>, b: Option<Rc<RefCell<Self>>>) -> Option<Rc<RefCell<Self>>> {
        match (a, b) {
            (None, None) => None,
            (None, b@Some(_)) => b,
            (a@Some(_), None) => a,
            (Some(a_), Some(b_)) => {
                if a_.borrow().val > b_.borrow().val {
                    Some(Rc::new(RefCell::new(
                        TreeNode {
                            val: a_.borrow().val,
                            right: a_.borrow().right.clone(),
                            left: TreeNode::merge(
                                a_.borrow().left.clone(),
                                Some(b_)
                            ),
                        }
                    )))
                } else {
                    Some(Rc::new(RefCell::new(
                        TreeNode {
                            val: b_.borrow().val,
                            right: b_.borrow().right.clone(),
                            left: TreeNode::merge(
                                b_.borrow().left.clone(),
                                Some(a_)
                            ),
                        }
                    )))
                }
            },
        }
    }

    pub fn contains(tree: &Option<Rc<RefCell<Self>>>, x: i32) -> bool {
        match tree {
            None => false,
            Some(root) => {
                let root = root.borrow();
                match x.cmp(&root.val) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::contains(&root.left, x),
                    Ordering::Greater => Self::contains(&root.right, x),
                }
            }
        }
    }
}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if let Some(tree) = root {
            let tree = tree.borrow();
            let subtarget = k - tree.val;
            if subtarget < tree.val {
                TreeNode::contains(&tree.left, subtarget) ||
                    Solution::find_target(TreeNode::merge(tree.left.clone(), tree.right.clone()), k)
            } else {
                TreeNode::contains(&tree.right, subtarget) ||
                    Solution::find_target(TreeNode::merge(tree.left.clone(), tree.right.clone()), k)
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use super::Solution;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn ex1() {
        let tree = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        };
        assert!(Solution::find_target(Some(Rc::new(RefCell::new(tree))), 9));
    }

    #[test]
    fn ex2() {
        let tree = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        };
        assert!(!Solution::find_target(Some(Rc::new(RefCell::new(tree))), 28));
    }

    #[test]
    fn empty() {
        assert!(!Solution::find_target(None, 9));
    }

    #[test]
    fn ex3() {
        let tree = TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: None,
                    right: None,
                }))),
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        };
        assert!(Solution::find_target(Some(Rc::new(RefCell::new(tree))), -2));
    }
}
