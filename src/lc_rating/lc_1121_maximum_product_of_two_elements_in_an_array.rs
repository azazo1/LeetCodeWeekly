// https://leetcode.cn/problems/maximum-product-of-two-elements-in-an-array/


struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        let mut pq = std::collections::BinaryHeap::with_capacity(3);
        for i in nums {
            pq.push(Reverse(i));
            if pq.len() > 2 {
                pq.pop();
            }
        }
        (pq.pop().unwrap().0 - 1) * (pq.pop().unwrap().0 - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1121_maximum_product_of_two_elements_in_an_array::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }
}