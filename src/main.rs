extern crate leetcode;

use leetcode::coin_change::solution;

fn main() {
    let r = solution::Solution::coin_change(vec![2], 3);

    println!("{:?}", r);
}
// 7
// 1, 7, 5, 2, 4, 3, 9
// 1 7, 7 5, 5 2, 2 4, 4 3, 3 9
// 1 7 5, 7 5 2, 5 2 4, 2 4 3, 4 3 9
// 1 7 5 2, 7 5 2 4, 5 2 4 3, 2 4 3 9
// 1 7 5 2 4, 7 5 2 4 3, 5 2 4 3 9
// 1 7 5 2 4 3, 7 5 2 4 3 9
// 1 7 5 2 4 3 9

// 4
// 3, 1, 2, 4
// 3 1, 1 2, 2 4
// 3 1 2, 1 2 4,
// 3 1 2 4

// 1 7 5 2 4 3 9
//
