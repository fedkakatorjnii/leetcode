///
/// 14. Longest Common Prefix
///
/// https://leetcode.com/problems/longest-common-prefix/description/
///
/// Write a function to find the longest common prefix string
/// amongst an array of strings.
/// If there is no common prefix, return an empty string "".
///
/// Constraints:
/// - 1 <= strs.length <= 200;
/// - 0 <= strs[i].length <= 200;
/// - strs[i] consists of only lowercase English letters.
///
use super::shared::Solution;

impl Solution {
    pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
        let mut len: Option<usize> = None;
        for item in &strs {
            let current_len = item.len();

            if let Some(target_len) = len {
                if current_len < target_len {
                    len = Some(current_len);
                }
            } else {
                len = Some(current_len);
            }
        }

        let len = len.unwrap_or(0);

        let mut result: Vec<char> = vec![];
        for key in 0..len {
            println!("key: {key}");
            let mut target: Option<char> = None;

            for item in &strs {
                let ch = item.chars().skip(key).next();
                if ch.is_none() {
                    target = None;
                    break;
                }
                let ch = ch.unwrap();

                if let Some(char) = target {
                    if char != ch {
                        target = None;
                        break;
                    }
                } else {
                    target = Some(ch);
                }
            }
            if target.is_none() {
                break;
            }
            result.push(target.unwrap());
        }

        result.into_iter().collect()
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::from("");
        let mut key = 0;

        loop {
            let mut target: Option<char> = None;

            for item in &strs {
                if let Some(ch) = item.chars().skip(key).next() {
                    if let Some(char) = target {
                        if char != ch {
                            target = None;
                            break;
                        }
                    } else {
                        target = Some(ch);
                    }
                } else {
                    target = None;
                    break;
                }
            }

            if let Some(value) = target {
                result.push(value);
                key += 1;
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ],),
            String::from("fl"),
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ],),
            String::from(""),
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_common_prefix(vec![String::from(""), String::from("b"),],),
            String::from(""),
        );
    }
}
