pub struct Solution {}

#[derive(Debug)]
struct Node {
    idx: usize,
    value: i32,
    list: Vec<usize>,
    count: i32,
}

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> Vec<Vec<usize>> {
        let len = a.len();
        let mut right: Vec<Vec<usize>> = vec![vec![]; len];
        let mut stack: Vec<Node> = Vec::with_capacity(1);
        let mut counter: Vec<i32> = vec![0; len];
        for (idx, value) in a.iter().rev().enumerate() {
            let i = len - 1 - idx;
            println!("{}", value);
            let mut list: Vec<usize> = vec![];
            let mut count = 1;
            while !stack.is_empty() && &stack.last().unwrap().value > value {
                let last = stack.pop().unwrap();
                list.push(last.idx);
                for v in last.list {
                    list.push(v);
                }
                count += last.count;
            }
            let new_node = Node {
                idx: i,
                value: *value,
                list: list.to_vec(),
                count: count,
            };

            stack.push(new_node);

            right[i] = list;
            counter[i] = count;
        }

        right
    }
}
