// https://leetcode.cn/problems/ant-on-the-boundary/description/

struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut cnt = 0;
        for i in nums {
            cur += i;
            if cur == 0 {
                cnt += 1;
            }
        }
        cnt
    }
}