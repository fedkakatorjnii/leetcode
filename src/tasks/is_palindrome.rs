///
/// 9. Palindrome Number
///
/// https://leetcode.com/problems/palindrome-number/description/
///
/// Given an integer x, return true if x is a palindrome, and false otherwise.
///
/// Constraints:
/// - -2^31 <= x <= 2^31 - 1
///
/// Follow up: Could you solve it without converting the integer to a string?
///
use super::shared::Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        let numbers = x.to_string();
        let len = numbers.len();
        let half = len / 2;
        let numbers = numbers.chars().collect::<Vec<char>>();

        for key in 0..half {
            let first = numbers[key];
            let last = numbers[len - key - 1];

            if first != last {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zerro() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    #[ignore]
    fn is_negative_number() {
        assert_eq!(Solution::is_palindrome(-1), false);
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    // #[ignore]
    fn first() {
        assert_eq!(Solution::is_palindrome(1234321), true);
        assert_eq!(Solution::is_palindrome(12344321), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1121), false);
        // assert_eq!(Solution::is_palindrome(0), true);
        // assert!(Solution::is_palindrome(121));
    }
}
