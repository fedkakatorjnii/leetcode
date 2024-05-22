///
/// 20. Valid Parentheses
///
/// https://leetcode.com/problems/valid-parentheses/description/
///
/// Given a string s containing just the characters
/// '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
///
/// An input string is valid if:
/// 1. Open brackets must be closed by the same type of brackets;
/// 2. Open brackets must be closed in the correct order;
/// 3. Every close bracket has a corresponding open bracket of the same type.
///
/// Constraints:
// - 1 <= s.length <= 10^4;
// - s consists of parentheses only '()[]{}'.
//

struct Solution {}

impl Solution {
    pub fn open_bracket(count: i32) -> i32 {
        count + 1
    }

    // pub fn close_bracket(mut current_brackets: Vec<char>, count: i32, bracket: char) -> i32 {
    pub fn close_bracket(last_bracket: Option<char>, count: i32, bracket: char) -> i32 {
        let mut result = count;

        if let Some(value) = last_bracket {
            if value == bracket {
                result -= 1;
            } else {
                result = -1;
            }
        } else {
            result = -1;
        }

        result
    }

    pub fn is_valid(s: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        let brackets = s.chars().collect::<Vec<char>>();

        let mut current_brackets: Vec<char> = vec![];

        for bracket in brackets {
            match bracket {
                '(' => {
                    current_brackets.push('(');
                    x += 1;
                }
                ')' => x = Solution::close_bracket(current_brackets.pop(), x, '('),
                '[' => {
                    current_brackets.push('[');
                    y += 1;
                }
                ']' => y = Solution::close_bracket(current_brackets.pop(), y, '['),
                '{' => {
                    current_brackets.push('{');
                    z += 1;
                }
                '}' => z = Solution::close_bracket(current_brackets.pop(), z, '{'),
                _ => (),
            };

            if x < 0 || y < 0 || z < 0 {
                return false;
            }
        }

        x == 0 && y == 0 && z == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    // #[ignore]
    fn test_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    // #[ignore]
    fn test_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }

    #[test]
    // #[ignore]
    fn test_4() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    // #[ignore]
    fn test_5() {
        assert_eq!(Solution::is_valid(String::from(")(")), false);
    }

    #[test]
    // #[ignore]
    fn test_6() {
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }

    #[test]
    // #[ignore]
    fn test_7() {
        assert_eq!(Solution::is_valid(String::from("")), true);
    }
}
