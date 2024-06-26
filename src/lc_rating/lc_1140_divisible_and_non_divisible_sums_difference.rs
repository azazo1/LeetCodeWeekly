// https://leetcode.cn/problems/divisible-and-non-divisible-sums-difference/description/

// 数学公式解: https://leetcode.cn/problems/divisible-and-non-divisible-sums-difference/solutions/2472130/o1-shu-xue-gong-shi-yi-xing-gao-ding-pyt-m5cq/
impl Solution {
    pub fn math_solution(n: i32, m: i32) -> i32 {
        (1 + n) * n / 2 - (n / m + 1) * (n / m) * m
    }
    pub fn plain_solution(n: i32, m: i32) -> i32 {
        let mut num1 = 0;
        let mut num2 = 0;
        for i in 1..=n {
            if i % m != 0 {
                num1 += i;
            } else {
                num2 += i;
            }
        }
        num1 - num2
    }
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        Self::math_solution(n, m)
    }
}

struct Solution;