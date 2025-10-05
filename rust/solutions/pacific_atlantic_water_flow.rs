/*
 * @lc app=leetcode.cn id=417 lang=rust
 *
 * [417] Pacific Atlantic Water Flow
 *
 * https://leetcode.cn/problems/pacific-atlantic-water-flow/description/
 *
 * algorithms
 * Medium (56.79%)
 * Likes:    769
 * Dislikes: 0
 * Total Accepted:    116.8K
 * Total Submissions: 205K
 * Testcase Example:  '[[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]'
 *
 * There is an m x n rectangular island that borders both the Pacific Ocean and
 * Atlantic Ocean. The Pacific Ocean touches the island's left and top edges,
 * and the Atlantic Ocean touches the island's right and bottom edges.
 *
 * The island is partitioned into a grid of square cells. You are given an m x
 * n integer matrix heights where heights[r][c] represents the height above sea
 * level of the cell at coordinate (r, c).
 *
 * The island receives a lot of rain, and the rain water can flow to
 * neighboring cells directly north, south, east, and west if the neighboring
 * cell's height is less than or equal to the current cell's height. Water can
 * flow from any cell adjacent to an ocean into the ocean.
 *
 * Return a 2D list of grid coordinates result where result[i] = [ri, ci]
 * denotes that rain water can flow from cell (ri, ci) to both the Pacific and
 * Atlantic oceans.
 *
 *
 * Example 1:
 *
 *
 * Input: heights =
 * [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
 * Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
 * Explanation: The following cells can flow to the Pacific and Atlantic
 * oceans, as shown below:
 * [0,4]: [0,4] -> Pacific Ocean
 * [0,4] -> Atlantic Ocean
 * [1,3]: [1,3] -> [0,3] -> Pacific Ocean
 * [1,3] -> [1,4] -> Atlantic Ocean
 * [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
 * [1,4] -> Atlantic Ocean
 * [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
 * [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
 * [3,0]: [3,0] -> Pacific Ocean
 * [3,0] -> [4,0] -> Atlantic Ocean
 * [3,1]: [3,1] -> [3,0] -> Pacific Ocean
 * [3,1] -> [4,1] -> Atlantic Ocean
 * [4,0]: [4,0] -> Pacific Ocean
 * ⁠      [4,0] -> Atlantic Ocean
 * Note that there are other possible paths for these cells to flow to the
 * Pacific and Atlantic oceans.
 *
 *
 * Example 2:
 *
 *
 * Input: heights = [[1]]
 * Output: [[0,0]]
 * Explanation: The water can flow from the only cell to the Pacific and
 * Atlantic oceans.
 *
 *
 *
 * Constraints:
 *
 *
 * m == heights.length
 * n == heights[r].length
 * 1 <= m, n <= 200
 * 0 <= heights[r][c] <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let pacific_start = (0..m)
            .map(|i| (i, 0))
            .chain((0..n).map(|j| (0, j)))
            .collect::<Vec<_>>();
        let pacific_visited = Self::bfs(&heights, &pacific_start, m, n);
        let atlantic_start = (0..m)
            .map(|i| (i, n - 1))
            .chain((0..n).map(|j| (m - 1, j)))
            .collect::<Vec<_>>();
        let atlantic_visited = Self::bfs(&heights, &atlantic_start, m, n);
        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if pacific_visited[i][j] && atlantic_visited[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }

    fn bfs(
        heights: &Vec<Vec<i32>>,
        starts: &Vec<(usize, usize)>,
        m: usize,
        n: usize,
    ) -> Vec<Vec<bool>> {
        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        for &(i, j) in starts {
            if !visited[i][j] {
                visited[i][j] = true;
                queue.push_back((i, j));
            }
        }
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((i, j)) = queue.pop_front() {
            for &(di, dj) in &directions {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if !visited[ni][nj] && heights[ni][nj] >= heights[i][j] {
                        visited[ni][nj] = true;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
        visited
    }
}
// @lc code=end

#[cfg(test)]
mod test_pacific_atlantic_water_flow {
    use super::Solution;

    #[test]
    fn test1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let mut result = Solution::pacific_atlantic(heights);
        result.sort();
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let heights = vec![vec![1]];
        let result = Solution::pacific_atlantic(heights);
        let expected = vec![vec![0, 0]];
        assert_eq!(result, expected);
    }
}
