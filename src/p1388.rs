/*
 * @lc app=leetcode.cn id=1388 lang=rust
 *
 * [1388] 3n 块披萨
 *
 * https://leetcode.cn/problems/pizza-with-3n-slices/description/
 *
 * algorithms
 * Hard (64.34%)
 * Likes:    241
 * Dislikes: 0
 * Total Accepted:    19.5K
 * Total Submissions: 30.4K
 * Testcase Example:  '[1,2,3,4,5,6]'
 *
 * 给你一个披萨，它由 3n 块不同大小的部分组成，现在你和你的朋友们需要按照如下规则来分披萨：
 *
 *
 * 你挑选 任意 一块披萨。
 * Alice 将会挑选你所选择的披萨逆时针方向的下一块披萨。
 * Bob 将会挑选你所选择的披萨顺时针方向的下一块披萨。
 * 重复上述过程直到没有披萨剩下。
 *
 *
 * 每一块披萨的大小按顺时针方向由循环数组 slices 表示。
 *
 * 请你返回你可以获得的披萨大小总和的最大值。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：slices = [1,2,3,4,5,6]
 * 输出：10
 * 解释：选择大小为 4 的披萨，Alice 和 Bob 分别挑选大小为 3 和 5 的披萨。然后你选择大小为 6 的披萨，Alice 和 Bob
 * 分别挑选大小为 2 和 1 的披萨。你获得的披萨总大小为 4 + 6 = 10 。
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：slices = [8,9,8,6,1,1]
 * 输出：16
 * 解释：两轮都选大小为 8 的披萨。如果你选择大小为 9 的披萨，你的朋友们就会选择大小为 8 的披萨，这种情况下你的总和不是最大的。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= slices.length <= 500
 * slices.length % 3 == 0
 * 1 <= slices[i] <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let mut slices = slices.clone();
        let n = slices.len();
        slices.append(&mut slices.clone());
        let m = slices.len();
        let mut dp = vec![vec![0; m + 1]; m + 1];
        for length in 1..=n / 3 {
            let length = length * 3;
            for l in 0..=m - length {
                let r = l + length;
                // Pick all slices in [l, r)
                // The last step is picking l, k, r-1
                for i in 0..length / 3 {
                    let k = l + i * 3 + 1;
                    dp[l][r] = dp[l][r].max(dp[l + 1][k] + dp[k + 1][r - 1] + slices[k]);
                }
                // The last step is not picking l, k, r-1
                for i in 1..length / 3 {
                    let k = l + i * 3;
                    dp[l][r] = dp[l][r].max(dp[l][k] + dp[k][r]);
                }
            }
        }
        (0..n).map(|i| dp[i][i + n]).max().unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::max_size_slices(vec![10, 9, 1, 10, 8, 5, 10, 2, 2]),
            30
        );
    }
}

fn main() {
    println!("{}", Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]));
    println!("{}", Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]));
    println!(
        "{}",
        Solution::max_size_slices(vec![10, 9, 1, 10, 8, 5, 10, 2, 2])
    );
}
