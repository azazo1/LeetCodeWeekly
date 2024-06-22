// https://leetcode.cn/problems/concatenation-of-array/description/

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let mut rst = Vec::with_capacity(nums.len());
        rst.clone_from(&nums);
        rst.append(&mut nums);
        rst
    }
}

struct Solution;