///
/// 2000. Reverse Prefix of Word
///
/// https://leetcode.com/problems/reverse-prefix-of-word/description/
///
/// Given a 0-indexed string word and a character ch,
/// reverse the segment of word that starts at index 0 and ends
/// at the index of the first occurrence of ch (inclusive).
/// If the character ch does not exist in word, do nothing.
///
/// - For example, if word = "abcdefd" and ch = "d",
///   then you should reverse the segment that starts at 0
///   and ends at 3 (inclusive). The resulting string will be "dcbaefd".
///
/// Return the resulting string.
///
/// Constraints:
/// - 1 <= word.length <= 250;
/// - word consists of lowercase English letters;
/// - ch is a lowercase English letter.
///
use super::shared::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut result = String::from("");
        let mut flag = false;

        for current in word.chars() {
            if flag {
                result = format!("{result}{current}");
            } else {
                result = format!("{current}{result}");
            }

            if current == ch {
                flag = true;
            }
        }

        if !flag {
            return word;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let word = String::from("abcdefd");
        let ch = 'd';

        assert_eq!(Solution::reverse_prefix(word, ch), String::from("dcbaefd"));
    }

    #[test]
    fn test_2() {
        let word = String::from("xyxzxe");
        let ch = 'z';

        assert_eq!(Solution::reverse_prefix(word, ch), String::from("zxyxxe"));
    }

    #[test]
    fn test_3() {
        let word = String::from("abcd");
        let ch = 'z';

        assert_eq!(Solution::reverse_prefix(word, ch), String::from("abcd"));
    }
}
