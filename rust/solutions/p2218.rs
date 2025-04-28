/*
 * @lc app=leetcode.cn id=2218 lang=rust
 *
 * [2218] 从栈中取出 K 个硬币的最大面值和
 *
 * https://leetcode.cn/problems/maximum-value-of-k-coins-from-piles/description/
 *
 * algorithms
 * Hard (57.75%)
 * Likes:    112
 * Dislikes: 0
 * Total Accepted:    12.7K
 * Total Submissions: 20.4K
 * Testcase Example:  '[[1,100,3],[7,8,9]]\n2'
 *
 * 一张桌子上总共有 n 个硬币 栈 。每个栈有 正整数 个带面值的硬币。
 *
 * 每一次操作中，你可以从任意一个栈的 顶部 取出 1 个硬币，从栈中移除它，并放入你的钱包里。
 *
 * 给你一个列表 piles ，其中 piles[i] 是一个整数数组，分别表示第 i 个栈里 从顶到底 的硬币面值。同时给你一个正整数 k ，请你返回在
 * 恰好 进行 k 次操作的前提下，你钱包里硬币面值之和 最大为多少 。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：piles = [[1,100,3],[7,8,9]], k = 2
 * 输出：101
 * 解释：
 * 上图展示了几种选择 k 个硬币的不同方法。
 * 我们可以得到的最大面值为 101 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
 * 输出：706
 * 解释：
 * 如果我们所有硬币都从最后一个栈中取，可以得到最大面值和。
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == piles.length
 * 1 <= n <= 1000
 * 1 <= piles[i][j] <= 10^5
 * 1 <= k <= sum(piles[i].length) <= 2000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![i32::MIN; k as usize + 1];
        dp[0] = 0;
        for pile in piles.iter() {
            let acc = pile.iter().fold(Vec::new(), |mut acc, x| {
                acc.push(acc.last().unwrap_or(&0) + x);
                acc
            });
            for j in (1..=k as usize).rev() {
                for t in 1..=std::cmp::min(j, pile.len()) {
                    dp[j] = dp[j].max(dp[j - t] + acc[t - 1]);
                }
            }
        }
        dp[k as usize]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::max_value_of_coins(
                vec![
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::max_value_of_coins(
                vec![vec![
                    48, 14, 23, 38, 33, 79, 3, 52, 73, 58, 49, 23, 74, 44, 69, 76, 83, 41, 46, 32,
                    28
                ]],
                10
            ),
            421
        )
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::max_value_of_coins(
                vec![
                    vec![37, 88],
                    vec![51, 64, 65, 20, 95, 30, 26],
                    vec![9, 62, 20],
                    vec![44]
                ],
                9
            ),
            494
        )
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2)
    );
    println!(
        "{}",
        Solution::max_value_of_coins(
            vec![
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![1, 1, 1, 1, 1, 1, 700]
            ],
            7
        )
    );
    println!(
        "{}",
        Solution::max_value_of_coins(
            vec![vec![
                48, 14, 23, 38, 33, 79, 3, 52, 73, 58, 49, 23, 74, 44, 69, 76, 83, 41, 46, 32, 28
            ]],
            10
        )
    );
    println!(
        "{}",
        Solution::max_value_of_coins(
            vec![
                vec![37, 88],
                vec![51, 64, 65, 20, 95, 30, 26],
                vec![9, 62, 20],
                vec![44]
            ],
            9
        )
    );
}
