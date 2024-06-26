// https://leetcode.cn/problems/n-th-tribonacci-number/description/


type Matrix = [[i32; 3]; 3];
pub fn mat_mul(m1: &Matrix, m2: &Matrix) -> Matrix {
    let mut new_mat = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            new_mat[i][j] = m1[i][0] * m2[0][j] + m1[i][1] * m2[1][j] + m1[i][2] * m2[2][j];
        }
    }
    new_mat
}

impl Solution {
    /// https://leetcode.cn/problems/n-th-tribonacci-number/solutions/921898/di-n-ge-tai-bo-na-qi-shu-by-leetcode-sol-kn16/
    pub fn matrix_trial(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        } else if n == 3 {
            return 2;
        }
        let log = (n - 2).ilog2();
        let rem = (n - 2) - 2i32.pow(log);
        let initial_matrix = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
        let mut mat = initial_matrix.clone();
        for _ in 0..log {
            mat = mat_mul(&mat, &mat);
        }
        for _ in 0..rem {
            mat = mat_mul(&mat, &initial_matrix);
        }
        mat[0][0] + mat[0][1]
    }
    pub fn first_trial(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 1;
        }
        let mut n2 = 0;
        let mut n1 = 1;
        let mut n0 = 1;
        for i in 3..=n {
            let tmp = n0 + n1 + n2;
            n2 = n1;
            n1 = n0;
            n0 = tmp;
        }
        n0
    }
    pub fn tribonacci(n: i32) -> i32 {
        Self::first_trial(n)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1143_n_th_tribonacci_number::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::matrix_trial(4), 4);
    }
}