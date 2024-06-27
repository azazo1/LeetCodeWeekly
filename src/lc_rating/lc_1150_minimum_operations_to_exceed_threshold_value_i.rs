// https://leetcode.cn/problems/minimum-operations-to-exceed-threshold-value-i/description/

impl Solution {
    pub fn first_trial(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        for i in nums {
            if i < k {
                cnt += 1;
            }
        }
        cnt
    }
    pub fn iter_trial(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().filter(|x| **x < k).count() as i32
    }
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        Self::iter_trial(nums, k)
    }
}

struct Solution;