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

    pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&num| num != val);
        nums.len() as i32
    }

    pub fn remove_element_3(nums: &mut Vec<i32>, val: i32) -> i32 {
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

    pub fn get_last_different(nums: &Vec<i32>, val: i32) -> Option<(usize, i32)> {
        let mut result: Option<(usize, i32)> = None;

        for (i, num) in nums.iter().rev().enumerate() {
            if *num != val {
                result = Some((nums.len() - i - 1, *num));
                break;
            }
        }

        result
    }

    pub fn remove_element_4(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = nums.len() as i32;

        if nums.is_empty() {
            return 0;
        }

        for i in 0..nums.len() {
            if nums[i] != val {
                count = i as i32 + 1;
                continue;
            }

            let tmp = Self::get_last_different(nums, val);

            if let Some((j, last_num)) = tmp {
                if i >= j {
                    break;
                }

                count = i as i32 + 1;
                nums[i] = last_num;
                nums[j] = val;
            } else {
                count = i as i32;
                break;
            }
        }

        count
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[index] = nums[i];
                index += 1;
            }
        }

        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        // let result_nums = vec![2, 2, 3, 3];

        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        // assert_eq!(nums, result_nums);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        // let result_nums = vec![0, 1, 4, 0, 3, 2, 2, 2];

        assert_eq!(Solution::remove_element(&mut nums, val), 5);
        // assert_eq!(nums, result_nums);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![3, 3];
        let val = 3;
        // let result_nums = vec![];

        assert_eq!(Solution::remove_element(&mut nums, val), 0);
        // assert_eq!(nums, result_nums);
    }
    #[test]
    fn test_4() {
        let mut nums = vec![4, 5];
        let val = 4;
        // let result_nums = vec![];

        assert_eq!(Solution::remove_element(&mut nums, val), 1);
        // assert_eq!(nums, result_nums);
    }
}
