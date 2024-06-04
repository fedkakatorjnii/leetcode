///
/// 202. Happy Number
///
/// https://leetcode.com/problems/happy-number/description/
///
/// Write an algorithm to determine if a number n is happy.
///
/// A happy number is a number defined by the following process:
/// - Starting with any positive integer, replace the number by
///   the sum of the squares of its digits;
/// - Repeat the process until the number equals 1 (where it will stay),
///   or it loops endlessly in a cycle which does not include 1;
/// - Those numbers for which this process ends in 1 are happy;
/// - Return true if n is a happy number, and false if not.
///
/// Constraints:
/// - 1 <= n <= 231 - 1.
///
use std::collections::HashSet;

use super::shared::Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set: HashSet<u32> = HashSet::new();
        let mut result = n as u32;

        loop {
            if result == 1 {
                break;
            }

            set.insert(result);

            let current = format!("{result}")
                .chars()
                .map(|ch| ch.to_digit(10).unwrap().pow(2))
                .sum::<u32>();
            result = current;
            if set.get(&current).is_some() {
                result = current;
                break;
            } else {
                set.insert(current);
            }
        }

        result == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_happy_1() {
        let n = 19;
        let result = true;
        // Explanation:
        // 1^2 + 9^2 = 82
        // 8^2 + 2^2 = 68
        // 6^2 + 8^2 = 100
        // 1^2 + 0^2 + 0^2 = 1

        assert_eq!(Solution::is_happy(n), result);
    }

    #[test]
    // #[ignore]
    fn is_happy_2() {
        let n = 2;
        let result = false;

        assert_eq!(Solution::is_happy(n), result);
    }
}
