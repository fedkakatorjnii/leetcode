///
/// 80. Remove Duplicates from Sorted Array II
///
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii
///
/// Given an integer array nums sorted in non-decreasing order,
/// remove some duplicates in-place such that each unique element
/// appears at most twice.
/// The relative order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array in some languages,
/// you must instead have the result be placed in the first part
/// of the array nums.
/// More formally, if there are k elements after removing the duplicates,
/// then the first k elements of nums should hold the final result.
/// It does not matter what you leave beyond the first k elements.
///
/// Return k after placing the final result in the first k slots of nums.
///
/// Do not allocate extra space for another array.
/// You must do this by modifying the input array in-place
/// with O(1) extra memory.
///
/// Custom Judge:
/// The judge will test your solution with the following code:
///
/// int[] nums = [...]; // Input array
/// int[] expectedNums = [...]; // The expected answer with correct length
///
/// int k = removeDuplicates(nums); // Calls your implementation
///
/// assert k == expectedNums.length;
/// for (int i = 0; i < k; i++) {
///     assert nums[i] == expectedNums[i];
/// }
///
/// If all assertions pass, then your solution will be accepted.
///
/// Constraints:
/// - 1 <= nums.length <= 3 * 10^4;
/// - -104 <= nums[i] <= 10^4;
/// - nums is sorted in non-decreasing order.
///

struct Solution {}

impl Solution {
    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        println!("//");
        println!("//// PRE {:?}", nums);

        let mut len = nums.len();

        if len < 2 {
            return len as i32;
        }

        let mut i = 1;
        let mut j = 1;
        let mut prev = nums[j];

        while i < len {
            let current = nums[i];

            if prev == current {
                i += 1;
                //
            } else {
                nums[j] = current;
                prev = current;
                j += 1;
                i += 1;
            }
        }
        // for i in len..j {
        //     nums.remove(i - 1);
        // }

        println!("//// POST {:?}", nums);
        // println!("//// {j}");
        println!("////");
        for i in len..j - 1 {
            println!("////// {i}");
        }
        while j != nums.len() {
            nums.remove(nums.len() - 1);
        }

        println!("////");
        println!("//");

        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();

        if len < 3 {
            return len as i32;
        }

        let mut i = 1;
        let mut j = 1;
        let mut prev = nums[0];
        let mut count = 1;

        while i < len {
            let current = nums[i];

            if prev == current {
                count += 1;

                if count == 2 {
                    nums[j] = current;
                    j += 1;
                }
            } else {
                nums[j] = current;
                prev = current;
                count = 1;
                j += 1;
            }

            i += 1;
        }

        while j != nums.len() {
            nums.remove(nums.len() - 1);
        }

        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let result = 5;

        //nums = [1,1,2,2,3,_]
        //Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
        //It does not matter what you leave beyond the returned k (hence they are underscores).
        assert_eq!(Solution::remove_duplicates(&mut nums), result);
    }

    #[test]
    fn remove_duplicates_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let result = 7;

        // nums = [0,0,1,1,2,3,3,_,_]
        // Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
        // It does not matter what you leave beyond the returned k (hence they are underscores).
        assert_eq!(Solution::remove_duplicates(&mut nums), result);
    }

    #[test]
    fn remove_duplicates_3() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let result = 7;

        assert_eq!(Solution::remove_duplicates(&mut nums), result);
    }

    #[test]
    fn remove_duplicates_4() {
        let mut nums = vec![1, 2, 2];
        let result = 3;

        assert_eq!(Solution::remove_duplicates(&mut nums), result);
    }
}
