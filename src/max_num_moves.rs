pub struct Solution {}
/*
2684. Maximum Number of Moves in a Grid
You are given a 0-indexed m x n matrix grid consisting of positive integers.

You can start at any cell in the first column of the matrix, and traverse the grid in the following way:

From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1)
such that the value of the cell you move to, should be strictly bigger than the value of the current cell.
Return the maximum number of moves that you can perform.

*/
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![0; m];
        for i in (0..n - 1).rev() {
            let mut next = vec![0; m];
            for j in 0..m {
                let mut moves: Vec<i32> = vec![0];
                if grid[j][i] < grid[j][i + 1] {
                    moves.push(result[j] + 1);
                }
                if j > 0 && grid[j][i] < grid[j - 1][i + 1] {
                    moves.push(result[j - 1] + 1);
                }
                if j < m - 1 && grid[j][i] < grid[j + 1][i + 1] {
                    moves.push(result[j + 1] + 1);
                }
                next[j] = *moves.iter().max().unwrap();
            }
            result = next;
        }
        *result.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // allows access to functions/types from the main module

    // helper only visible inside tests
    fn run_assert(nums: Vec<Vec<i32>>, expected: i32) {
        let result = Solution::max_moves(nums);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_case1() {
        // Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
        // Output: 0
        // Explanation: Starting from any cell in the first column we cannot perform any moves.
        run_assert(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]], 0);
    }

    #[test]
    fn test_case2() {
        // Input: nums = [3,2,4], target = 6
        // Output: [1,2]
        run_assert(
            vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15],
            ],
            3,
        );
    }
}
