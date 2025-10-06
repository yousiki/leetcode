/*
 * @lc app=leetcode id=3702 lang=rust
 *
 * [3702] Longest Subsequence With Non-Zero Bitwise XOR
 *
 * https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor/description/
 *
 * algorithms
 * Medium (35.19%)
 * Likes:    56
 * Dislikes: 3
 * Total Accepted:    22.2K
 * Total Submissions: 63K
 * Testcase Example:  '[1,2,3]'
 *
 * You are given an integer array nums.
 *
 * Return the length of the longest subsequence in nums whose bitwise XOR is
 * non-zero. If no such subsequence exists, return 0.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3]
 *
 * Output: 2
 *
 * Explanation:
 *
 * One longest subsequence is [2, 3]. The bitwise XOR is computed as 2 XOR 3 =
 * 1, which is non-zero.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,3,4]
 *
 * Output: 3
 *
 * Explanation:
 *
 * The longest subsequence is [2, 3, 4]. The bitwise XOR is computed as 2 XOR 3
 * XOR 4 = 5, which is non-zero.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let (xor_sum, has_non_zero) =
            nums.iter().fold((0, false), |(xor_sum, has_non_zero), &x| {
                (xor_sum ^ x, has_non_zero || (x != 0))
            });
        if xor_sum != 0 {
            nums.len() as i32
        } else if has_non_zero {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test_longest_subsequence_with_non_zero_bitwise_xor {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3);
    }
}
