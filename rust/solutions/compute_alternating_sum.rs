/*
 * @lc app=leetcode id=3701 lang=rust
 *
 * [3701] Compute Alternating Sum
 *
 * https://leetcode.com/problems/compute-alternating-sum/description/
 *
 * algorithms
 * Easy (88.29%)
 * Likes:    23
 * Dislikes: 2
 * Total Accepted:    32K
 * Total Submissions: 36.3K
 * Testcase Example:  '[1,3,5,7]'
 *
 * You are given an integer array nums.
 *
 * The alternating sum of nums is the value obtained by adding elements at even
 * indices and subtracting elements at odd indices. That is, nums[0] - nums[1]
 * + nums[2] - nums[3]...
 *
 * Return an integer denoting the alternating sum of nums.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,3,5,7]
 *
 * Output: -4
 *
 * Explanation:
 *
 *
 * Elements at even indices are nums[0] = 1 and nums[2] = 5 because 0 and 2 are
 * even numbers.
 * Elements at odd indices are nums[1] = 3 and nums[3] = 7 because 1 and 3 are
 * odd numbers.
 * The alternating sum is nums[0] - nums[1] + nums[2] - nums[3] = 1 - 3 + 5 - 7
 * = -4.
 *
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [100]
 *
 * Output: 100
 *
 * Explanation:
 *
 *
 * The only element at even indices is nums[0] = 100 because 0 is an even
 * number.
 * There are no elements on odd indices.
 * The alternating sum is nums[0] = 100.
 *
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 1 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(i, &x)| if i % 2 == 0 { x } else { -x })
            .sum()
    }
}
// @lc code=end

#[cfg(test)]
mod test_compute_alternating_sum {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::alternating_sum(vec![1, 3, 5, 7]), -4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::alternating_sum(vec![100]), 100);
    }
}
