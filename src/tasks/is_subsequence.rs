///
/// 392. Is Subsequence
///
/// Given two strings s and t, return true if s is a subsequence of t,
/// or false otherwise.
///
/// A subsequence of a string is a new string that is formed from
/// the original string by deleting some (can be none) of the characters
/// without disturbing the relative positions of the remaining characters.
/// (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
use super::shared::Solution;

impl Solution {
    pub fn is_subsequence_1(s: String, t: String) -> bool {
        if t.len() == 0 {
            return s.len() == 0;
        }
        if s.len() == 0 {
            return true;
        }

        let mut main = t.chars();
        let mut sub = s.chars();

        let mut sub_char: Option<char> = None;
        let mut is_found = true;

        loop {
            let main_char = main.next();
            if is_found {
                sub_char = sub.next();
            }

            if !sub_char.is_none() && !main_char.is_none() {
                is_found = false;
            }

            if let Some(sub_char) = sub_char {
                if let Some(main_char) = main_char {
                    if main_char == sub_char {
                        is_found = true;
                    } else {
                        is_found = false;
                    }
                } else {
                    is_found = false;
                    break;
                }
            } else {
                break;
            }
        }

        is_found
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut main = t.chars();
        let mut is_found = true;

        for sub_char in s.chars() {
            loop {
                if let Some(main_char) = main.next() {
                    if main_char == sub_char {
                        is_found = true;
                        break;
                    }
                } else {
                    is_found = false;
                    break;
                }
            }
        }

        is_found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_subsequence_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let result = true;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }

    #[test]
    fn is_subsequence_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let result = false;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }

    #[test]
    fn is_subsequence_3() {
        let s = "".to_string();
        let t = "ahbgdc".to_string();
        let result = true;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }

    #[test]
    fn is_subsequence_4() {
        let s = "acb".to_string();
        let t = "ahbgdc".to_string();
        let result = false;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }

    #[test]
    fn is_subsequence_5() {
        let s = "bc".to_string();
        let t = "ahbgdc".to_string();
        let result = true;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }
}
