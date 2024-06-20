// https://leetcode.cn/problems/number-of-students-doing-homework-at-a-given-time/description/

impl Solution {
    fn bucket(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut bucket = Vec::with_capacity(1001);
        bucket.resize(1001, 0);
        for i in 0..start_time.len() {
            for j in start_time[i]..=end_time[i] {
                bucket[j as usize] += 1;
            }
        }
        bucket[query_time as usize]
    }
    /// 差分数组
    fn difference_array(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut max_end_time = 0;
        if let Some(&max_end) = end_time.iter().max() {
            max_end_time = max_end;
        }
        if query_time > max_end_time {
            return 0;
        }
        // 加 2 的原因在于, 题目说数值的最大值是 1000, 那么, 不管从 0 开始还是从 1 开始, 都需要 1001 个位置来保证 1000 这个数有对应的位置.
        // 同时对于 1000 这个数值如果是 end_time, 需要 1000 + 1 = 1001 这个索引的元素进行标记, 表示人数的减少, 所以需要 1002 个位置.
        let mut diff = vec![0; 2 + max_end_time as usize];
        // 假设 a 是 bucket.
        // 差分数组 d[i] = a[i] - a[i - 1], d[0] = a[0], 那么对 d[0..query_time + 1] 求和就得到了 a[query_time].
        // https://blog.csdn.net/qq_44786250/article/details/100056975, 看文章中的图.
        for i in 0..start_time.len() {
            diff[start_time[i] as usize] += 1;
            diff[end_time[i] as usize + 1] -= 1;
        }
        // 计算 [0, query_time] 中的和.
        diff.iter().take(query_time as usize + 1).sum()
    }
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        Self::difference_array(start_time, end_time, query_time)
    }
}

struct Solution;