// https://leetcode.cn/problems/shuffle-the-array/

struct Solution {}

impl Solution {
    pub fn first_trial(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut rst = Vec::with_capacity(nums.len());
        let zip = nums[0..n as usize].iter().zip(nums[(n as usize)..(2 * n as usize)].iter());
        zip.for_each(|(x, y)| {
            rst.push(*x);
            rst.push(*y);
        });
        rst
    }
    pub fn second_trial(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut rst = Vec::with_capacity(nums.len());
        for i in 0..n {
            rst.push(nums[i as usize]);
            rst.push(nums[(i + n) as usize]);
        }
        rst
    }

    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        Self::second_trial(nums, n)
    }
}