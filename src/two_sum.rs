use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            if lookup.contains_key(&nums[i]) {
                let compl = lookup[&nums[i]];
                if compl != i {
                    return vec![i as i32, compl as i32];
                }
            }
            lookup.insert(target - nums[i], i);
        }
        return vec![-1, -1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet; // allows access to functions/types from the main module

    // helper only visible inside tests
    fn check_two_sum(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        let ans: HashSet<_> = expected.into_iter().collect();
        let result: HashSet<_> = Solution::two_sum(nums, target).into_iter().collect();
        assert_eq!(result, ans);
    }
    #[test]
    fn test_case1() {
        // Input: nums = [2,7,11,15], target = 9
        // Output: [0,1]
        // Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
        check_two_sum(vec![2, 7, 11, 15], 9, vec![0, 1]);
    }

    #[test]
    fn test_case2() {
        // Input: nums = [3,2,4], target = 6
        // Output: [1,2]
        check_two_sum(vec![3, 2, 4], 6, vec![1, 2]);
    }

    #[test]
    fn test_case3() {
        // Input: nums = [3,3], target = 6
        // Output: [0,1]
        check_two_sum(vec![3, 3], 6, vec![0, 1]);
    }
}
