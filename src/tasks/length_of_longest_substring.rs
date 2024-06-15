use std::collections::HashSet;

///
/// https://leetcode.com/problems/longest-substring-without-repeating-characters
///
/// 3. Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest substring
/// without repeating characters.
///
/// Constraints:
/// 0 <= s.length <= 5 * 10^4;
/// s consists of English letters, digits, symbols and spaces.
///
use super::shared::Solution;

impl Solution {
    pub fn is_uniq_str(s: &str) -> bool {
        let mut set: HashSet<char> = HashSet::new();

        for ch in s.chars() {
            if set.get(&ch).is_some() {
                return false;
            }
            set.insert(ch);
        }
        true
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left_index = 0;
        let mut right_index = 0;
        let mut result = 0;
        let len = s.len();

        if len != 0 {
            result = 1
        }

        while right_index <= len {
            let current = &s[left_index..right_index];
            if Self::is_uniq_str(current) {
                let count = current.len() as i32;

                if count >= result {
                    result = count;
                }

                right_index += 1;
            } else {
                left_index += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn length_of_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = 3;

        // Explanation: The answer is "abc", with the length of 3.
        assert_eq!(Solution::length_of_longest_substring(s), result);
    }

    #[test]
    // #[ignore]
    fn length_of_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = 1;

        // Explanation: The answer is "b", with the length of 1.
        assert_eq!(Solution::length_of_longest_substring(s), result);
    }

    #[test]
    // #[ignore]
    fn length_of_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = 3;

        // Explanation: The answer is "wke", with the length of 3.
        //              Notice that the answer must be a substring,
        //              "pwke" is a subsequence and not a substring.
        assert_eq!(Solution::length_of_longest_substring(s), result);
    }

    #[test]
    // #[ignore]
    fn length_of_longest_substring_4() {
        let s = "au".to_string();
        let result = 2;

        assert_eq!(Solution::length_of_longest_substring(s), result);
    }
}
