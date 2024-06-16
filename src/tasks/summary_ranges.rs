///
/// 228. Summary Ranges
///
/// https://leetcode.com/problems/summary-ranges/description
///
/// You are given a sorted unique integer array nums.
///
/// A range [a,b] is the set of all integers from a to b (inclusive).
///
/// Return the smallest sorted list of ranges that cover all the numbers
/// in the array exactly.
/// That is, each element of nums is covered by exactly one of the ranges,
/// and there is no integer x such that x is in one of the ranges
/// but not in nums.
///
/// Each range [a,b] in the list should be output as:
/// - "a->b" if a != b;
/// - "a" if a == b.
///
/// Constraints:
/// - 0 <= nums.length <= 20;
/// - -2^31 <= nums[i] <= 2^31 - 1;
/// - All the values of nums are unique;
/// - nums is sorted in ascending order.
///

struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut current: Option<(i32, i32, usize)> = None;

        for (key, &num) in nums.iter().enumerate() {
            if let Some((start_value, end_value, start_index)) = current {
                if (end_value + 1) < num {
                    result.push(format!("{num}"));

                    current = Some((num, num, key));
                } else {
                    *result.last_mut().unwrap() = format!("{start_value}->{num}").to_string();
                    current = Some((start_value, num, start_index));
                }
            } else {
                current = Some((num, num, key));
                result.push(format!("{num}"));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn summary_ranges_1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = vec!["0->2", "4->5", "7"];

        // Explanation: The ranges are:
        // [0,2] --> "0->2"
        // [4,5] --> "4->5"
        // [7,7] --> "7"
        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn summary_ranges_2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = vec!["0", "2->4", "6", "8->9"];

        // Explanation: The ranges are:
        // [0,0] --> "0"
        // [2,4] --> "2->4"
        // [6,6] --> "6"
        // [8,9] --> "8->9"
        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn summary_ranges_3() {
        let nums = vec![-2147483648, -2147483647, 2147483647];
        let result = vec!["-2147483648->-2147483647", "2147483647"];

        assert_eq!(Solution::summary_ranges(nums), result);
    }
}
