pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let l1 = w1.len() + 1;
        let l2 = w2.len() + 1;
        let mut matrix = vec![vec![0; l2]; l1];

        for i in 0..l2 {
            matrix[0][i] = i;
        }

        for i in 0..l1 {
            matrix[i][0] = i;
        }

        for i in 1..l1 {
            for j in 1..l2 {
                let insert = matrix[i][j - 1] + 1;
                let mut min_dist = insert;
                let deletion = matrix[i - 1][j] + 1;
                if deletion <= min_dist {
                    min_dist = deletion;
                }
                if w1[i - 1] == w2[j - 1] {
                    let matched = matrix[i - 1][j - 1];
                    if matched <= min_dist {
                        min_dist = matched;
                    }
                } else {
                    let mismatched = matrix[i - 1][j - 1] + 1;
                    if mismatched <= min_dist {
                        min_dist = mismatched;
                    }
                }

                matrix[i][j] = min_dist;
            }
        }

        matrix[l1-1][l2-1] as i32
    }
}
//   - i n t e n t i o n
// - 0
// e
// x
// e
// c
// u
// t
// i
// o
// n
