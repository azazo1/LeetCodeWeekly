// https://leetcode.cn/problems/count-negative-numbers-in-a-sorted-matrix/description/

impl Solution {
    fn initial_trial(grid: Vec<Vec<i32>>) -> i32 {
        let row_cnt = grid.len();
        let column_cnt = grid[0].len();

        let mut row: i32 = (row_cnt - 1) as i32;
        let mut column = 0;

        let mut rst: i32 = 0;

        while row >= 0 {
            while column < column_cnt && grid[row as usize][column] >= 0 {
                column += 1;
            }
            rst += (column_cnt - column) as i32;
            row -= 1;
        }
        rst
    }
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        Self::initial_trial(grid)
    }
}

struct Solution;