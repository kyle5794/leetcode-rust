pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut cur_max_prod: i32 = nums[0];
        let mut cur_min_prod: i32 = nums[0];

        let mut max_prod: i32 = nums[0];

        for i in 1..nums.len() {
            let value = nums[i];
            if value < 0 {
                let temp = cur_max_prod;
                cur_max_prod = cur_min_prod;
                cur_min_prod = temp;
            }

            cur_max_prod *= value;
            if value >= cur_max_prod {
                cur_max_prod = value;
            }

            cur_min_prod *= value;
            if value <= cur_min_prod {
                cur_min_prod = value;
            }

            if cur_max_prod >= max_prod {
                max_prod = cur_max_prod
            }
        }

        max_prod
    }
}
