// Problem 1175
// Return the number of permutations of 1 to n so that prime numbers are at prime indices (1-indexed.)

// (Recall that an integer is prime if and only if it is greater than 1, and cannot be written as a product of two positive integers both smaller than it.)

// Since the answer may be large, return the answer modulo 10^9 + 7.

// Example 1:

// Input: n = 5
// Output: 12
// Explanation: For example [1,2,5,4,3] is a valid permutation, but [5,2,3,4,1] is not because the prime number 5 is at index 1.

// Example 2:

// Input: n = 100
// Output: 682289015

pub struct Solution;
impl Solution {
    const MODULO: i64 = 1000000007;
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut num_primes = 0;
        for i in 0..(n + 1) {
            if Self::is_prime(i) {
                num_primes += 1;
            }
        }

        let result = (Self::factorial(num_primes) % Self::MODULO)
            * (Self::factorial(n - num_primes) % Self::MODULO);

        return (result % Self::MODULO) as i32;
    }

    fn is_prime(n: i32) -> bool {
        return match n {
            0 | 1 => false,
            2 | 3 | 5 | 7 | 11 => true,
            _ => {
                let sqrt = (n as f64).sqrt() as i32;
                let mut found = false;
                for i in 2..(sqrt + 1) {
                    if n % i == 0 {
                        found = true;
                        break;
                    }
                }
                !found
            }
        };
    }

    fn factorial(n: i32) -> i64 {
        match n {
            0 | 1 => 1,
            _ => (Self::factorial(n - 1) * (n as i64)) % Self::MODULO,
        }
    }
}
