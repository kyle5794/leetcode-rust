/**
 * Your AutocompleteSystem object will be instantiated and called as such:
 * let obj = AutocompleteSystem::new(sentences, times);
 * let ret_1: Vec<String> = obj.input(c);
 */
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str;

pub struct AutocompleteSystem {
    root: Rc<RefCell<Node>>,
    current_node: Option<Rc<RefCell<Node>>>,
    sentence: Option<String>,
    has_next: bool,
}

pub struct Node {
    pub pattern: u8,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub next_nodes: Vec<Rc<RefCell<Node>>>,
    pub rank: i32,
}

#[derive(Clone)]
pub struct Sentence {
    pub sentence: String,
    pub rank: i32,
}

impl AutocompleteSystem {
    pub fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        AutocompleteSystem {
            root: Self::build_tries(sentences, times),
            current_node: None,
            sentence: None,
            has_next: true,
        }
    }

    pub fn build_tries(sentences: Vec<String>, times: Vec<i32>) -> Rc<RefCell<Node>> {
        let root = Node {
            pattern: 2, // start of text `STX`
            next_nodes: vec![],
            rank: 0,
            parent: None,
        };

        let rc = Rc::new(RefCell::new(root));

        for (i, sentence) in sentences.iter().enumerate() {
            Self::add(Rc::clone(&rc), sentence, times[i]);
        }

        return rc;
    }

    fn add(parent: Rc<RefCell<Node>>, sentence: &str, rank: i32) {
        let mut current = Rc::clone(&parent);
        for &character in sentence.as_bytes() {
            current = Self::insert(current, character);
        }

        current = Self::insert(current, 35); // insert '#'
        current.borrow_mut().rank += rank;
    }

    fn insert(current: Rc<RefCell<Node>>, character: u8) -> Rc<RefCell<Node>> {
        let mut current_node = current.borrow_mut();
        for next_node in &current_node.next_nodes {
            let clone = Rc::clone(next_node);
            if clone.borrow().pattern == character {
                return Rc::clone(next_node);
            }
        }

        let node = Node {
            pattern: character,
            next_nodes: vec![],
            rank: 0,
            parent: Option::from(Rc::clone(&current)),
        };

        let new_node = Rc::new(RefCell::new(node));
        let clone = Rc::clone(&new_node);
        current_node.next_nodes.push(new_node);
        return clone;
    }

    pub fn input(&mut self, pattern: char) -> Vec<String> {
        let mut sentence: String = match &self.sentence {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        if pattern == '#' {
            let clone = Rc::clone(&self.root);
            Self::add(clone, &sentence, 1);
            self.reset();
            vec![]
        } else {
            let current = match &self.current_node {
                Some(node) => Rc::clone(&node),
                None => Rc::clone(&self.root),
            };

            sentence.push(pattern);
            self.sentence = Option::from(sentence);
            let mut results: Vec<Sentence> = Vec::new();
            if self.has_next {
                let mut has_next = false;
                for trie_root in &current.borrow().next_nodes {
                    let clone = Rc::clone(&trie_root);
                    if clone.borrow().pattern == pattern as u8 {
                        self.current_node = Option::from(clone);
                        results = Self::dfs(Rc::clone(&trie_root));
                        has_next = true;
                        break;
                    }
                }
                self.has_next = has_next;
            }

            results.sort_by(|a, b| {
                a.rank
                    .cmp(&b.rank)
                    .then_with(|| b.sentence.cmp(&a.sentence))
            });
            let mut count = 0;
            let mut r: Vec<String> = Vec::with_capacity(3);
            while !results.is_empty() && count < 3 {
                r.push(results.pop().unwrap().sentence);
                count += 1;
            }

            r
        }
    }

    // pub fn print(&self) {
    //     for trie_root in &self.root.borrow().next_nodes {
    //         let clone = Rc::clone(&trie_root);
    //         let result = Self::dfs(clone);
    //         for r in result {
    //             println!("{}: {}", r.sentence, r.rank);
    //         }
    //     }
    // }

    fn dfs(node: Rc<RefCell<Node>>) -> Vec<Sentence> {
        let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut result: Vec<Sentence> = Vec::new();
        q.push_back(node);
        while !q.is_empty() {
            let cur_node = q.pop_back().unwrap();
            let node_clone = Rc::clone(&cur_node);

            if node_clone.borrow().pattern == 35 {
                result.push(Self::read_str(node_clone));
                continue;
            }

            for child in &node_clone.borrow().next_nodes {
                q.push_back(Rc::clone(child))
            }
        }

        result
    }

    fn read_str(child: Rc<RefCell<Node>>) -> Sentence {
        let mut c: Vec<u8> = Vec::new();
        let rank = child.borrow().rank;
        if let Some(parent_node) = &child.borrow().parent {
            let mut parent_clone = Rc::clone(&parent_node);
            while parent_clone.borrow().pattern != 2 {
                c.push(parent_clone.borrow().pattern);

                let x = match &parent_clone.borrow().parent {
                    Some(pp) => Rc::clone(&pp),
                    None => panic!("rip"),
                };

                parent_clone = x;
            }
        }
        c.reverse();
        match str::from_utf8(&c) {
            Ok(s) => Sentence {
                sentence: s.to_string(),
                rank: rank,
            },
            Err(e) => panic!("failed to decode utf8 {:?}", e),
        }
    }

    fn reset(&mut self) {
        self.current_node = None;
        self.sentence = None;
        self.has_next = true;
    }
}
// pub fn build_suffix_array(sentence: &str) -> Vec<usize> {
//     let mut order = Self::sort_single(sentence);
//     let mut class = Self::compute_class(sentence, &order);
//     let mut size = 1;
//     let l = sentence.len();
//     while size < l {
//         order = Self::sort_double(sentence, size, order, &class);
//         class = Self::update_class(&order, class, size);
//         size *= 2;
//     }

//     order
// }

// // abeeaaebee#
// // count[0] = 1 #
// // count[1] = 3 a
// // count[2] = 2 b
// // count[3] = 0 c
// // count[4] = 0 d
// // count[5] = 5 e

// // count[0] = 1 #
// // count[1] = 4 #
// // count[2] = 6 b
// // count[3] = 6 c
// // count[4] = 6 d
// // count[5] = 11 e
// // ....
// // count[27] = 0 `space`
// // sort_single
// // order[0] = 10 #
// // order[10] = 9 e
// // order[9] = 8 e
// // order[5] = 7 b
// // order[8] = 6 e
// // order[3] = 5 a
// // order[2] = 4 a
// // order[7] = 3 e
// // order[6] = 2 e
// // order[4] = 1 b
// // order[1] = 0 a
// // class[10] = 0
// // class[]
// fn sort_single(sentence: &str) -> Vec<usize> {
//     let mut count: [i32; NUM_LETTERS] = [0; NUM_LETTERS];
//     let l = sentence.len();
//     let mut order: Vec<usize> = vec![0; l];
//     let bytes = sentence.as_bytes();
//     for b in bytes {
//         let idx: usize = match b {
//             32 => 27, // (space)
//             35 => 0,  // #
//             _ => (b - A + 1) as usize,
//         };
//         count[idx] += 1;
//     }

//     for i in 1..NUM_LETTERS {
//         count[i] += count[i - 1];
//     }

//     for i in (0..l).rev() {
//         let idx: usize = match bytes[i] {
//             32 => 27, // (space)
//             35 => 0,  // #
//             b => (b - A + 1) as usize,
//         };
//         count[idx] -= 1;
//         order[count[idx] as usize] = i;
//     }

//     order
// }

// fn sort_double(
//     sentence: &str,
//     size: usize,
//     order: Vec<usize>,
//     class: &Vec<usize>,
// ) -> Vec<usize> {
//     let l = sentence.len();
//     let mut count: Vec<i32> = vec![0; l];
//     let mut new_order: Vec<usize> = vec![0; l];
//     for i in 0..l {
//         count[class[i]] += 1;
//     }

//     for i in 1..l {
//         count[i] += count[i - 1];
//     }
//     for i in (0..l).rev() {
//         let start = ((order[i] as i32 - size as i32 + l as i32) % l as i32) as usize;
//         let cl = class[start];
//         count[cl] -= 1;
//         new_order[count[cl] as usize] = start;
//     }

//     new_order
// }

// fn compute_class(sentence: &str, order: &Vec<usize>) -> Vec<usize> {
//     let l = sentence.len();
//     let mut class: Vec<usize> = vec![0; l];
//     class[order[0] as usize] = 0;
//     let bytes = sentence.as_bytes();
//     for i in 1..l {
//         if bytes[order[i]] != bytes[order[i - 1]] {
//             class[order[i]] = class[order[i - 1]] + 1
//         } else {
//             class[order[i]] = class[order[i - 1]]
//         }
//     }

//     class
// }

// fn update_class(order: &Vec<usize>, class: Vec<usize>, size: usize) -> Vec<usize> {
//     let l = order.len();
//     let mut new_class: Vec<usize> = vec![0; l];

//     for i in 1..l {
//         let (cur, prev) = (order[i], order[i - 1]);
//         let (mid, mid_prev) = (cur + size, (prev + size) % l);
//         if class[cur] != class[prev] || class[mid] != class[mid_prev] {
//             new_class[cur] = new_class[prev] + 1
//         } else {
//             new_class[cur] = new_class[prev]
//         }
//     }

//     new_class
// }
// }
