/*
 * @lc app=leetcode.cn id=1039 lang=rust
 *
 * [1039] Minimum Score Triangulation of Polygon
 *
 * https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/description/
 *
 * algorithms
 * Medium (68.14%)
 * Likes:    333
 * Dislikes: 0
 * Total Accepted:    36.8K
 * Total Submissions: 53.5K
 * Testcase Example:  '[1,2,3]'
 *
 * You have a convex n-sided polygon where each vertex has an integer value.
 * You are given an integer array values where values[i] is the value of the
 * i^th vertex in clockwise order.
 *
 * Polygon triangulation is a process where you divide a polygon into a set of
 * triangles and the vertices of each triangle must also be vertices of the
 * original polygon. Note that no other shapes other than triangles are allowed
 * in the division. This process will result in n - 2 triangles.
 *
 * You will triangulate the polygon. For each triangle, the weight of that
 * triangle is the product of the values at its vertices. The total score of
 * the triangulation is the sum of these weights over all n - 2 triangles.
 *
 * Return the minimum possible score that you can achieve with some
 * triangulation of the polygon.
 *
 *
 *
 * Example 1:
 *
 *
 *
 *
 * Input: values = [1,2,3]
 *
 * Output: 6
 *
 * Explanation: The polygon is already triangulated, and the score of the only
 * triangle is 6.
 *
 *
 * Example 2:
 *
 *
 *
 *
 * Input: values = [3,7,4,5]
 *
 * Output: 144
 *
 * Explanation: There are two triangulations, with possible scores: 3*7*5 +
 * 4*5*7 = 245, or 3*4*5 + 3*4*7 = 144.
 * The minimum score is 144.
 *
 *
 * Example 3:
 *
 *
 *
 *
 * Input: values = [1,3,1,4,1,5]
 *
 * Output: 13
 *
 * Explanation: The minimum score triangulation is 1*1*3 + 1*1*4 + 1*1*5 +
 * 1*1*1 = 13.
 *
 *
 *
 * Constraints:
 *
 *
 * n == values.length
 * 3 <= n <= 50
 * 1 <= values[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];

        for length in 2..n {
            for i in 0..n - length {
                let j = i + length;
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[j] * values[k]);
                }
            }
        }

        dp[0][n - 1]
    }
}
// @lc code=end

#[cfg(test)]
mod test_minimum_score_triangulation_of_polygon {
    use super::Solution;

    #[test]
    fn test_1() {
        let values = vec![1, 2, 3];
        let result = Solution::min_score_triangulation(values);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let values = vec![3, 7, 4, 5];
        let result = Solution::min_score_triangulation(values);
        assert_eq!(result, 144);
    }

    #[test]
    fn test_3() {
        let values = vec![1, 3, 1, 4, 1, 5];
        let result = Solution::min_score_triangulation(values);
        assert_eq!(result, 13);
    }
}
