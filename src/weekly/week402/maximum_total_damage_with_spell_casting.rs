use std::cmp::max;
use std::collections::HashMap;

/// https://leetcode.cn/problems/maximum-total-damage-with-spell-casting/description/
struct Solution {}

struct Cache {
    mapping: HashMap<i32, i64>,
}

/// 缓存函数调用的结果
impl Cache {
    fn new() -> Cache {
        Cache {
            mapping: HashMap::new()
        }
    }
    fn get(&self, i: i32) -> Option<i64> {
        self.mapping.get(&i).cloned()
    }

    fn set(&mut self, k: i32, v: i64) {
        self.mapping.insert(k, v);
    }
}

impl Solution {
    /// 计算 sorted_keys: \[0, current_damage_idx\] 区间中的最大魔法伤害.
    pub fn dfs(cache: &mut Cache, cnt: &HashMap<i32, i32>, sorted_keys: &Vec<i32>, current_damage_idx: i32) -> i64 {
        if let Some(v) = cache.get(current_damage_idx) {
            return v;
        }
        if current_damage_idx < 0 {
            return 0;
        }
        let current_damage = sorted_keys[current_damage_idx as usize];
        let mut acceptable_last = current_damage_idx;
        // 排除 current_damage - 1 和 current_damage - 2 这两个伤害
        while acceptable_last > 0 && sorted_keys[(acceptable_last - 1) as usize] >= current_damage - 2 {
            acceptable_last -= 1;
        }
        // 两种情况, 一种是不用 current_damage, 另一种是用 current_damage, 第二种情况下需要考虑题目的限制.
        let rst = max(
            Self::dfs(cache, cnt, sorted_keys, current_damage_idx - 1),
            Self::dfs(cache, cnt, sorted_keys, acceptable_last - 1)
                + (current_damage as i64 * cnt[&current_damage] as i64),
        );
        cache.set(current_damage_idx, rst);
        return rst;
    }

    /// 计数, 产生一个映射, 键为不重复的伤害值, 值为此伤害值出现的次数.
    pub fn counter(power: &Vec<i32>) -> HashMap<i32, i32> {
        let mut cnt = HashMap::new();
        for i in power {
            match cnt.get(i) {
                Some(c) => {
                    cnt.insert(*i, c + 1);
                }
                None => {
                    cnt.insert(*i, 1);
                }
            }
        }
        cnt
    }

    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let cnt = Self::counter(&power);
        let mut sorted_keys: Vec<_> = cnt.keys().cloned().collect();
        sorted_keys.sort();
        Self::dfs(&mut Cache::new(), &cnt, &sorted_keys, (sorted_keys.len() - 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let power = vec![1, 1, 3, 4];
        let rst = Solution::maximum_total_damage(power);
        assert_eq!(rst, 6);
    }
}