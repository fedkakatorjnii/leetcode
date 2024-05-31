///
/// 66. Plus One
///
/// https://leetcode.com/problems/plus-one/description/
///
/// You are given a large integer represented as an integer array digits,
/// where each digits[i] is the ith digit of the integer.
/// The digits are ordered from most significant to least significant
/// in left-to-right order. The large integer does not contain any leading 0's.
///
/// Increment the large integer by one and return the resulting array of digits.
///
/// Constraints:
/// - 1 <= digits.length <= 100;
/// - 0 <= digits[i] <= 9;
/// - digits does not contain any leading 0's.
///
use super::shared::Solution;

impl Solution {
    pub fn plus_one_1(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        let len = result.len();

        for i in 0..len {
            let key = len - 1 - i;

            if result[key] == 9 {
                if key == 0 {
                    result[key] = 1;
                    result.push(0);
                } else {
                    result[key] = 0;
                }
            } else {
                result[key] += 1;
                break;
            }
        }

        result
    }

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        let len = result.len();

        for key in (0..len).rev() {
            if result[key] == 9 {
                if key == 0 {
                    result[key] = 1;
                    result.push(0);
                } else {
                    result[key] = 0;
                }
            } else {
                result[key] += 1;
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
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::plus_one(vec![1, 0, 9, 9, 9]), vec![1, 1, 0, 0, 0]);
    }

    #[test]
    #[ignore]
    fn test_() {
        assert_eq!(Solution::plus_one(vec![]), vec![]);
    }
}
