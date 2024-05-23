///
/// 88. Merge Sorted Array
///
/// https://leetcode.com/problems/merge-sorted-array/description/
///
/// You are given two integer arrays nums1 and nums2,
/// sorted in non-decreasing order, and two integers m and n,
/// representing the number of elements in nums1 and nums2 respectively.
///
/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
///
/// The final sorted array should not be returned by the function,
/// but instead be stored inside the array nums1.
/// To accommodate this, nums1 has a length of m + n,
/// where the first m elements denote the elements that should be merged,
/// and the last n elements are set to 0 and should be ignored.
/// nums2 has a length of n.
///
/// Constraints:
/// - nums1.length == m + n;
/// - nums2.length == n;
/// - 0 <= m, n <= 200;
/// - 1 <= m + n <= 200;
/// - -10^9 <= nums1[i], nums2[j] <= 10^9;
///
/// Follow up: Can you come up with an algorithm that runs in O(m + n) time?

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut c1 = m;
        let mut c2 = n;

        while c1 + c2 > 0 {
            let index = c1 as usize + c2 as usize - 1;

            if c1 == 0 {
                nums1[index] = nums2[c2 as usize - 1];
                c2 -= 1;
                continue;
            }
            if c2 == 0 {
                nums1[index] = nums1[c1 as usize - 1];
                c1 -= 1;
                continue;
            }

            let i = c1 as usize - 1;
            let j = c2 as usize - 1;

            if nums1[i] > nums2[j] {
                nums1[index] = nums1[i];
                c1 -= 1;
            } else {
                nums1[index] = nums2[j];
                c2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_4() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![1, 2, 3];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_5() {
        let mut nums1 = vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0];
        let m = 5;
        let mut nums2 = vec![2, 4, 6, 8, 10];
        let n = 5;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
