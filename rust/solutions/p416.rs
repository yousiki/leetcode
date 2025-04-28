/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 *
 * https://leetcode.cn/problems/partition-equal-subset-sum/description/
 *
 * algorithms
 * Medium (53.31%)
 * Likes:    2309
 * Dislikes: 0
 * Total Accepted:    730.4K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,5,11,5]'
 *
 * 给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,5,11,5]
 * 输出：true
 * 解释：数组可以分割成 [1, 5, 5] 和 [11] 。
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3,5]
 * 输出：false
 * 解释：数组不能分割成两个元素和相等的子集。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            false
        } else {
            let target = (sum / 2) as usize;
            let mut dp = vec![false; target + 1];
            dp[0] = true;
            for num in nums {
                if num as usize <= target {
                    for j in (num as usize..=target).rev() {
                        dp[j] = dp[j] || dp[j - num as usize];
                    }
                }
            }
            dp[target]
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::can_partition(vec![3, 3, 3, 4, 5]), true);
    }
}

fn main() {
    println!("{}", Solution::can_partition(vec![1, 5, 11, 5]));
    println!("{}", Solution::can_partition(vec![1, 2, 3, 5]));
    println!("{}", Solution::can_partition(vec![1, 2, 5]));
    println!("{}", Solution::can_partition(vec![3, 3, 3, 4, 5]));
}
