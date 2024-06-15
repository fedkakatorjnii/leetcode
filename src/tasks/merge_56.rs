///
/// 56. Merge Intervals
///
/// https://leetcode.com/problems/merge-intervals/description
///
/// Given an array of intervals where intervals[i] = [starti, endi],
/// merge all overlapping intervals, and return an array of
/// the non-overlapping intervals that cover all the intervals in the input.
///
/// Constraints:
/// - 1 <= intervals.length <= 10^4;
/// - intervals[i].length == 2;
/// - 0 <= starti <= endi <= 10^4.
///

pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        intervals.sort_unstable_by_key(|item| item[0]);

        let mut result: Vec<Vec<i32>> = vec![];

        for item in intervals {
            let current_start = item[0];
            let current_end = item[1];

            if let Some(last_range) = result.last_mut() {
                let last_end = last_range[1];
                if current_start > last_end && current_end > last_end {
                    result.push(vec![current_start, current_end]);
                } else {
                    if current_start <= last_end {
                        if current_end > last_end {
                            last_range[1] = current_end;
                        }
                    }
                }
            } else {
                result.push(vec![current_start, current_end]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        // Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
        assert_eq!(Solution::merge(intervals), result);
    }

    #[test]
    fn merge_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = vec![vec![1, 5]];

        // Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
        assert_eq!(Solution::merge(intervals), result);
    }

    #[test]
    fn merge_3() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let result = vec![vec![0, 4]];

        assert_eq!(Solution::merge(intervals), result);
    }
}
