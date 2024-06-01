///
/// 205. Isomorphic Strings
///
/// Given two strings s and t, determine if they are isomorphic.
///
/// Two strings s and t are isomorphic if the characters in s
/// can be replaced to get t.
///
/// All occurrences of a character must be replaced with another character
/// while preserving the order of characters.
/// No two characters may map to the same character,
/// but a character may map to itself.
///
/// Constraints:
/// - 1 <= s.length <= 5 * 104;
/// - t.length == s.length;
/// - s and t consist of any valid ascii character.
///
use std::collections::{HashMap, HashSet};

use super::shared::Solution;

impl Solution {
    pub fn print_map(map: &HashMap<char, i32>) {
        for (char, count) in map {
            println!("{char}:{count}");
        }
    }

    pub fn fill_in_map(map: &mut HashMap<char, i32>, s: &str) {
        for char in s.chars() {
            // let count = s_map.get(char).unwrap_or(0);
            // if let Some(currnet) = s_map.get_mut(char) {}
            let count = *map.get(&char).unwrap_or(&0);

            // *s_map.get(&char).unwrap_or(&0) + 1;
        }
    }

    pub fn is_isomorphic_hard(s: String, t: String) -> bool {
        let mut result_map: HashMap<i32, i32> = HashMap::new();
        let mut s_map: HashMap<char, i32> = HashMap::new();
        // let mut s_result_map: HashMap<i32, i32> = HashMap::new();

        let mut t_map: HashMap<char, i32> = HashMap::new();
        // let mut t_result_map: HashMap<i32, i32> = HashMap::new();

        // result_map.insert(1, 2);
        // match result_map.get_mut(&1) {
        //     Some(value) => {
        //         *value -= 1;
        //     }
        //     None => {
        //         //
        //     }
        // }

        // Self::fill_in_map(&s_map, &s);
        // Self::fill_in_map(&t_map, &t);
        for char in s.chars() {
            let old_count = *s_map.get(&char).unwrap_or(&0);
            let new_count = old_count + 1;

            if old_count != 0 {
                if let Some(value) = result_map.get_mut(&old_count) {
                    *value -= 1;
                }
            }
            if let Some(value) = result_map.get_mut(&new_count) {
                *value += 1;
            } else {
                result_map.insert(new_count, 1);
            }

            // match result_map.get_mut(&old_count) {
            //     Some(value) => {
            //         *value -= 1;
            //     }
            //     _ => (),
            // }
            // match result_map.get_mut(&new_count) {
            //     Some(value) => {
            //         *value -= 1;
            //     }
            //     _ => {
            //         result_map.insert(new_count, 1);
            //     }
            // }

            s_map.insert(char, new_count);
            // s_map.insert(char, *s_map.get(&char).unwrap_or(&0) + 1);
            // *s_map.get(&char).unwrap_or(&0) + 1;
        }
        for char in t.chars() {
            let old_count = *t_map.get(&char).unwrap_or(&0);
            let new_count = old_count + 1;

            if old_count != 0 {
                if let Some(value) = result_map.get_mut(&old_count) {
                    *value += 1;
                }
            }
            if let Some(value) = result_map.get_mut(&new_count) {
                *value -= 1;
            } else {
                result_map.insert(new_count, -1);
            }

            t_map.insert(char, new_count);
            // t_map.insert(char, *t_map.get(&char).unwrap_or(&0) + 1);
            // *s_map.get(&char).unwrap_or(&0) + 1;
        }

        // s_map.insert('q', 1);

        println!("/// s ///");
        for (char, count) in &s_map {
            println!("{char}:{count}");
        }
        println!("/// t ///");
        for (char, count) in &t_map {
            println!("{char}:{count}");
        }
        println!("/// result_map ///");
        for (char, count) in &result_map {
            println!("{char}:{count}");
        }

        Self::print_map(&s_map);
        Self::print_map(&t_map);

        return false;
    }

    pub fn is_isomorphic_dif(s: String, t: String) -> bool {
        let mut result_map: HashMap<i32, i32> = HashMap::new();
        let mut s_map: HashMap<char, i32> = HashMap::new();

        let mut t_map: HashMap<char, i32> = HashMap::new();

        for char in s.chars() {
            let old_count = *s_map.get(&char).unwrap_or(&0);
            let new_count = old_count + 1;

            s_map.insert(char, new_count);
        }
        for char in t.chars() {
            let old_count = *t_map.get(&char).unwrap_or(&0);
            let new_count = old_count + 1;

            t_map.insert(char, new_count);
        }

        for (char, count) in &s_map {
            result_map.insert(*count, *result_map.get(&count).unwrap_or(&0) + 1);
        }
        for (char, count) in &t_map {
            if let Some(value) = result_map.get_mut(&count) {
                let new_value = *value - 1;
                if new_value < 1 {
                    result_map.remove(count);
                } else {
                    result_map.insert(*count, new_value);
                }
            } else {
                return false;
            }
        }

        return true;
    }

    pub fn is_isomorphic_1(s: String, t: String) -> bool {
        let mut result: HashSet<String> = HashSet::new();
        let mut s_map: HashMap<char, String> = HashMap::new();
        let mut t_map: HashMap<char, String> = HashMap::new();

        for (key, char) in s.chars().enumerate() {
            if let Some(value) = s_map.get_mut(&char) {
                *value = format!("{}{}", *value, key);
            } else {
                s_map.insert(char, format!("{key}"));
            }
        }
        for (key, char) in t.chars().enumerate() {
            if let Some(value) = t_map.get_mut(&char) {
                *value = format!("{}{}", *value, key);
            } else {
                t_map.insert(char, format!("{key}"));
            }
        }
        for (_, positions) in &s_map {
            result.insert(positions.clone());
        }
        for (_, positions) in &t_map {
            let current = result.get(positions);

            if current.is_some() {
                result.remove(&positions.clone());
            } else {
                return false;
            }
        }

        result.len() == 0
    }

    pub fn insert(map: &mut HashMap<char, String>, ch: char, key: usize) {
        if let Some(value) = map.get_mut(&ch) {
            *value = format!("{}{}", *value, key);
        } else {
            map.insert(ch, format!("{key}"));
        }
    }

    pub fn is_isomorphic_2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        let mut s_map: HashMap<char, String> = HashMap::new();
        let mut t_map: HashMap<char, String> = HashMap::new();

        for key in 0..s.len() {
            let s_char = s_chars.next().unwrap();
            let t_char = t_chars.next().unwrap();

            let s_char_fround = match s_map.get(&s_char) {
                Some(value) => value,
                None => "",
            };
            let t_char_fround = match t_map.get(&t_char) {
                Some(value) => value,
                None => "",
            };

            if *s_char_fround != *t_char_fround {
                return false;
            }

            if let Some(value) = s_map.get_mut(&s_char) {
                *value = format!("{}{}", *value, key);
            } else {
                s_map.insert(s_char, format!("{key}"));
            }
            if let Some(value) = t_map.get_mut(&t_char) {
                *value = format!("{}{}", *value, key);
            } else {
                t_map.insert(t_char, format!("{key}"));
            }
        }

        return true;
    }

    pub fn is_isomorphic_3(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        let mut map: HashMap<char, char> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();

        for _ in 0..s.len() {
            let s_char = s_chars.next().unwrap();
            let t_char = t_chars.next().unwrap();

            if let Some(value) = map.get(&s_char) {
                if *value != t_char {
                    return false;
                }
            } else {
                if set.get(&t_char).is_some() {
                    return false;
                }

                map.insert(s_char, t_char);
                set.insert(t_char);
            }
        }

        true
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, char> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();

        for (key, s_char) in s.chars().enumerate() {
            let t_char = t.chars().skip(key).next().unwrap();

            if let Some(value) = map.get(&s_char) {
                if *value != t_char {
                    return false;
                }
            } else {
                if set.get(&t_char).is_some() {
                    return false;
                }

                map.insert(s_char, t_char);
                set.insert(t_char);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_isomorphic_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let result = true;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }

    #[test]
    fn is_isomorphic_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let result = false;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }

    #[test]
    fn is_isomorphic_3() {
        let s = "aaba".to_string();
        let t = "babb".to_string();
        let result = false;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }

    #[test]
    fn is_isomorphic_4() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        let result = false;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }
}
