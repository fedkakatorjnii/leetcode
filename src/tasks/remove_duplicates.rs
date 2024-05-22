///
/// 26. Remove Duplicates from Sorted Array
///
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
///
/// Given an integer array nums sorted in non-decreasing order,
/// remove the duplicates in-place such that each unique element
/// appears only once. The relative order of the elements
/// should be kept the same. Then return the number of unique
/// elements in nums.
///
/// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
/// - Change the array nums such that the first k elements of nums
///   contain the unique elements in the order they were present
///   in nums initially. The remaining elements of nums are
///   not important as well as the size of nums;
/// - Return k.
///
/// Custom Judge:
///
/// The judge will test your solution with the following code:
///   int[] nums = [...]; // Input array
///   int[] expectedNums = [...]; // The expected answer with correct length
///   
///   int k = removeDuplicates(nums); // Calls your implementation
///   
///   assert k == expectedNums.length;
///   for (int i = 0; i < k; i++) {
///       assert nums[i] == expectedNums[i];
///   }
///
/// If all assertions pass, then your solution will be accepted.
///
/// Constraints:
/// - 1 <= nums.length <= 3 * 10^4;
/// - -100 <= nums[i] <= 100;
/// - nums is sorted in non-decreasing order.

struct Solution {}

impl Solution {
    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut last: Option<i32> = None;

        for num in nums {
            if let Some(value) = last {
                if *num != value {
                    count += 1;
                    last = Some(*num);
                }
            } else {
                count += 1;
                last = Some(*num);
            }
        }

        return count;
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut last: Option<i32> = None;
        let len = nums.len();

        // for num in nums {
        //     if let Some(value) = last {
        //         if *num != value {
        //             count += 1;
        //             last = Some(*num);
        //         }
        //     } else {
        //         count += 1;
        //         last = Some(*num);
        //     }
        // }

        if len < 2 {
            return len as i32;
        }

        // let qwe = nums.iter().filter_map(|x| {
        //     //
        //     None
        // });
        // nums.iter().map(|item| {
        //     //
        // })

        // println!("!!!");
        // for i in 1..len {
        //     let prev = nums[i - 1];
        //     let current = nums[i];
        //     if prev == current {
        //         nums.swap_remove(i);
        //     }

        //     println!("{i}");
        // }
        // println!("!!!");

        nums.len() as i32
        // return count;
    }

    pub fn remove_duplicates_3(nums: &mut Vec<i32>) -> i32 {
        let mut last: Option<i32> = None;
        let mut i = 0;

        while i < nums.len() {
            let current = nums[i];

            if let Some(last_value) = last {
                if current == last_value {
                    nums.remove(i);
                } else {
                    last = Some(current);
                    i += 1;
                }
            } else {
                last = Some(current);
                i += 1;
            }
        }

        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = nums.len() - 1;

        while i > 0 {
            if nums[i] == nums[i - 1] {
                nums.remove(i);
            }
            i -= 1;
        }

        nums.len() as i32
    }

    pub fn remove_duplicates_5(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        let result_nums = vec![1, 2];

        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, result_nums);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result_nums = vec![0, 1, 2, 3, 4];

        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, result_nums);
    }
}
