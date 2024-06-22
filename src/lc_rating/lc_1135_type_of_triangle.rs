// https://leetcode.cn/problems/type-of-triangle/description/

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] <= nums[2]
            || nums[1] + nums[2] <= nums[0]
            || nums[0] + nums[2] <= nums[1] {
            "none"
        } else if nums[0] == nums[1] && nums[1] == nums[2] {
            "equilateral"
        } else if nums[0] == nums[1] || nums[1] == nums[2] || nums[2] == nums[0] {
            "isosceles"
        } else {
            "scalene"
        }.to_string()
    }
}

struct Solution;