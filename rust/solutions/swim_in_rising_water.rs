/*
 * @lc app=leetcode id=778 lang=rust
 *
 * [778] Swim in Rising Water
 *
 * https://leetcode.com/problems/swim-in-rising-water/description/
 *
 * algorithms
 * Hard (65.43%)
 * Likes:    4276
 * Dislikes: 299
 * Total Accepted:    287.3K
 * Total Submissions: 439.4K
 * Testcase Example:  '[[0,2],[1,3]]'
 *
 * You are given an n x n integer matrix grid where each value grid[i][j]
 * represents the elevation at that point (i, j).
 *
 * It starts raining, and water gradually rises over time. At time t, the water
 * level is t, meaning any cell with elevation less than equal to t is
 * submerged or reachable.
 *
 * You can swim from a square to another 4-directionally adjacent square if and
 * only if the elevation of both squares individually are at most t. You can
 * swim infinite distances in zero time. Of course, you must stay within the
 * boundaries of the grid during your swim.
 *
 * Return the minimum time until you can reach the bottom right square (n - 1,
 * n - 1) if you start at the top left square (0, 0).
 *
 *
 * Example 1:
 *
 *
 * Input: grid = [[0,2],[1,3]]
 * Output: 3
 * Explanation:
 * At time 0, you are in grid location (0, 0).
 * You cannot go anywhere else because 4-directionally adjacent neighbors have
 * a higher elevation than t = 0.
 * You cannot reach point (1, 1) until time 3.
 * When the depth of water is 3, we can swim anywhere inside the grid.
 *
 *
 * Example 2:
 *
 *
 * Input: grid =
 * [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
 * Output: 16
 * Explanation: The final route is shown.
 * We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
 *
 *
 *
 * Constraints:
 *
 *
 * n == grid.length
 * n == grid[i].length
 * 1 <= n <= 50
 * 0 <= grid[i][j] < n^2
 * Each value grid[i][j] is unique.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let m = grid.len();
        let n = grid[0].len();

        let mut distances = vec![vec![i32::MAX; n]; m];
        let mut heap = BinaryHeap::new();

        distances[0][0] = grid[0][0];
        heap.push(Reverse((distances[0][0], 0, 0)));

        while let Some(Reverse((d, x, y))) = heap.pop() {
            if d != distances[x][y] {
                continue;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if 0 <= nx && nx < m as i32 && 0 <= ny && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let nd = std::cmp::max(d, grid[nx][ny]);
                    if distances[nx][ny] > d {
                        distances[nx][ny] = nd;
                        heap.push(Reverse((nd, nx, ny)));
                    }
                }
            }
        }

        distances[m - 1][n - 1]
    }
}
// @lc code=end

#[cfg(test)]
mod test_swim_in_rising_water {
    use super::*;

    #[test]
    fn test1() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        assert_eq!(Solution::swim_in_water(grid), 3);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(grid), 16);
    }
}
