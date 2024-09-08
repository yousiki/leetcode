/*
 * @lc app=leetcode.cn id=3177 lang=rust
 *
 * [3177] 求出最长好子序列 II
 *
 * https://leetcode.cn/problems/find-the-maximum-length-of-a-good-subsequence-ii/description/
 *
 * algorithms
 * Hard (37.29%)
 * Likes:    24
 * Dislikes: 0
 * Total Accepted:    7.9K
 * Total Submissions: 14K
 * Testcase Example:  '[1,2,1,1,3]\n2'
 *
 * 给你一个整数数组 nums 和一个 非负 整数 k 。如果一个整数序列 seq 满足在范围下标范围 [0, seq.length - 2] 中存在
 * 不超过 k 个下标 i 满足 seq[i] != seq[i + 1] ，那么我们称这个整数序列为 好 序列。
 *
 * 请你返回 nums 中 好 子序列 的最长长度
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
 * 1 <= nums.length <= 5 * 10^3
 * 1 <= nums[i] <= 10^9
 * 0 <= k <= min(50, nums.length)
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let (nums, n) = Solution::unique_map(nums);
        let mut dp = vec![vec![0; n]; k as usize + 1];
        let mut max = vec![0; k as usize + 1];
        for num in nums.into_iter() {
            for i in (0..=k as usize).rev() {
                // prev == num: dp[i][num] = dp[i][num] + 1
                dp[i][num] = dp[i][num] + 1;
                // prev != num: dp[i][num] = max_j(dp[i-1][j]) + 1
                if i > 0 {
                    dp[i][num] = dp[i][num].max(max[i - 1] + 1);
                }
                max[i] = max[i].max(dp[i][num]);
            }
        }
        max[k as usize] as i32
    }

    fn unique_map(nums: Vec<i32>) -> (Vec<usize>, usize) {
        let map = nums
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<i32, usize>, &num| {
                if !map.contains_key(&num) {
                    map.insert(num, map.len());
                }
                map
            });
        (nums.iter().map(|&num| map[&num]).collect(), map.len())
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
