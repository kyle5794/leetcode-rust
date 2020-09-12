// Given a complete binary tree, count the number of nodes.

// Note:

// Definition of a complete binary tree from Wikipedia:
// In a complete binary tree every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.

// Example:

// Input:
//     1
//    / \
//   2   3
//  / \  /
// 4  5 6

// Output: 6

// Definition for a binary tree node.

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
pub struct Solution {}

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
