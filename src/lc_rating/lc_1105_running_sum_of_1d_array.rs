// https://leetcode.cn/problems/running-sum-of-1d-array/description/

struct Solution {}

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut current_sum = 0;
        for i in 0..nums.len() {
            current_sum += nums[i];
            nums[i] = current_sum;
        }
        nums
    }
}