///
/// 27. Remove Element
///
/// https://leetcode.com/problems/remove-element/description/
///
/// Given an integer array nums and an integer val,
/// remove all occurrences of val in nums in-place.
/// The order of the elements may be changed.
/// Then return the number of elements in nums which are not equal to val.
///
/// Consider the number of elements in nums which are not equal to val be k,
/// to get accepted, you need to do the following things:
/// - Change the array nums such that the first k elements of nums
///   contain the elements which are not equal to val.
///   The remaining elements of nums are not important as well
///   as the size of nums;
/// - Return k.
///
/// Custom Judge:
/// The judge will test your solution with the following code:
///   int[] nums = [...]; // Input array
///   int val = ...; // Value to remove
///   int[] expectedNums = [...]; // The expected answer with correct length.
///                               // It is sorted with no values equaling val.
///   
///   int k = removeElement(nums, val); // Calls your implementation
///   
///   assert k == expectedNums.length;
///   sort(nums, 0, k); // Sort the first k elements of nums
///   for (int i = 0; i < actualLength; i++) {
///       assert nums[i] == expectedNums[i];
///   }
///
/// If all assertions pass, then your solution will be accepted.
///

struct Solution {}

impl Solution {
    pub fn remove_element_1(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = nums.len() - 1;

        if nums.is_empty() {
            return 0;
        }

        loop {
            if nums[i] == val {
                nums.remove(i);
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        nums.len() as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&num| num != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let result_nums = vec![2, 2];

        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(nums, result_nums);
    }
}
