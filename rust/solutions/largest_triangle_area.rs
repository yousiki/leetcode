/*
 * @lc app=leetcode.com id=812 lang=rust
 *
 * [812] Largest Triangle Area
 *
 * https://leetcode.com/problems/largest-triangle-area/description/
 *
 * algorithms
 * Easy (68.26%)
 * Likes:    208
 * Dislikes: 0
 * Total Accepted:    42.5K
 * Total Submissions: 62.3K
 * Testcase Example:  '[[0,0],[0,1],[1,0],[0,2],[2,0]]'
 *
 * Given an array of points on the X-Y plane points where points[i] = [xi, yi],
 * return the area of the largest triangle that can be formed by any three
 * different points. Answers within 10^-5 of the actual answer will be
 * accepted.
 *
 *
 * Example 1:
 *
 *
 * Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
 * Output: 2.00000
 * Explanation: The five points are shown in the above figure. The red triangle
 * is the largest.
 *
 *
 * Example 2:
 *
 *
 * Input: points = [[1,0],[0,0],[0,1]]
 * Output: 0.50000
 *
 *
 *
 * Constraints:
 *
 *
 * 3 <= points.length <= 50
 * -50 <= xi, yi <= 50
 * All the given points are unique.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut answer = 0f64;
        for i in 0..points.len() - 2 {
            for j in i + 1..points.len() - 1 {
                for k in j + 1..points.len() {
                    let area = Self::triangle_area(&points[i], &points[j], &points[k]);
                    if area > answer {
                        answer = area;
                    }
                }
            }
        }
        answer
    }

    fn triangle_area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
        let area = (a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1])).abs();
        area as f64 / 2.0
    }
}
// @lc code=end

#[cfg(test)]
mod test_largest_triangle_area {
    use super::Solution;

    #[test]
    fn test1() {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        let result = 2.0;
        assert_eq!(Solution::largest_triangle_area(points), result);
    }

    #[test]
    fn test2() {
        let points = vec![vec![1, 0], vec![0, 0], vec![0, 1]];
        let result = 0.5;
        assert_eq!(Solution::largest_triangle_area(points), result);
    }
}
