// https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three/description/

impl Solution {
    pub fn first_trial(nums: Vec<i32>) -> i32 {
        let nums:Vec<_> = nums.iter().filter(|x| **x % 6 == 0).collect();
        if nums.len() == 0 {
            0
        } else {
            nums.iter().cloned()
                .sum::<i32>() / nums.len() as i32
        }
    }
    pub fn average_value(nums: Vec<i32>) -> i32 {
        Self::first_trial(nums)
    }
}

struct Solution;