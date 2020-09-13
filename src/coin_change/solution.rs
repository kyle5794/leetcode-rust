pub struct Solution {}

const BIG: i32 = 1000000000;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let total = (amount + 1) as usize;
        let mut n_coins: Vec<i32> = vec![-1; total];
        n_coins[0] = 0;
        for a in 1..total {
            let mut min_coins = BIG;
            for &coin in &coins {
                let ucoin = coin as usize;
                if a < ucoin || n_coins[a - ucoin] == -1 {
                    continue;
                }

                let next = n_coins[a - ucoin] + 1;

                if min_coins >= next {
                    min_coins = next
                }
            }

            if min_coins != BIG {
                n_coins[a] = min_coins;
            }
        }

        n_coins[amount as usize]
    }
}
