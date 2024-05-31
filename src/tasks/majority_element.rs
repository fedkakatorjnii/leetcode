///
/// 169. Majority Element
///
/// Given an array nums of size n, return the majority element.
///
/// The majority element is the element that appears more than ⌊n / 2⌋ times.
/// You may assume that the majority element always exists in the array.
///
/// Follow-up: Could you solve the problem in linear time and in O(1) space?
///
use std::collections::HashMap;

use super::shared::Solution;

impl Solution {
    pub fn majority_element_1(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result_number = 0;
        let mut result_count = 0;

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for (&number, &count) in map.iter() {
            if result_count <= count {
                result_number = number;
                result_count = count;
            }
        }

        result_number
    }

    pub fn majority_element_2(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = nums.len() as i32;

        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;

            if *count > len / 2 {
                return num;
            }
        }

        0
    }

    pub fn majority_element_3(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = nums.len() as i32;
        let mut result_number = 0;

        for num in nums {
            let count = *map.get(&num).unwrap_or(&0) + 1;

            if count > len / 2 {
                result_number = num;
                break;
            }

            map.insert(num, count);
        }

        result_number
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;

        for num in nums {
            if count == 0 {
                max = num;
                count += 1;
            } else if num == max {
                count += 1;
            } else {
                count -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn majority_element_1() {
        let nums = vec![3, 2, 3];

        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn majority_element_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];

        assert_eq!(Solution::majority_element(nums), 2);
    }

    #[test]
    fn majority_element_3() {
        let nums = vec![1, 3, 1, 1, 3, 1, 2, 4];

        assert_eq!(Solution::majority_element(nums), 1);
    }
}
