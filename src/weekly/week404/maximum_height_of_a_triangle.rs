// https://leetcode.cn/contest/weekly-contest-404/problems/maximum-height-of-a-triangle/

use std::cmp::max;

impl Solution {
    /// return height
    fn rec(red: i32, blue: i32, current_depth: i32, red_color: bool) -> i32 {
        if red < current_depth && blue < current_depth {
            return 0;
        }
        if red == current_depth && blue == 0 && red_color {
            return current_depth;
        }
        if blue == current_depth && red == 0 && !red_color {
            return current_depth;
        }
        let mut height = 0;
        if red >= current_depth && red_color {
            height = max(height, match Self::rec(red - current_depth, blue, current_depth + 1, !red_color) {
                0 => current_depth,
                v => v
            });
        }
        if blue >= current_depth && !red_color {
            height = max(height, match Self::rec(red, blue - current_depth, current_depth + 1, !red_color) {
                0 => current_depth,
                v => v
            });
        }
        height
    }
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        // let matrix = [[0; 100]; 100];
        max(max(Self::rec(red -1, blue, 2, false), 1), max(1, Self::rec(red, blue -1, 2, true)))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::weekly::week404::maximum_height_of_a_triangle::Solution;

    #[test]
    fn test() {
        // assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
        // assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
        // assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
        assert_eq!(Solution::max_height_of_triangle(8, 2), 3);
    }
}