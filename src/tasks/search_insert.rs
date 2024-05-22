///
/// 35. Search Insert Position
///
/// https://leetcode.com/problems/search-insert-position/description/
///
/// Given a sorted array of distinct integers and a target value,
/// return the index if the target is found.
/// If not, return the index where it would be if it were inserted in order.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Constraints:
/// - 1 <= nums.length <= 10&4;
/// - 10^4 <= nums[i] <= 10^4;
/// - nums contains distinct values sorted in ascending order;
/// - -10^4 <= target <= 10^4.
///

struct Solution {}

impl Solution {
    // O(n)
    pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = nums.len() as i32;

        for i in 0..nums.len() {
            if nums[i] >= target {
                result = i as i32;
                break;
            }
        }

        result
    }

    //
    pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut result = (nums.len() - 1) as i32;

        while l <= r - 1 {
            let m = (l + r) / 2;
            let mid_value = nums[m];

            if mid_value == target {
                // искомое число найдено
                result = m as i32;
                break;
            }
            if mid_value < target {
                if (m + 1) == nums.len() {
                    result = nums.len() as i32;
                    break;
                }

                if nums[m + 1] > target {
                    result = (m + 1) as i32;
                    break;
                }

                l = m;
            }
            if mid_value > target {
                if (m - 1) < 0 {
                    result = 0;
                    break;
                }

                if nums[m - 1] < target {
                    result = m as i32;
                    break;
                }
                r = m;
            }
        }

        result
    }

    pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as _
    }

    pub fn search_insertr_4(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }

    pub fn search_insert_5(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&num| num < target) as i32
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search(&nums, target, 0, nums.len() - 1)
    }

    pub fn binary_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        if start >= end {
            if target > nums[end] {
                return (end + 1) as i32;
            }

            return end as i32;
        }

        let mid = (start + end) / 2;

        if target < nums[mid] {
            return Self::binary_search(nums, target, start, mid);
        }
        if target > nums[mid] {
            return Self::binary_search(nums, target, mid + 1, end);
        }

        return mid as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    // #[ignore]
    fn test_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    // #[ignore]
    fn test_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    // #[ignore]
    fn test_4() {
        assert_eq!(
            Solution::search_insert(vec![1, 2, 3, 4, 6, 7, 8, 9], 5),
            // Solution::search_insert(vec![0, 1], 5),
            4
        );
    }
}
