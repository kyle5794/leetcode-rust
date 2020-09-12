// Given the array nums consisting of n positive integers. You computed the sum of all non-empty continous subarrays from the array and then sort them in non-decreasing order, creating a new array of n * (n + 1) / 2 numbers.

// Return the sum of the numbers from index left to index right (indexed from 1), inclusive, in the new array. Since the answer can be a huge number return it modulo 10^9 + 7.

// Example 1:

// Input: nums = [1,2,3,4], n = 4, left = 1, right = 5
// Output: 13
// Explanation: All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.

// Example 2:

// Input: nums = [1,2,3,4], n = 4, left = 3, right = 4
// Output: 6
// Explanation: The given array is the same as example 1. We have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 3 to ri = 4 is 3 + 3 = 6.

// Example 3:

// Input: nums = [1,2,3,4], n = 4, left = 1, right = 10
// Output: 50

// Constraints:

//     1 <= nums.length <= 10^3
//     nums.length == n
//     1 <= nums[i] <= 100
//     1 <= left <= right <= n * (n + 1) / 2
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

pub struct Solution;
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
                next: idx+1,
            });
        }
        for counter in 0..right {
            let cur = pq.pop().unwrap();
            if counter >= left-1 {
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
