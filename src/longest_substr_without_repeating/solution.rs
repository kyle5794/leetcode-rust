pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length: usize = 0;
        // let mut char_arr: [usize; 128] = [0; 128];
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut i: usize = 0;
        for (idx, c) in s.chars().enumerate() {
            // let u = c as usize;
            // if char_arr[u] != 0 {
            //     if char_arr[u] >= i {
            //         i = char_arr[u];
            //     }
            // }

            if let Some(&j) = char_map.get(&c) {
                if j >= i {
                    i = j;
                }
            }

            let cur_length = idx - i + 1;
            if cur_length > max_length {
                max_length = cur_length;
            }
            char_map.insert(c, idx + 1);
            // char_arr[u] = idx + 1;
        }

        return max_length as i32;
    }
}
