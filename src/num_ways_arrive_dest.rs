use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution {}
/*
You are in a city that consists of n intersections numbered from 0 to n - 1 with bi-directional roads between some intersections.
The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.

You are given an integer n and a 2D integer array roads where roads[i] = [ui, vi, timei] means that there is a road between intersections ui and vi that takes timei minutes to travel.
You want to know in how many ways you can travel from intersection 0 to intersection n - 1 in the shortest amount of time.

Return the number of ways you can arrive at your destination in the shortest amount of time.
Since the answer may be large, return it modulo 109 + 7.
*/
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, HashMap<i32, usize>> = HashMap::new();
        for link in roads {
            if let &[u, v, t] = &link[..] {
                graph
                    .entry(u)
                    .or_insert_with(HashMap::new)
                    .insert(v, t as usize);
                graph
                    .entry(v)
                    .or_insert_with(HashMap::new)
                    .insert(u, t as usize);
            }
        }
        if graph.is_empty() {
            return 1;
        }
        let mut known: HashMap<i32, usize> = HashMap::new();
        let mut counts: HashMap<i32, usize> = HashMap::new();
        known.insert(0, 0);
        counts.insert(0, 1);
        let mut heap: BinaryHeap<(Reverse<usize>, (i32, i32))> = BinaryHeap::new();
        for (&v, &t) in graph[&0].iter() {
            heap.push((Reverse(t), (0, v)))
        }
        while let Some((Reverse(t), (u, v))) = heap.pop() {
            if !known.contains_key(&v) {
                known.insert(v, t);
                counts.insert(v, counts[&u]);
                for (&u, &t) in graph[&v].iter() {
                    if !known.contains_key(&u) {
                        println!("dist={},nodes={},{}", t + known[&v], v, u);
                        heap.push((Reverse(t + known[&v]), (v, u)));
                    }
                }
            } else if known[&v] == t {
                counts.insert(v, (counts[&v] + counts[&u]) % (1_000_000_000 + 7));
            }
        }
        counts[&(n - 1)] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // allows access to functions/types from the main module

    // helper only visible inside tests
    fn run_assert(n: i32, nums: Vec<Vec<i32>>, expected: i32) {
        let result = Solution::count_paths(n, nums);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_case0() {
        // IInput: n = 1, roads = []
        // Output: 1
        let roads: [Vec<i32>; 0] = [];
        run_assert(7, roads.iter().map(|row| row.to_vec()).collect(), 1);
    }
    #[test]
    fn test_case1() {
        // IInput: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
        // Output: 4
        // Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
        // The four ways to get there in 7 minutes are:
        // - 0 ➝ 6
        // - 0 ➝ 4 ➝ 6
        // - 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
        // - 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
        let roads = [
            [0, 6, 7],
            [0, 1, 2],
            [1, 2, 3],
            [1, 3, 3],
            [6, 3, 3],
            [3, 5, 1],
            [6, 5, 1],
            [2, 5, 1],
            [0, 4, 5],
            [4, 6, 2],
        ];
        run_assert(7, roads.iter().map(|row| row.to_vec()).collect(), 4);
    }

    #[test]
    fn test_case2() {
        // Input: n = 2, roads = [[1,0,10]]
        // Output: 1
        // Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
        let roads = [[1, 0, 10]];
        run_assert(2, roads.iter().map(|row| row.to_vec()).collect(), 1);
    }
}
