/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] Triangle
 *
 * https://leetcode.cn/problems/triangle/description/
 *
 * algorithms
 * Medium (69.63%)
 * Likes:    1483
 * Dislikes: 0
 * Total Accepted:    446.6K
 * Total Submissions: 641.5K
 * Testcase Example:  '[[2],[3,4],[6,5,7],[4,1,8,3]]'
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
 *
 * For each step, you may move to an adjacent number of the row below. More
 * formally, if you are on index i on the current row, you may move to either
 * index i or index i + 1 on the next row.
 *
 *
 * Example 1:
 *
 *
 * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * Output: 11
 * Explanation: The triangle looks like:
 * ⁠  2
 * ⁠ 3 4
 * ⁠6 5 7
 * 4 1 8 3
 * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined
 * above).
 *
 *
 * Example 2:
 *
 *
 * Input: triangle = [[-10]]
 * Output: -10
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= triangle.length <= 200
 * triangle[0].length == 1
 * triangle[i].length == triangle[i - 1].length + 1
 * -10^4 <= triangle[i][j] <= 10^4
 *
 *
 *
 * Follow up: Could you do this using only O(n) extra space, where n is the
 * total number of rows in the triangle?
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();
        for row in triangle.iter().take(n - 1).rev() {
            dp = row
                .iter()
                .zip(dp.windows(2))
                .map(|(x, w)| x + w[0].min(w[1]))
                .collect();
        }
        dp[0]
    }
}
// @lc code=end

#[cfg(test)]
mod test_triangle {
    use super::Solution;

    #[test]
    fn test1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let res = Solution::minimum_total(triangle);
        assert_eq!(res, 11);
    }

    #[test]
    fn test2() {
        let triangle = vec![vec![-10]];
        let res = Solution::minimum_total(triangle);
        assert_eq!(res, -10);
    }
}
