pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let r: HashSet<String> = emails
            .into_iter()
            .map(|email| {
                let results: Vec<&str> = email.split("@").collect();
                let local = results[0].replace(".", "");
                let uniques: Vec<&str> = local.split("+").collect();

                let mut owned_str = results[1].to_owned();
                owned_str.push_str(uniques[0]);
                owned_str
            })
            .collect();

        r.len() as i32
    }
}
