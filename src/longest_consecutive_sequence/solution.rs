pub struct Solution {}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut visited: HashSet<i32> = HashSet::with_capacity(l);
        let set: HashSet<i32> = nums.iter().cloned().collect();
        let mut max_streak = 0;
        for i in 0..l {
            let mut q: VecDeque<i32> = VecDeque::with_capacity(l);
            q.push_back(nums[i]);
            let mut current_streak = 0;
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                visited.insert(cur);
                let prev = cur - 1;
                let next = cur + 1;

                if !visited.contains(&prev) && set.contains(&prev) {
                    q.push_back(prev);
                }

                if !visited.contains(&next) && set.contains(&next) {
                    q.push_back(next);
                }
                current_streak += 1;
            }

            if current_streak > max_streak {
                max_streak = current_streak;
            }
        }

        max_streak
    }
}
