pub struct Solution {}

#[derive(Debug)]
struct V {
    value: i32,
    count: i32,
}
const MODULO: i32 = 1000000007;
impl Solution {
    pub fn sum_subarray_mins(a: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut dot: i32 = 0;
        let mut stack: Vec<V> = vec![];

        for value in a.iter() {
            let mut count = 1;
            while !stack.is_empty() && &stack.last().unwrap().value >= value {
                let last = stack.pop().unwrap();
                count += last.count;
                dot -= last.value * last.count;
            }
            let new = V {
                value: *value,
                count: count,
            };
            stack.push(new);
            dot += value * count;
            sum =  (sum + dot) % MODULO;
        }

        sum % MODULO
    }
}