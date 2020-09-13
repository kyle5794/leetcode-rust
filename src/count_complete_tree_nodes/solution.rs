pub struct Solution {}

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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn count_nodes(root: Option<Node>) -> i32 {
        match root {
            Some(root_node) => {
                let mut q: VecDeque<Node> = VecDeque::new();
                q.push_back(root_node);
                let mut counter = 0;
                while !q.is_empty() {
                    counter += 1;
                    let cur_node = q.pop_front().unwrap();
                    let node_clone = Rc::clone(&cur_node);
                    let clone = node_clone.borrow();
                    if let Some(left_node) = &clone.left {
                        let left = Rc::clone(left_node);
                        q.push_back(left)
                    }

                    if let Some(right_node) = &clone.right {
                        let right = Rc::clone(right_node);
                        q.push_back(right)
                    }
                }

                return counter;
            }
            None => return 0,
        }
    }
}
