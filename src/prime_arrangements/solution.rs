pub struct Solution;

const MODULO: i64 = 1000000007;
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut num_primes = 0;
        for i in 0..(n + 1) {
            if Self::is_prime(i) {
                num_primes += 1;
            }
        }

        let result =
            (Self::factorial(num_primes) % MODULO) * (Self::factorial(n - num_primes) % MODULO);

        return (result % MODULO) as i32;
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
            _ => (Self::factorial(n - 1) * (n as i64)) % MODULO,
        }
    }
}
