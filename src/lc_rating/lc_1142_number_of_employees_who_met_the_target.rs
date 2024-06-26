// https://leetcode.cn/problems/number-of-employees-who-met-the-target/

impl Solution {
    pub fn first_trial(hours: Vec<i32>, target: i32) -> i32 {
        let mut rst = 0;
        for i in hours {
            if i >= target {
                rst += 1;
            }
        }
        rst
    }
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        Self::first_trial(hours, target)
    }
}

struct Solution;