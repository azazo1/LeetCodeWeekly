// https://leetcode.cn/problems/peaks-in-array/description/

struct TimeoutSolution {}

struct Solution {}

impl TimeoutSolution {
    fn count_peaks_in(slice: &[i32]) -> i32 {
        if slice.len() < 2 {
            return 0;
        }
        let mut cnt = 0;
        for i in 1..slice.len() - 1 {
            if slice[i] > slice[i - 1] && slice[i] > slice[i + 1] {
                cnt += 1;
            }
        }
        cnt
    }
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut rst = Vec::new();
        for v in queries {
            match v[0] {
                1 => {
                    rst.push(Self::count_peaks_in(&nums[v[1] as usize..=v[2] as usize]));
                }
                2 => {
                    nums[v[1] as usize] = v[2];
                }
                _ => {
                    panic!("Invalid input");
                }
            }
        }
        rst
    }
}

impl Solution {
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {}
}

#[cfg(test)]
mod tests {
    use crate::week402::peaks_in_array::Solution;

    #[test]
    pub fn test() {
        let nums = vec![3, 1, 4, 2, 5];
        let queries = vec![vec![2, 3, 4], vec![1, 0, 4]];
        let rst = Solution::count_of_peaks(nums, queries);
        assert_eq!(rst, vec![0]);
    }

    #[test]
    pub fn test1() {
        let nums = vec![4, 1, 4, 2, 1, 5];
        let queries = vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]];
        let rst = Solution::count_of_peaks(nums, queries);
        assert_eq!(rst, vec![0, 1]);
    }

    #[test]
    pub fn test2() {
        let nums = vec![7, 10, 7];
        let queries = vec![vec![1, 2, 2], vec![2, 0, 6], vec![1, 0, 2]];
        let rst = Solution::count_of_peaks(nums, queries);
        assert_eq!(rst, vec![0, 1]);
    }
}