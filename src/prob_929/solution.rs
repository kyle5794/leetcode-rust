// Every email consists of a local name and a domain name, separated by the @ sign.

// For example, in alice@leetcode.com, alice is the local name, and leetcode.com is the domain name.

// Besides lowercase letters, these emails may contain '.'s or '+'s.

// If you add periods ('.') between some characters in the local name part of an email address, mail sent there will be forwarded to the same address without dots in the local name.  For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.  (Note that this rule does not apply for domain names.)

// If you add a plus ('+') in the local name, everything after the first plus sign will be ignored. This allows certain emails to be filtered, for example m.y+name@email.com will be forwarded to my@email.com.  (Again, this rule does not apply for domain names.)

// It is possible to use both of these rules at the same time.

// Given a list of emails, we send one email to each address in the list.  How many different addresses actually receive mails?
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