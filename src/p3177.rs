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
        let mut dp = vec![vec![0; n as usize + 1]; k as usize + 1];
        let mut tree1 = vec![vec![0; n as usize + 1]; k as usize + 1];
        let mut tree2 = vec![vec![0; n as usize + 1]; k as usize + 1];
        nums.iter().for_each(|&num| {
            let num_rev = n - num + 1;
            for i in 0..=k as usize {
                let mut new_dp = dp[i][num as usize] + 1;
                if i > 0 {
                    new_dp = new_dp.max(Solution::query(&tree1[i - 1], num as usize - 1) + 1);
                    new_dp = new_dp.max(Solution::query(&tree2[i - 1], num_rev as usize - 1) + 1);
                }
                dp[i][num as usize] = new_dp;
                Solution::add(tree1[i].as_mut(), num as usize, new_dp);
                Solution::add(tree2[i].as_mut(), num_rev as usize, new_dp);
            }
        });
        dp.iter()
            .map(|row| *row.iter().max().unwrap())
            .max()
            .unwrap()
    }

    fn unique_map(nums: Vec<i32>) -> (Vec<i32>, i32) {
        let map = nums
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<i32, i32>, &num| {
                if !map.contains_key(&num) {
                    map.insert(num, map.len() as i32);
                }
                map
            });
        (
            nums.iter().map(|&num| map[&num] + 1).collect(),
            map.len() as i32,
        )
    }

    fn add(tree: &mut Vec<i32>, mut p: usize, v: i32) {
        while p < tree.len() {
            tree[p] = tree[p].max(v);
            p += p & p.wrapping_neg();
        }
    }

    fn query(tree: &Vec<i32>, mut p: usize) -> i32 {
        let mut res = 0;
        while p > 0 {
            res = res.max(tree[p]);
            p -= p & p.wrapping_neg();
        }
        res
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
    // println!("{}", Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0));
    // println!("{}", Solution::maximum_length(vec![1, 2, 1, 1, 3], 0));
}
