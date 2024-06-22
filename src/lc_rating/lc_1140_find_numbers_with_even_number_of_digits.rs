// https://leetcode.cn/problems/find-numbers-with-even-number-of-digits/description/

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut rst = 0;
        for i in nums {
            if (10 <= i && i < 100)
                || (1000 <= i && i < 10000)
                || (100000 <= i) {
                rst += 1;
            }
        }
        rst
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1140_find_numbers_with_even_number_of_digits::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_numbers(vec![12, 123, 2, 5, 1234]), 2);
    }
}