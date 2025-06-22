// Category: algorithms
// Level: Medium
// Percent: 44.50056%

// You are given an integer n and an undirected tree rooted at node 0 with n nodes numbered from 0 to n - 1. This is represented by a 2D array edges of length n - 1, where edges[i] = [ui, vi] indicates an edge from node ui to vi .
//
// Each node i has an associated cost given by cost[i], representing the cost to traverse that node.
//
// The score of a path is defined as the sum of the costs of all nodes along the path.
//
// Your goal is to make the scores of all root-to-leaf paths equal by increasing the cost of any number of nodes by any non-negative amount.
//
// Return the minimum number of nodes whose cost must be increased to make all root-to-leaf path scores equal.
//
//
// Example 1:
//
//
// Input: n = 3, edges = [[0,1],[0,2]], cost = [2,1,3]
//
// Output: 1
//
// Explanation:
//
//
//
// There are two root-to-leaf paths:
//
//
// 	Path 0 → 1 has a score of 2 + 1 = 3.
// 	Path 0 → 2 has a score of 2 + 3 = 5.
//
//
// To make all root-to-leaf path scores equal to 5, increase the cost of node 1 by 2.
// Only one node is increased, so the output is 1.
//
//
// Example 2:
//
//
// Input: n = 3, edges = [[0,1],[1,2]], cost = [5,1,4]
//
// Output: 0
//
// Explanation:
//
//
//
// There is only one root-to-leaf path:
//
//
//
// 	Path 0 → 1 → 2 has a score of 5 + 1 + 4 = 10.
//
//
//
// Since only one root-to-leaf path exists, all path costs are trivially equal, and the output is 0.
//
//
// Example 3:
//
//
// Input: n = 5, edges = [[0,4],[0,1],[1,2],[1,3]], cost = [3,4,1,1,7]
//
// Output: 1
//
// Explanation:
//
//
//
// There are three root-to-leaf paths:
//
//
// 	Path 0 → 4 has a score of 3 + 7 = 10.
// 	Path 0 → 1 → 2 has a score of 3 + 4 + 1 = 8.
// 	Path 0 → 1 → 3 has a score of 3 + 4 + 1 = 8.
//
//
// To make all root-to-leaf path scores equal to 10, increase the cost of node 1 by 2. Thus, the output is 1.
//
//
//
// Constraints:
//
//
// 	2 <= n <= 10⁵
// 	edges.length == n - 1
// 	edges[i] == [ui, vi]
// 	0 <= ui, vi < n
// 	cost.length == n
// 	1 <= cost[i] <= 10⁹
// 	The input is generated such that edges represents a valid tree.
//

#[allow(unused)]
struct Solution;

#[allow(unused)]
// @lc code=start
impl Solution {
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let mut adj_nodes = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj_nodes[u].push(v);
            adj_nodes[v].push(u);
        }
        Self::dfs(0, usize::MAX, &adj_nodes, &cost).0
    }

    fn dfs(u: usize, f: usize, adj_nodes: &Vec<Vec<usize>>, cost: &Vec<i32>) -> (i32, i32) {
        let mut nodes = 0;
        let mut max_costs = vec![];
        for v in adj_nodes[u].clone() {
            if v == f {
                continue;
            }
            let (nodes_v, max_cost_v) = Self::dfs(v, u, adj_nodes, cost);
            nodes += nodes_v;
            max_costs.push(max_cost_v);
        }
        let max_cost = *max_costs.iter().max().unwrap_or(&0);
        nodes += max_costs.iter().filter(|&&x| x != max_cost).count() as i32;
        (nodes, max_cost + cost[u])
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::min_increase(3, vec![vec![0, 1], vec![0, 2]], vec![2, 1, 3]),
            1
        );
        assert_eq!(
            Solution::min_increase(3, vec![vec![0, 1], vec![1, 2]], vec![5, 1, 4]),
            0
        );
        assert_eq!(
            Solution::min_increase(
                5,
                vec![vec![0, 4], vec![0, 1], vec![1, 2], vec![1, 3]],
                vec![3, 4, 1, 1, 7]
            ),
            1
        );
        assert_eq!(
            Solution::min_increase(
                5,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![3, 4]],
                vec![6, 13, 13, 12, 25]
            ),
            2
        );
    }
}
