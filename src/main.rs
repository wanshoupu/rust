mod two_sum;
mod max_num_moves;

use crate::two_sum::Solution;

fn main() {
    let tests: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![2, 7, 11, 15], 9, vec![2, 7]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];
    for test in tests {
        let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, test.2, "Test failed: {:?}", test);
    }
}
