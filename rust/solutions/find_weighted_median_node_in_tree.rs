/*
 * @lc app=leetcode.cn id=3585 lang=rust
 *
 * [3585] Find Weighted Median Node in Tree
 *
 * https://leetcode.cn/problems/find-weighted-median-node-in-tree/description/
 *
 * algorithms
 * Hard (43.25%)
 * Likes:    4
 * Dislikes: 0
 * Total Accepted:    1.2K
 * Total Submissions: 2.8K
 * Testcase Example:  '2\n[[0,1,7]]\n[[1,0],[0,1]]'
 *
 * You are given an integer n and an undirected, weighted tree rooted at node 0
 * with n nodes numbered from 0 to n - 1. This is represented by a 2D array
 * edges of length n - 1, where edges[i] = [ui, vi, wi] indicates an edge from
 * node ui to vi with weight wi.
 *
 * The weighted median node is defined as the first node x on the path from ui
 * to vi such that the sum of edge weights from ui to x is greater than or
 * equal to half of the total path weight.
 *
 * You are given a 2D integer array queries. For each queries[j] = [uj, vj],
 * determine the weighted median node along the path from uj to vj.
 *
 * Return an array ans, where ans[j] is the node index of the weighted median
 * for queries[j].
 *
 *
 * Example 1:
 *
 *
 * Input: n = 2, edges = [[0,1,7]], queries = [[1,0],[0,1]]
 *
 * Output: [0,1]
 *
 * Explanation:
 *
 *
 *
 *
 *
 *
 * Query
 * Path
 * Edge
 * Weights
 * Total
 * Path
 * Weight
 * Half
 * Explanation
 * Answer
 *
 *
 *
 *
 * [1, 0]
 * 1 → 0
 * [7]
 * 7
 * 3.5
 * Sum from 1 → 0 = 7 >= 3.5, median is node 0.
 * 0
 *
 *
 * [0, 1]
 * 0 → 1
 * [7]
 * 7
 * 3.5
 * Sum from 0 → 1 = 7 >= 3.5, median is node 1.
 * 1
 *
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input: n = 3, edges = [[0,1,2],[2,0,4]], queries = [[0,1],[2,0],[1,2]]
 *
 * Output: [1,0,2]
 *
 * Explanation:
 *
 *
 *
 *
 *
 *
 * Query
 * Path
 * Edge
 * Weights
 * Total
 * Path
 * Weight
 * Half
 * Explanation
 * Answer
 *
 *
 *
 *
 * [0, 1]
 * 0 → 1
 * [2]
 * 2
 * 1
 * Sum from 0 → 1 = 2 >= 1, median is node 1.
 * 1
 *
 *
 * [2, 0]
 * 2 → 0
 * [4]
 * 4
 * 2
 * Sum from 2 → 0 = 4 >= 2, median is node 0.
 * 0
 *
 *
 * [1, 2]
 * 1 → 0 → 2
 * [2, 4]
 * 6
 * 3
 * Sum from 1 → 0 = 2 < 3.
 * Sum from 1 → 2 = 2 + 4 = 6 >= 3, median is node 2.
 * 2
 *
 *
 *
 *
 *
 * Example 3:
 *
 *
 * Input: n = 5, edges = [[0,1,2],[0,2,5],[1,3,1],[2,4,3]], queries =
 * [[3,4],[1,2]]
 *
 * Output: [2,2]
 *
 * Explanation:
 *
 *
 *
 *
 *
 *
 * Query
 * Path
 * Edge
 * Weights
 * Total
 * Path
 * Weight
 * Half
 * Explanation
 * Answer
 *
 *
 *
 *
 * [3, 4]
 * 3 → 1 → 0 → 2 → 4
 * [1, 2, 5, 3]
 * 11
 * 5.5
 * Sum from 3 → 1 = 1 < 5.5.
 * Sum from 3 → 0 = 1 + 2 = 3 < 5.5.
 * Sum from 3 → 2 = 1 + 2 + 5 = 8 >= 5.5, median is node 2.
 * 2
 *
 *
 * [1, 2]
 * 1 → 0 → 2
 * [2, 5]
 * 7
 * 3.5
 *
 * Sum from 1 → 0 = 2 < 3.5.
 * Sum from 1 → 2 = 2 + 5 = 7 >= 3.5, median is node
 * 2.
 *
 * 2
 *
 *
 *
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= n <= 10^5
 * edges.length == n - 1
 * edges[i] == [ui, vi, wi]
 * 0 <= ui, vi < n
 * 1 <= wi <= 10^9
 * 1 <= queries.length <= 10^5
 * queries[j] == [uj, vj]
 * 0 <= uj, vj < n
 * The input is generated such that edges represents a valid tree.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_median(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let log_n = (n as f64).log2().ceil() as usize + 1;
        // Create an adjacency list to represent the tree
        let mut adjacency_list = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            adjacency_list[u].push((v, w));
            adjacency_list[v].push((u, w));
        }
        // The depth of each node
        let mut depth = vec![0; n];
        // The weighted depth of each node
        let mut weighted_depth = vec![i64::MIN; n];
        // The ancestors of each node (ST table)
        let mut ancestors = vec![vec![0; log_n]; n];
        // Take node 0 as the root and perform BFS
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        weighted_depth[0] = 0;
        while let Some(node) = queue.pop_front() {
            for &(neighbor, weight) in &adjacency_list[node] {
                if weighted_depth[neighbor] == i64::MIN {
                    depth[neighbor] = depth[node] + 1;
                    weighted_depth[neighbor] = weighted_depth[node] + weight as i64;
                    ancestors[neighbor][0] = node;
                    for i in 1..log_n {
                        ancestors[neighbor][i] = ancestors[ancestors[neighbor][i - 1]][i - 1];
                        if ancestors[neighbor][i] == 0 {
                            break;
                        }
                    }
                    queue.push_back(neighbor);
                }
            }
        }
        // Process each query
        queries
            .iter()
            .map(|query| {
                let u = query[0] as usize;
                let v = query[1] as usize;
                // Find the LCA of u and v
                let lca = Self::lca(u, v, &depth, &ancestors);
                // Calculate the total path weight from u to v
                let total_weight = weighted_depth[u] + weighted_depth[v] - 2 * weighted_depth[lca];
                // Find the weighted median node
                Self::find_weighted_median(u, v, lca, total_weight, &weighted_depth, &ancestors)
            })
            .collect()
    }

    /// Support function to find the LCA of two nodes
    fn lca(node_a: usize, node_b: usize, depth: &Vec<i32>, ancestors: &Vec<Vec<usize>>) -> usize {
        let mut node_a = node_a;
        let mut node_b = node_b;
        // Ensure node_a is deeper than node_b
        if depth[node_a] < depth[node_b] {
            std::mem::swap(&mut node_a, &mut node_b);
        }
        // Lift node_a to the same depth as node_b
        for i in (0..ancestors[0].len()).rev() {
            if depth[ancestors[node_a][i]] >= depth[node_b] {
                node_a = ancestors[node_a][i];
            }
        }
        // If they are already the same node, return it
        if node_a == node_b {
            return node_a;
        }
        // Lift both nodes until they meet
        for i in (0..ancestors[0].len()).rev() {
            if ancestors[node_a][i] != ancestors[node_b][i] {
                node_a = ancestors[node_a][i];
                node_b = ancestors[node_b][i];
            }
        }
        // Return the parent of either node, which is the LCA
        ancestors[node_a][0]
    }

    /// Support function to find the weighted median node
    fn find_weighted_median(
        u: usize,
        v: usize,
        lca: usize,
        total_weight: i64,
        weighted_depth: &Vec<i64>,
        ancestors: &Vec<Vec<usize>>,
    ) -> i32 {
        if (weighted_depth[u] - weighted_depth[lca]) * 2 >= total_weight {
            // Find the median on the path from u to lca
            let mut current = u;
            for i in (0..ancestors[0].len()).rev() {
                if (weighted_depth[u] - weighted_depth[ancestors[current][i]]) * 2 < total_weight {
                    current = ancestors[current][i];
                }
            }
            if current == lca {
                // If we reached the LCA, return it
                return lca as i32;
            } else {
                // Otherwise, return the parent of the current node
                ancestors[current][0] as i32
            }
        } else {
            // Find the median on the path from v to lca
            let mut current = v;
            for i in (0..ancestors[0].len()).rev() {
                if (weighted_depth[v] - weighted_depth[ancestors[current][i]]) * 2 <= total_weight {
                    current = ancestors[current][i];
                }
            }
            current as i32
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test_find_weighted_median_node_in_tree {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median(2, vec![vec![0, 1, 7]], vec![vec![1, 0], vec![0, 1]]),
            vec![0, 1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median(
                3,
                vec![vec![0, 1, 2], vec![2, 0, 4]],
                vec![vec![0, 1], vec![2, 0], vec![1, 2]]
            ),
            vec![1, 0, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_median(
                5,
                vec![vec![0, 1, 2], vec![0, 2, 5], vec![1, 3, 1], vec![2, 4, 3]],
                vec![vec![3, 4], vec![1, 2]]
            ),
            vec![2, 2]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_median(2, vec![vec![0, 1, 1]], vec![vec![0, 1]]),
            vec![1]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::find_median(2, vec![vec![0, 1, 2]], vec![vec![1, 1]]),
            vec![1]
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::find_median(3, vec![vec![0, 1, 9], vec![1, 2, 4]], vec![vec![1, 2]]),
            vec![2]
        );
    }
}
