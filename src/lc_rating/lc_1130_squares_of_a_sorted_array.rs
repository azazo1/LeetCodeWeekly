// https://leetcode.cn/problems/squares-of-a-sorted-array/description/

impl Solution {
    fn double_ptr_trial_2nd(nums: Vec<i32>) -> Vec<i32> {
        let mut left_ptr = 0;
        let mut right_ptr = nums.len() - 1;
        let mut rst = vec![0; nums.len()];
        let mut index = nums.len() - 1;
        while right_ptr >= left_ptr {
            let larger = if nums[right_ptr].abs() > nums[left_ptr].abs() {
                let tmp = nums[right_ptr].pow(2);
                right_ptr -= 1;
                tmp
            } else {
                let tmp = nums[left_ptr].pow(2);
                left_ptr += 1;
                tmp
            };
            rst[index] = larger;
            index = index.saturating_add_signed(-1);
        }
        rst
    }

    /// # 错误
    ///
    /// 思路反了, 不应该先找最近 0 的数从左到右放元素, 而是应该让左右指针逐渐想中间靠拢, 修正后的方法见
    /// [`double_ptr_trial_2nd`](Self::double_ptr_trial_2nd)
    ///
    /// # 思路
    ///
    /// https://leetcode.cn/problems/squares-of-a-sorted-array/solutions/2806253/xiang-xiang-shuang-zhi-zhen-cong-da-dao-blda6/
    fn on_trial(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_nearest = 0;
        let mut nearest_square = i32::MAX;
        let mut rst = Vec::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            let square = v * v;
            if square <= nearest_square {
                nearest_square = square;
                zero_nearest = i;
            } else {
                break;
            }
        }
        rst.push(nearest_square);
        let mut right_iter = nums.iter().skip(zero_nearest + 1).peekable();
        let mut left_iter = nums.iter().rev().skip(nums.len() - zero_nearest).peekable();
        loop {
            let right_sq = if let Some(v) = right_iter.peek() {
                **v * **v
            } else {
                i32::MAX
            };
            let left_sq = if let Some(v) = left_iter.peek() {
                **v * **v
            } else {
                i32::MAX
            };
            let minor = if left_sq < right_sq {
                left_iter.next();
                left_sq
            } else {
                right_iter.next();
                right_sq
            };
            if minor == i32::MAX {
                break;
            }
            rst.push(minor);
        }
        rst
    }
    fn initial_trial(nums: Vec<i32>) -> Vec<i32> {
        let mut squared: Vec<i32> = nums.iter().map(|n| n * n).collect();
        squared.sort();
        squared
    }
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        Self::double_ptr_trial_2nd(nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1130_squares_of_a_sorted_array::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    }
}
