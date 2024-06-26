// https://leetcode.cn/problems/maximum-product-difference-between-two-pairs/description/

impl Solution {
    pub fn greedy_trial(nums: Vec<i32>) -> i32 {
        let mut pri_max = i32::MIN;
        let mut sec_max = i32::MIN;
        let mut pri_min = i32::MAX;
        let mut sec_min = i32::MAX;
        for i in nums {
            if i > pri_max {
                sec_max = pri_max;
                pri_max = i;
            } else if i > sec_max {
                sec_max = i;
            }
            if i < pri_min {
                sec_min = pri_min;
                pri_min = i;
            } else if i < sec_min {
                sec_min = i;
            }
        }
        pri_max * sec_max - pri_min * sec_min
    }
    pub fn first_trial(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        nums[len - 1] * nums[len - 2] - nums[0] * nums[1]
    }
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        Self::greedy_trial(nums)
    }
}
struct Solution;