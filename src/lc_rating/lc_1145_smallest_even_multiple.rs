// https://leetcode.cn/problems/smallest-even-multiple/description/


impl Solution {
    fn gcd(mut a:i32, mut b: i32 ) -> i32 {
        while b > 0 {
            let tmp = a;
            a = b;
            b = tmp % b;
        }
        a
    }
    pub fn first_trial(n: i32) -> i32 {
        2 * n / Self::gcd(2, n)
    }
    pub fn smallest_even_multiple(n: i32) -> i32 {
        Self::first_trial(n)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1145_smallest_even_multiple::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::first_trial(5), 10)
    }
}