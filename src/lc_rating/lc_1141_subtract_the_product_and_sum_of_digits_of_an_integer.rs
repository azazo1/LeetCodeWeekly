// https://leetcode.cn/problems/subtract-the-product-and-sum-of-digits-of-an-integer/description/

impl Solution {
    pub fn first_trial(n: i32) -> i32 {
        let mut tmp = n;
        let mut product = 1;
        let mut sum = 0;
        while tmp > 0 {
            product *= tmp % 10;
            sum += tmp % 10;
            tmp /= 10;
        }
        product - sum
    }
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        Self::first_trial(n)
    }
}

struct Solution;