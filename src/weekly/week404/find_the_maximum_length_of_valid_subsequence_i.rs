// https://leetcode.cn/contest/weekly-contest-404/problems/find-the-maximum-length-of-valid-subsequence-i/

use std::cmp::max;

impl Solution {
    fn trial(nums: &Vec<i32>, even: bool) -> i32 {
        let mut max_len = 0;
        for start in 0..nums.len() {
            let mut sub: Vec<i32> = vec![nums[start]];
            for i in (start + 1)..nums.len() {
                let i1 = nums[i];
                if ((sub.last().unwrap() + i1) % 2 == 0) == even {
                    sub.push(i1);
                }
            }
            max_len = max(max_len, sub.len())
        }
        max_len as i32
    }
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        max(Self::trial(&nums, true), Self::trial(&nums, false))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::weekly::week404::find_the_maximum_length_of_valid_subsequence_i::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
    }
}