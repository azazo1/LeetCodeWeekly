// https://leetcode.cn/problems/neither-minimum-nor-maximum/description/

impl Solution {
    pub fn o1_trial(nums: Vec<i32>) -> i32 {
        if 2 >= nums.len() {
            return -1;
        }
        let mut nums: Vec<_> = nums[0..3]
            .iter()
            .cloned()
            .collect();
        nums.sort();
        nums[1]
    }
    pub fn first_trial(nums: Vec<i32>) -> i32 { // error
        let mut min = nums[0];
        let mut max = nums[0];
        let mut other = -1;
        for i in &nums[1..] {
            let i = *i;
            if i < min {
                if max != min {
                    other = min;
                }
                min = i;
            } else if i > max {
                if max != min {
                    other = max;
                }
                max = i;
            } else {
                other = i;
            }
        }
        if other == max || other == min {
            -1
        } else {
            other
        }
    }

    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        Self::o1_trial(nums)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1148_neither_minimum_nor_maximum::Solution;

    #[test]
    fn test() {
        println!("{}", Solution::find_non_min_or_max(vec![1, 2, 3, 4]));
    }
}