// https://leetcode.cn/contest/weekly-contest-404/problems/find-the-maximum-length-of-valid-subsequence-ii/

use std::cmp::max;

impl Solution {
    fn trial(nums: &Vec<i32>, rem: i32, k:i32) -> i32 {
        let mut max_len = 0;
        for start in 0..nums.len() {
            let mut sub: Vec<i32> = vec![nums[start]];
            for i in (start + 1)..nums.len() {
                let i1 = nums[i];
                if ((sub.last().unwrap() + i1) % k) == rem {
                    sub.push(i1);
                }
            }
            max_len = max(max_len, sub.len())
        }
        max_len as i32
    }
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut maximum = 0;
        for i in 0..k {
            maximum = max(Self::trial(&nums, i, k), maximum);
        }
        maximum
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::weekly::week404::find_the_maximum_length_of_valid_subsequence_ii::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_length(vec![1, 7, 9], 10), 2);
    }
}