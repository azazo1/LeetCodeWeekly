// https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-subarrays/description/

use std::collections::HashMap;

impl Solution {
    pub fn first_trial(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target_map: HashMap<i32, i32> = HashMap::new();
        for i in target {
            *target_map.entry(i).or_insert(0) += 1;
        }
        let mut arr_map:HashMap<i32, i32> = HashMap::new();
        for i in arr {
            *arr_map.entry(i).or_insert(0) += 1;
        }
        return arr_map == target_map
    }
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        Self::first_trial(target, arr)
    }
}

struct Solution;