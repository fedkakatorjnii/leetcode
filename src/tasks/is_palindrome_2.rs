///
/// 125. Valid Palindrome
///
/// A phrase is a palindrome if, after converting all uppercase letters
/// into lowercase letters and removing all non-alphanumeric characters,
/// it reads the same forward and backward.
/// Alphanumeric characters include letters and numbers.
///
/// Given a string s, return true if it is a palindrome, or false otherwise.
///
use super::shared::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let result = true;

        // amanaplanacanalpanama
        assert_eq!(Solution::is_palindrome(s), true);
    }

    #[test]
    fn is_palindrome_2() {
        let s = "race a car".to_string();
        let result = true;

        // raceacar
        assert_eq!(Solution::is_palindrome(s), false);
    }
}
