
use std::collections::HashMap;

const RATE_LIMIT: i32 = 10;

pub struct Logger {
    msgs: HashMap<String, i32>,
}

impl Logger {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Logger {
            msgs: HashMap::new(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        match self.msgs.get(&message) {
            Some(ts) => {
                if timestamp - ts >= RATE_LIMIT {
                    self.msgs.insert(message, timestamp);
                    true
                } else {
                    false
                }
            }
            None => {
                self.msgs.insert(message, timestamp);
                true
            }
        }
    }
}
