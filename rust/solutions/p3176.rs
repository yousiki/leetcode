/*
 * @lc app=leetcode.cn id=3176 lang=rust
 *
 * [3176] 求出最长好子序列 I
 *
 * https://leetcode.cn/problems/find-the-maximum-length-of-a-good-subsequence-i/description/
 *
 * algorithms
 * Medium (31.08%)
 * Likes:    38
 * Dislikes: 0
 * Total Accepted:    10.9K
 * Total Submissions: 24.6K
 * Testcase Example:  '[1,2,1,1,3]\n2'
 *
 * 给你一个整数数组 nums 和一个 非负 整数 k 。如果一个整数序列 seq 满足在下标范围 [0, seq.length - 2] 中 最多只有 k
 * 个下标 i 满足 seq[i] != seq[i + 1] ，那么我们称这个整数序列为 好 序列。
 *
 * 请你返回 nums 中 好 子序列 的最长长度。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,1,1,3], k = 2
 *
 * 输出：4
 *
 * 解释：
 *
 * 最长好子序列为 [1,2,1,1,3] 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3,4,5,1], k = 0
 *
 * 输出：2
 *
 * 解释：
 *
 * 最长好子序列为 [1,2,3,4,5,1] 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 500
 * 1 <= nums[i] <= 10^9
 * 0 <= k <= min(nums.length, 25)
 *
 *
 */

use std::vec;

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .fold(Vec::new(), |mut dp: Vec<Vec<i32>>, (i, &num)| {
                dp.push(nums.iter().take(i).zip(dp.iter()).fold(
                    vec![1; k as usize + 1],
                    |mut new_dp, (&prev_num, prev_dp)| {
                        let cost = (prev_num != num) as usize;
                        for j in cost..=k as usize {
                            new_dp[j] = new_dp[j].max(prev_dp[j - cost] + 1);
                        }
                        new_dp
                    },
                ));
                dp
            })
            .iter()
            .map(|v| *v.iter().max().unwrap())
            .max()
            .unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 0), 3);
    }
}

fn main() {
    println!("{}", Solution::maximum_length(vec![1, 2, 1, 1, 3], 2));
    println!("{}", Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0));
    println!("{}", Solution::maximum_length(vec![1, 2, 1, 1, 3], 0));
}
