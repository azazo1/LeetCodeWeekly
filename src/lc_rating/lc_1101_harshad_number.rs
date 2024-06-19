// https://leetcode.cn/problems/harshad-number/description/

struct Solution {}

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut xi = x;
        let mut sum = 0;
        while xi > 0 {
            sum += xi % 10;
            xi /= 10;
        }
        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}