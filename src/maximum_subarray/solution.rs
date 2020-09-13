pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut current_sum = nums[0];
        let mut max_sum = nums[0];
        for i in 1..len {
            let local_sum = nums[i] + current_sum;
            if local_sum > nums[i] {
                current_sum = local_sum
            } else {
                current_sum = nums[i]
            }

            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }

        max_sum
    }
}
