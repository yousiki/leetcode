/*
 * @lc app=leetcode.cn id=3127 lang=rust
 *
 * [3127] 构造相同颜色的正方形
 *
 * https://leetcode.cn/problems/make-a-square-with-the-same-color/description/
 *
 * algorithms
 * Easy (56.48%)
 * Likes:    13
 * Dislikes: 0
 * Total Accepted:    10.6K
 * Total Submissions: 16.3K
 * Testcase Example:  '[["B","W","B"],["B","W","W"],["B","W","B"]]'
 *
 * 给你一个二维 3 x 3 的矩阵 grid ，每个格子都是一个字符，要么是 'B' ，要么是 'W' 。字符 'W' 表示白色，字符 'B'
 * 表示黑色。
 *
 * 你的任务是改变 至多一个 格子的颜色，使得矩阵中存在一个 2 x 2 颜色完全相同的正方形。
 *
 * 如果可以得到一个相同颜色的 2 x 2 正方形，那么返回 true ，否则返回 false 。
 *
 *
 * .grid-container {
 * ⁠ display: grid;
 * ⁠ grid-template-columns: 30px 30px 30px;
 * ⁠ padding: 10px;
 * }
 * .grid-item {
 * ⁠ background-color: black;
 * ⁠ border: 1px solid gray;
 * ⁠ height: 30px;
 * ⁠ font-size: 30px;
 * ⁠ text-align: center;
 * }
 * .grid-item-white {
 * ⁠ background-color: white;
 * }
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * 输入：grid = [["B","W","B"],["B","W","W"],["B","W","B"]]
 *
 * 输出：true
 *
 * 解释：
 *
 * 修改 grid[0][2] 的颜色，可以满足要求。
 *
 *
 * 示例 2：
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * 输入：grid = [["B","W","B"],["W","B","W"],["B","W","B"]]
 *
 * 输出：false
 *
 * 解释：
 *
 * 只改变一个格子颜色无法满足要求。
 *
 *
 * 示例 3：
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * 输入：grid = [["B","W","B"],["B","W","W"],["B","W","W"]]
 *
 * 输出：true
 *
 * 解释：
 *
 * grid 已经包含一个 2 x 2 颜色相同的正方形了。
 *
 *
 *
 *
 * 提示：
 *
 *
 * grid.length == 3
 * grid[i].length == 3
 * grid[i][j] 要么是 'W' ，要么是 'B' 。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        // Check if there is a 2x2 square with at most one different color.
        for i in 0..grid.len() - 1 {
            for j in 0..grid[i].len() - 1 {
                if Self::check(&grid, i, j) {
                    return true;
                }
            }
        }
        false
    }

    fn check(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        assert!(x < grid.len() - 1);
        assert!(y < grid[x].len() - 1);
        let count = vec![
            grid[x][y],
            grid[x + 1][y],
            grid[x][y + 1],
            grid[x + 1][y + 1],
        ]
        .iter()
        .fold([0, 0], |mut acc, &c| {
            if c == 'B' {
                acc[0] += 1;
            } else {
                acc[1] += 1;
            }
            acc
        });
        count[0] <= 1 || count[1] <= 1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B'],
        ];
        assert_eq!(Solution::can_make_square(grid), true);
    }

    #[test]
    fn case2() {
        let grid = vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B'],
        ];
        assert_eq!(Solution::can_make_square(grid), false);
    }

    #[test]
    fn case3() {
        let grid = vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'W'],
        ];
        assert_eq!(Solution::can_make_square(grid), true);
    }
}

fn main() {
    let grid = vec![
        vec!['B', 'W', 'B'],
        vec!['B', 'W', 'W'],
        vec!['B', 'W', 'B'],
    ];
    println!("{}", Solution::can_make_square(grid));
    let grid = vec![
        vec!['B', 'W', 'B'],
        vec!['W', 'B', 'W'],
        vec!['B', 'W', 'B'],
    ];
    println!("{}", Solution::can_make_square(grid));
    let grid = vec![
        vec!['B', 'W', 'B'],
        vec!['B', 'W', 'W'],
        vec!['B', 'W', 'W'],
    ];
    println!("{}", Solution::can_make_square(grid));
}
