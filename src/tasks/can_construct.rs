///
/// 383. Ransom Note
///
/// Given two strings ransomNote and magazine, return true
/// if ransomNote can be constructed by using the letters from magazine
/// and false otherwise.
///
/// Each letter in magazine can only be used once in ransomNote.
///
/// Constraints:
///  - 1 <= ransomNote.length, magazine.length <= 105;
///  - ransomNote and magazine consist of lowercase English letters.
///
use super::shared::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut chars_map: HashMap<char, i32> = HashMap::new();

        for char in magazine.chars() {
            *chars_map.entry(char).or_insert(0) += 1;
            // chars_map.insert(
            //     char,
            //     if let Some(value) = chars_map.get(&char) {
            //         *value + 1
            //     } else {
            //         0
            //     },
            // );
        }
        for char in ransom_note.chars() {
            let count = chars_map.get_mut(&char);

            if let Some(value) = count {
                if *value > 0 {
                    *value -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        let result = false;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }

    #[test]
    fn can_construct_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        let result = false;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }

    #[test]
    fn can_construct_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        let result = true;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }
}
