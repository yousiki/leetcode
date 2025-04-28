/*
 * @lc app=leetcode.cn id=2140 lang=rust
 *
 * [2140] 解决智力问题
 *
 * https://leetcode.cn/problems/solving-questions-with-brainpower/description/
 *
 * algorithms
 * Medium (45.66%)
 * Likes:    157
 * Dislikes: 0
 * Total Accepted:    30.6K
 * Total Submissions: 61.7K
 * Testcase Example:  '[[3,2],[4,3],[4,4],[2,5]]'
 *
 * 给你一个下标从 0 开始的二维整数数组 questions ，其中 questions[i] = [pointsi, brainpoweri] 。
 *
 * 这个数组表示一场考试里的一系列题目，你需要 按顺序 （也就是从问题 0 开始依次解决），针对每个问题选择 解决 或者 跳过 操作。解决问题 i 将让你
 * 获得  pointsi 的分数，但是你将 无法 解决接下来的 brainpoweri 个问题（即只能跳过接下来的 brainpoweri
 * 个问题）。如果你跳过问题 i ，你可以对下一个问题决定使用哪种操作。
 *
 *
 * 比方说，给你 questions = [[3, 2], [4, 3], [4, 4], [2, 5]] ：
 *
 *
 * 如果问题 0 被解决了， 那么你可以获得 3 分，但你不能解决问题 1 和 2 。
 * 如果你跳过问题 0 ，且解决问题 1 ，你将获得 4 分但是不能解决问题 2 和 3 。
 *
 *
 *
 *
 * 请你返回这场考试里你能获得的 最高 分数。
 *
 *
 *
 * 示例 1：
 *
 * 输入：questions = [[3,2],[4,3],[4,4],[2,5]]
 * 输出：5
 * 解释：解决问题 0 和 3 得到最高分。
 * - 解决问题 0 ：获得 3 分，但接下来 2 个问题都不能解决。
 * - 不能解决问题 1 和 2
 * - 解决问题 3 ：获得 2 分
 * 总得分为：3 + 2 = 5 。没有别的办法获得 5 分或者多于 5 分。
 *
 *
 * 示例 2：
 *
 * 输入：questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
 * 输出：7
 * 解释：解决问题 1 和 4 得到最高分。
 * - 跳过问题 0
 * - 解决问题 1 ：获得 2 分，但接下来 2 个问题都不能解决。
 * - 不能解决问题 2 和 3
 * - 解决问题 4 ：获得 5 分
 * 总得分为：2 + 5 = 7 。没有别的办法获得 7 分或者多于 7 分。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= questions.length <= 10^5
 * questions[i].length == 2
 * 1 <= pointsi, brainpoweri <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n + 1];
        for i in (0..n).rev() {
            let (points, brainpower) = (questions[i][0], questions[i][1]);
            let next = std::cmp::min(i + brainpower as usize + 1, n);
            dp[i] = std::cmp::max(dp[i + 1], points as i64 + dp[next]);
        }
        dp[0]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ]),
            7
        );
    }
}

fn main() {
    println!(
        "{}",
        Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
    );
    println!(
        "{}",
        Solution::most_points(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ])
    );
}
