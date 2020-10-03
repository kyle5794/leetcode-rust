pub struct Solution {}

const BIG: i32 = 1_000_000_000;
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        Self::split_array_binary_search(nums, m)
    }

    fn split_array_binary_search(nums: Vec<i32>, m: i32) -> i32 {
        let l = nums.len();
        let mut search_max = 0;
        let mut search_min = nums[0];
        for i in 0..l {
            search_max += nums[i];
            if nums[i] >= search_min {
                search_min = nums[i];
            }
        }

        let mut min = BIG;

        while search_min <= search_max {
            let search_mid = (search_max + search_min) / 2;

            if Self::check(&nums, l, m, search_mid) {
                search_max = search_mid - 1;
                min = search_mid;
            } else {
                search_min = search_mid + 1;
            }
        }

        min
    }

    fn check(nums: &Vec<i32>, l: usize, m: i32, search_mid: i32) -> bool {
        let mut count = 0;
        let mut sum = 0;
        for i in 0..l {
            if nums[i] > search_mid {
                return false;
            }

            sum += nums[i];

            if sum > search_mid {
                count += 1;
                sum = nums[i];
            }
        }
        count += 1;

        count <= m
    }

    // fn split_array_dynamic(nums: Vec<i32>, m: i32) -> i32 {
    //     let l = nums.len();
    //     let size: usize = (m + 1) as usize;
    //     let mut steps: Vec<Vec<i32>> = vec![vec![BIG; size]; l + 1];
    //     let mut acc: Vec<i32> = vec![0; l + 1];
    //     for i in 0..l {
    //         acc[i + 1] = acc[i] + nums[i];
    //     }

    //     steps[0][0] = 0;

    //     for i in 1..(l + 1) {
    //         for j in 1..size {
    //             for k in 0..i {
    //                 let value = acc[i] - acc[k];
    //                 let local_max = match value > steps[k][j - 1] {
    //                     true => value,
    //                     false => steps[k][j - 1],
    //                 };

    //                 if local_max <= steps[i][j] {
    //                     steps[i][j] = local_max
    //                 }
    //             }
    //         }
    //     }

    //     for step in &steps {
    //         println!("{:?}", step);
    //     }

    //     steps[l][m as usize]
    // }
}
// j = 1
// 0 0 0 0 0 0
// 0
// 0
// 0
// 0

// 7 2 5 10 8
// 7 / 2 5 10 8 -> 25
// 7 2 / 5 10 8 -> 23
// 7 2 5 / 10 8 -> 18
// 7 2 5 10 / 8 -> 26

// 7 / 2 / 5 10 8 -> 20
// 7 / 2 5 / 10 8 -> 18
// 7 / 2 5 10 / 8 -> 17
// 7 2 / 5 / 10 8 -> 18
// 7 2 / 5 10 / 8 -> 15
// 7 2 5 / 10 / 8 -> 14

// 7 / 2 / 5 / 10 8 -> 18
// 7 / 2 / 5 10 / 8 -> 15
// 7 / 2 5 / 10 / 8 -> 10
// 7 2 / 5 / 10 / 8 -> 10

// +++++++++++++++++++++++++

// 1 / 2 3 4 5 -> 15
// 1 2 / 3 4 5 -> 12
// 1 2 3 / 4 5 -> 9   xxxxxxxxxxxxxxxxxx
// 1 2 3 4 / 5 -> 11

// 1 / 2 / 3 4 5 -> 12
// 1 / 2 3 / 4 5 -> 9
// 1 / 2 3 4 / 5 -> 9
// 1 2 / 3 / 4 5 -> 9
// 1 2 / 3 4 / 5 -> 7
// 1 2 3 / 4 / 5 -> 6  xxxxxxxxxxxxxxxxxx

// +++++++++++++++++++++++++
// 9 8 5 3 2
// 9 / 8 5 3 2 -> 18
// 9 8 / 5 3 2 -> 17 xxxxxxxxxxxxxxxxxx
// 9 8 5 / 3 2 -> 22
// 9 8 5 3 / 2 -> 25

// 9 / 8 / 5 3 2 -> 10 xxxxxxxxxxxxxxxxxxxxxxxx
// 9 / 8 5 / 3 2 -> 13
// 9 / 8 5 3 / 2 -> 16
// 9 8 / 5 / 3 2 -> 17
// 9 8 / 5 3 / 2 -> 17
// 9 8 5 / 3 / 2 -> 22

// 9 / 8 / 5 / 3 2 -> 9
// 9 / 8 / 5 3 / 2 -> 9
// 9 / 8 5 / 3 / 2 -> 13
// 9 8 / 5 / 3 / 2 -> 17

// +++++++++++++++++++++++++
// 7 8 5 3 2
// 7 / 8 5 3 2 -> 20 xxxxxxxxxxxxxxxxxx
// 7 8 / 5 3 2 -> 28
// 7 8 5 / 3 2 -> 33
// 7 8 5 3 / 2 -> 36

// 20 / 8 / 5 3 2 -> 20
// 20 / 8 5 / 3 2 -> 20
// 20 / 8 5 3 / 2 -> 20
// 20 8 / 5 / 3 2 -> 28
// 20 8 / 5 3 / 2 -> 28
// 20 8 5 / 3 / 2 -> 33

// 20 / 8 / 5 / 3 2 -> 20
// 20 / 8 / 5 3 / 2 -> 20
// 20 / 8 5 / 3 / 2 -> 20
// 20 8 / 5 / 3 / 2 -> 28

// +++++++++++++++++++++++++
// 1 9 3 7 6 7 8 2 9 5
// 1 / 9 3 7 6 7 8 2 9 5 -> 56
// 1 9 / 3 7 6 7 8 2 9 5 -> 47
// 1 9 3 / 7 6 7 8 2 9 5 -> 44
// 1 9 3 7 / 6 7 8 2 9 5 -> 37
// 1 9 3 7 6 / 7 8 2 9 5 -> 31 xxxxxxxxxxxxxxxxxx
// 1 9 3 7 6 7 / 8 2 9 5 -> 33
// 1 9 3 7 6 7 8 / 2 9 5 -> 41
// 1 9 3 7 6 7 8 2 / 9 5 -> 43
// 1 9 3 7 6 7 8 2 9 / 5 -> 52

//
