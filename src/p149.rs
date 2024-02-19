/*
 * @lc app=leetcode.cn id=149 lang=rust
 *
 * [149] 直线上最多的点数
 *
 * https://leetcode.cn/problems/max-points-on-a-line/description/
 *
 * algorithms
 * Hard (40.05%)
 * Likes:    536
 * Dislikes: 0
 * Total Accepted:    88.2K
 * Total Submissions: 220.1K
 * Testcase Example:  '[[1,1],[2,2],[3,3]]'
 *
 * 给你一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。求最多有多少个点在同一条直线上。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：points = [[1,1],[2,2],[3,3]]
 * 输出：3
 *
 *
 * 示例 2：
 *
 *
 * 输入：points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * 输出：4
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * points[i].length == 2
 * -10^4 i, yi
 * points 中的所有点 互不相同
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = if points.is_empty() { 0 } else { 1 };
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let mut cnt = 2;
                for k in (j + 1)..points.len() {
                    if (points[i][0] - points[j][0]) * (points[i][1] - points[k][1])
                        == (points[i][1] - points[j][1]) * (points[i][0] - points[k][0])
                    {
                        cnt += 1;
                    }
                }
                ans = ans.max(cnt);
            }
        }
        ans
    }
}
// @lc code=end

fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(Solution::max_points(points), 3);

    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    assert_eq!(Solution::max_points(points), 4);
}
