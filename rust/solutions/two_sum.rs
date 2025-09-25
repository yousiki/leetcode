/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode.cn/problems/two-sum/description/
 *
 * algorithms
 * Easy (55.08%)
 * Likes:    20146
 * Dislikes: 0
 * Total Accepted:    6.8M
 * Total Submissions: 12.3M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers nums and an integer target, return indices of the
 * two numbers such that they add up to target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * You can return the answer in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * Only one valid answer exists.
 *
 *
 *
 * Follow-up: Can you come up with an algorithm that is less than O(n^2) time
 * complexity?
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        panic!("No two sum solution");
    }
}
// @lc code=end

#[cfg(test)]
mod test_two_sum {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), result);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), result);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), result);
    }
}
