pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut low: i32 = -2147483648;
        let mut high: i32 = 0;
        // search space [low, high]
        // to search faster -> binary search in [low, high] to find the optimal low
        for w in &weights {
            high += *w;
            low = match low <= *w {
                true => *w,
                false => low,
            };
        }

        while low <= high {
            let mid = (high - low) / 2 + low;
            let mut per_day = 0;
            let mut day_req = 1;

            for w in &weights {
                per_day += *w;
                if per_day > mid {
                    day_req += 1;
                    per_day = *w;
                }
            }

            if day_req > d {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        low
    }
}
