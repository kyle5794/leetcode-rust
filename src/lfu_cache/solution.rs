use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    key: i32,
    value: i32,
    count: i32,
    t: Duration,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other
            .count
            .cmp(&self.count)
            .then_with(|| other.t.cmp(&self.t))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct LFUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    capacity: i32,
    length: i32,
    pq: BinaryHeap<Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        LFUCache {
            map: HashMap::new(),
            capacity: capacity,
            pq: BinaryHeap::new(),
            length: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(node) => {
                {
                    let clone_rc = Rc::clone(node);
                    let mut clone = clone_rc.borrow_mut();
                    clone.count += 1;
                    clone.t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                }

                {
                    self.pq = BinaryHeap::new();
                    for (_key, elem) in self.map.iter() {
                        let clone = Rc::clone(elem);
                        // println!("rebuild key={}, count={}", clone.borrow().key, clone.borrow().count);
                        self.pq.push(clone);
                    }
                }

                return Rc::clone(node).borrow().value;
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        match self.map.get(&key) {
            Some(existing) => {
                {
                    let clone_rc = Rc::clone(&existing);
                    let mut clone = clone_rc.borrow_mut();
                    clone.value = value;
                    clone.count += 1;
                    clone.t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                }
                // println!("override key={}, value={}", key, value);
                {
                    self.pq = BinaryHeap::new();
                    for (_key, elem) in self.map.iter() {
                        let clone = Rc::clone(elem);
                        // println!("rebuild key={}, count={}", clone.borrow().key, clone.borrow().count);
                        self.pq.push(clone);
                    }
                }
            }
            None => {
                if self.length >= self.capacity {
                    if let Some(node) = self.pq.pop() {
                        let clone = Rc::clone(&node);
                        self.map.remove(&clone.borrow().key);
                        self.length -= 1;
                        // println!(
                        //     "evict key={}, value={}",
                        //     &clone.borrow().key,
                        //     &clone.borrow().value
                        // );
                    };
                }

                let node = Rc::new(RefCell::new(Node {
                    key: key,
                    value: value,
                    count: 0,
                    t: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
                }));

                println!("add key={}, value={}", key, value);

                let clone = Rc::clone(&node);
                self.pq.push(node);
                self.map.insert(key, clone);
                self.length += 1;
            }
        }
    }
}

// /**
//  * Your LFUCache object will be instantiated and called as such:
//  * let obj = LFUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */
