///
/// 121. Best Time to Buy and Sell Stock
///
/// You are given an array prices where prices[i] is the price
/// of a given stock on the ith day.
///
/// You want to maximize your profit by choosing a single day to buy
/// one stock and choosing a different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from this transaction.
/// If you cannot achieve any profit, return 0.
///
use super::shared::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut left = prices[0];
        let mut profit = 0;

        for right in prices {
            if right < left {
                left = right;
            } else {
                let current_profit = right - left;

                if current_profit > profit {
                    profit = current_profit;
                }
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = 5;
        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn max_profit_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = 0;
        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn max_profit_3() {
        let prices = vec![2, 1, 2, 1, 0, 1, 2];
        let result = 2;
        assert_eq!(Solution::max_profit(prices), result);
    }
}
