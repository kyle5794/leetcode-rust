pub struct Solution;

use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    value: i32,
    next: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let modulo: i32 = 1000000007;
        let nu = n as usize;

        let mut pq: BinaryHeap<Node> = BinaryHeap::new();

        let mut sum = 0;
        for i in 0..nu {
            let idx = i as usize;
            pq.push(Node {
                value: nums[idx],
                next: idx + 1,
            });
        }
        for counter in 0..right {
            let cur = pq.pop().unwrap();
            if counter >= left - 1 {
                sum += cur.value;
                sum %= modulo;
            }

            if cur.next < nu {
                let new_node = Node {
                    value: cur.value + nums[cur.next],
                    next: cur.next + 1,
                };

                pq.push(new_node);
            }
        }

        return sum;
    }
}
