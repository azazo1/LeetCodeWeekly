/// https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-i/
///
/// 给你一个整数数组 hours，表示以 **小时** 为单位的时间，
/// 返回一个整数，表示满足 i < j 且 hours\[i\] + hours\[j\]
/// 构成 整天 的下标对 i, j 的数目。
///
/// 整天 定义为时间持续时间是 24 小时的 整数倍 。
///
/// 例如，1 天是 24 小时，2 天是 48 小时，3 天是 72 小时，以此类推。
struct Solution {}

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..hours.len() {
            for j in i + 1..hours.len() {
                if (hours[i] + hours[j]) % 24 == 0 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

pub fn main() {
    let hours = vec![12, 12, 30, 24, 24];
    let rst = Solution::count_complete_day_pairs(hours);
    println!("{rst}");
}