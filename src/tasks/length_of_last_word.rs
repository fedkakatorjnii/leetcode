///
/// 58. Length of Last Word
///
/// https://leetcode.com/problems/length-of-last-word/description/
///
/// Given a string s consisting of words and spaces,
/// return the length of the last word in the string.
///
/// A word is a maximal substring consisting of non-space characters only.
///
/// Constraints:
/// - 1 <= s.length <= 10^4;
/// s consists of only English letters and spaces ' ';
/// There will be at least one word in s.
///
use super::shared::Solution;

impl Solution {
    pub fn length_of_last_word_1(s: String) -> i32 {
        let rev_s = s.chars().rev().collect::<String>();
        let chars = rev_s.chars();
        let mut count = 0;

        for char in chars {
            if char != ' ' {
                count += 1;
            } else if count != 0 {
                break;
            }
        }

        count
    }

    pub fn length_of_last_word_2(s: String) -> i32 {
        let mut count = 0;

        for char in s.chars().rev() {
            if char != ' ' {
                count += 1;
            } else if count != 0 {
                break;
            }
        }

        count
    }

    pub fn length_of_last_word(s: String) -> i32 {
        match s.split_whitespace().next_back() {
            Some(data) => data.len() as i32,
            None => 0,
        }
    }

    // pub fn length_of_last_word(s: String) -> i32 {
    //     if s.len() == 0 {
    //         return 0;
    //     }

    //     let mut start: Option<usize> = None;
    //     let mut end: Option<usize> = None;

    //     for (index, char) in s.chars().rev().enumerate() {
    //         if char == ' ' {
    //             let is_end = if let Some(_) = end { true } else { false };
    //             let is_start = if let Some(_) = start { true } else { false };

    //             if is_end && is_start {
    //                 break;
    //             }
    //         }

    //         if char != ' ' {
    //             if let Some(_) = end {
    //                 if let Some(_) = start {
    //                 } else {
    //                     start = Some(index);
    //                 }
    //             } else {
    //                 end = Some(index);
    //             }
    //         }
    //         hello world
    //         println!("{index}: {char}")
    //     }

    //     0
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5,
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4,
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_last_word(String::from("luffy is still joyboy")),
            6,
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::length_of_last_word(String::from("")), 0)
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::length_of_last_word(String::from("         ")), 0)
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::length_of_last_word(String::from("qwerty")), 6)
    }
}
