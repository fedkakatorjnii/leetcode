///
/// 290. Word Pattern
///
/// Given a pattern and a string s, find if s follows the same pattern.
///
/// Here follow means a full match, such that there is a bijection
/// between a letter in pattern and a non-empty word in s.
///
/// Constraints:
/// - 1 <= pattern.length <= 300;
/// - pattern contains only lower-case English letters;
/// - 1 <= s.length <= 3000;
/// - s contains only lowercase English letters and spaces ' ';
/// - s does not contain any leading or trailing spaces;
/// - All the words in s are separated by a single space.
///
use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

use super::shared::Solution;

impl Solution {
    pub fn get_next_word(chars: &mut Chars) -> String {
        let mut current = "".to_string();
        while let Some(value) = chars.next() {
            if value == ' ' {
                if current.len() != 0 {
                    break;
                }
            } else {
                current = format!("{current}{value}");
            }
        }

        current
    }

    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map: HashMap<char, String> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();
        let mut chars = s.chars();

        for char in pattern.chars() {
            if let Some(word) = map.get(&char) {
                let current = Self::get_next_word(&mut chars);

                if *word != current {
                    return false;
                }
            } else {
                let current = Self::get_next_word(&mut chars);

                if set.get(&current).is_some() || current.len() == 0 {
                    return false;
                }

                map.insert(char, current.clone());
                set.insert(current.to_owned());
            }
        }

        if chars.next().is_some() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_pattern_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let result = true;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn word_pattern_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn word_pattern_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn word_pattern_4() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn word_pattern_5() {
        let pattern = "aa".to_string();
        let s = "dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn word_pattern_6() {
        let pattern = "ab".to_string();
        let s = "dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }
}
