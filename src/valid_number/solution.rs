// [NOT DONE]

// // Validate if a given string can be interpreted as a decimal number.

// // Some examples:
// // "0" => true
// // " 0.1 " => true
// // "abc" => false
// // "1 a" => false
// // "2e10" => true
// // " -90e3   " => true
// // " 1e" => false
// // "e3" => false
// // " 6e-1" => true
// // " 99e2.5 " => false
// // "53.5e93" => true
// // " --6 " => false
// // "-+3" => false
// // "95a54e53" => false

// // Note: It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:

// //     Numbers 0-9
// //     Exponent - "e"
// //     Positive/negative sign - "+"/"-"
// //     Decimal point - "."

// println!(" 0.1  : {0}", prob_65::Solution::is_number(" 0.1 ".to_string()));
// println!("0 : {0}", prob_65::Solution::is_number("0".to_string()));
// println!("abc : {0}", prob_65::Solution::is_number("abc".to_string()));
// println!("1 a : {0}", prob_65::Solution::is_number("1 a".to_string()));
// println!("2e10 : {0}", prob_65::Solution::is_number("2e10".to_string()));
// println!("-90e3 : {0}", prob_65::Solution::is_number("-90e3".to_string()));
// println!("1e : {0}", prob_65::Solution::is_number("1e".to_string()));
// println!("e3 : {0}", prob_65::Solution::is_number("e3".to_string()));
// println!("6e-1 : {0}", prob_65::Solution::is_number("6e-1".to_string()));
// println!(" 99e2.5  : {0}", prob_65::Solution::is_number(" 99e2.5 ".to_string()));
// println!("53.5e93 : {0}", prob_65::Solution::is_number("53.5e93".to_string()));
// println!("--6  : {0}", prob_65::Solution::is_number("--6 ".to_string()));
// println!("-+3 : {0}", prob_65::Solution::is_number("-+3 ".to_string()));
// println!("95a54e53 : {0}", prob_65::Solution::is_number("95a54e53".to_string()));
// println!(". : {0}", prob_65::Solution::is_number(".".to_string()));
// println!("  : {0}", prob_65::Solution::is_number(" ".to_string()));
// println!("0 : {0}", prob_65::Solution::is_number("0".to_string()));
// println!("6ee69 : {0}", prob_65::Solution::is_number("6ee69".to_string()));
// println!(" 005047e+6 : {0}", prob_65::Solution::is_number(" 005047e+6".to_string()));
// println!(" 005047e6+ : {0}", prob_65::Solution::is_number(" 005047e6+".to_string()));

// // Of course, the context of these characters also matters in the input.
// pub struct Solution;
// impl Solution {
//     pub fn is_number(s: String) -> bool {
//         let x = s.trim();
//         let bytes: Vec<u8> = x.bytes().collect();
//         let mut len = x.len();
//         if len == 0 {
//             return false;
//         }
//         let mut offset = 0;
//         let mut digits_before_dot: i32 = -1;
//         let mut coeff: Vec<u32> = Vec::new();
//         let mut digits_before_power: i32 = -1;

//         if bytes[offset] == b'-' {
//             offset += 1;
//             len -= 1;
//         } else if bytes[offset] == b'+' {
//             offset += 1;
//             len -= 1;
//         }

//         let max_n = b'9';

//         while len > 0 {
//             let b = bytes[offset];
//             match b {
//                 b'0'..=b'9' => {
//                     if b > max_n {
//                         return false;
//                     }

//                     coeff.push(u32::from(b - b'0'));
//                     offset += 1;
//                     len -= 1;
//                 }
//                 b'a'..=b'z' => {
//                     if b != b'e' {
//                         return false;
//                     }

//                     if digits_before_power >= 0 {
//                         return false;
//                     }
//                     digits_before_power = coeff.len() as i32;
//                     if digits_before_power <= 0 {
//                         return false;
//                     }
//                     offset += 1;
//                     len -= 1;
//                 }
//                 b'A'..=b'Z' => return false,
//                 b'.' => {
//                     if digits_before_power >= 0 {
//                         return false;
//                     }

//                     if digits_before_dot >= 0 {
//                         return false;
//                     }

//                     digits_before_dot = coeff.len() as i32;
//                     offset += 1;
//                     len -= 1;
//                 }
//                 b'-' | b'+' => {
//                     if digits_before_power < 0 {
//                         return false;
//                     }
//                     offset += 1;
//                     len -= 1;
//                 }
//                 _ => return false,
//             }
//         }

       


//         if coeff.len() == 0 {
//             return false;
//         }

//         if coeff.len() as i32 - digits_before_power <= 0 {
//             return false;
//         }

//         match bytes[offset - 1] {
//             b'0'..=b'9' => true,
//             _ => false
//         }
//     }
// }
