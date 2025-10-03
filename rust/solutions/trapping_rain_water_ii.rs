/*
 * @lc app=leetcode.cn id=407 lang=rust
 *
 * [407] Trapping Rain Water II
 *
 * https://leetcode.cn/problems/trapping-rain-water-ii/description/
 *
 * algorithms
 * Hard (57.46%)
 * Likes:    808
 * Dislikes: 0
 * Total Accepted:    50.1K
 * Total Submissions: 85.6K
 * Testcase Example:  '[[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]'
 *
 * Given an m x n integer matrix heightMap representing the height of each unit
 * cell in a 2D elevation map, return the volume of water it can trap after
 * raining.
 *
 *
 * Example 1:
 *
 *
 * Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
 * Output: 4
 * Explanation: After the rain, water is trapped between the blocks.
 * We have two small ponds 1 and 3 units trapped.
 * The total volume of water trapped is 4.
 *
 *
 * Example 2:
 *
 *
 * Input: heightMap =
 * [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
 * Output: 10
 *
 *
 *
 * Constraints:
 *
 *
 * m == heightMap.length
 * n == heightMap[i].length
 * 1 <= m, n <= 200
 * 0 <= heightMap[i][j] <= 2 * 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut total_water = 0;
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    heap.push(std::cmp::Reverse((height_map[i][j], i, j)));
                    visited[i][j] = true;
                }
            }
        }
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some(std::cmp::Reverse((height, x, y))) = heap.pop() {
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !visited[nx][ny] {
                        visited[nx][ny] = true;
                        total_water += (height - height_map[nx][ny]).max(0);
                        heap.push(std::cmp::Reverse((height.max(height_map[nx][ny]), nx, ny)));
                    }
                }
            }
        }
        total_water
    }
}
// @lc code=end

#[cfg(test)]
mod test_trapping_rain_water_ii {
    use super::Solution;

    #[test]
    fn test1() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let result = Solution::trap_rain_water(height_map);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let result = Solution::trap_rain_water(height_map);
        assert_eq!(result, 10);
    }
}
