///
/// 1. Two Sum
///
/// https://leetcode.com/problems/two-sum/description/
///
/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution,
/// and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Constraints:
/// - 2 <= nums.length <= 10^4;
/// - -10^9 <= nums[i] <= 10^9;
/// - -10^9 <= nums[i] <= 10^9;
/// - -10^9 <= target <= 10^9;
/// - Only one valid answer exists.
///
/// Follow-up: Can you come up with an algorithm that is less
/// than O(n2) time complexity?
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut checked_elements: HashMap<i32, i32> = HashMap::new();

        for (key, num) in nums.iter().enumerate() {
            let rest = target - num;

            if checked_elements.contains_key(num) {
                return vec![checked_elements[num], key as i32];
            }

            checked_elements.insert(rest, key as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
