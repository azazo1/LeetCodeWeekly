/// https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/
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
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut cnt = [0; 24];
        for i in 0..hours.len() {
            ans += cnt[((24 - hours[i] % 24) % 24) as usize];
            cnt[(hours[i] % 24) as usize] += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let hours = vec![12, 12, 30, 24, 24];
        let rst = Solution::count_complete_day_pairs(hours);
        assert_eq!(rst, 2);
    }
}