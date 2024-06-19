// https://leetcode.cn/problems/find-lucky-integer-in-an-array/description/

struct Solution {}
use std::collections::HashMap;


impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut hashmap = HashMap::new();
        for i in arr {
            match hashmap.get_mut(&i) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    hashmap.insert(i, 1);
                }
            }
        }
        let mut max = -1;
        for (k, v) in hashmap {
            if k == v && k > max {
                max = k;
            }
        }
        max
    }
}