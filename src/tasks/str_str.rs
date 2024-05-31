///
/// 28. Find the Index of the First Occurrence in a String
///
/// Given two strings needle and haystack, return the index
/// of the first occurrence of needle in haystack,
/// or -1 if needle is not part of haystack.
///
use super::shared::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if needle_len > haystack_len {
            return -1;
        }

        let count = haystack_len - needle_len + 1;
        let mut result = -1;

        for i in 0..count {
            let current = &haystack[i..(needle_len + i)];
            if current.to_string() == needle {
                result = i as i32;
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
    fn str_str_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let result = 0;

        assert_eq!(Solution::str_str(haystack, needle), result);
    }

    #[test]
    fn str_str_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let result = -1;

        assert_eq!(Solution::str_str(haystack, needle), result);
    }

    #[test]
    fn str_str_3() {
        let haystack = "leet".to_string();
        let needle = "leetcode".to_string();
        let result = -1;

        assert_eq!(Solution::str_str(haystack, needle), result);
    }
}
