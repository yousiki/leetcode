/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 *
 * https://leetcode.cn/problems/squares-of-a-sorted-array/description/
 *
 * algorithms
 * Easy (68.86%)
 * Likes:    1129
 * Dislikes: 0
 * Total Accepted:    890.2K
 * Total Submissions: 1.3M
 * Testcase Example:  '[-4,-1,0,3,10]'
 *
 * Given an integer array nums sorted in non-decreasing order, return an array
 * of the squares of each number sorted in non-decreasing order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-4,-1,0,3,10]
 * Output: [0,1,9,16,100]
 * Explanation: After squaring, the array becomes [16,1,0,9,100].
 * After sorting, it becomes [0,1,9,16,100].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-7,-3,2,3,11]
 * Output: [4,9,9,49,121]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums is sorted in non-decreasing order.
 *
 *
 *
 * Follow up: Squaring each element and sorting the new array is very trivial,
 * could you find an O(n) solution using a different approach?
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let nums = nums.iter().map(|x| x * x).collect::<Vec<i32>>();
        let mut result = Vec::with_capacity(nums.len());
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            if nums[left] > nums[right - 1] {
                result.push(nums[left]);
                left += 1;
            } else {
                result.push(nums[right - 1]);
                right -= 1;
            }
        }
        result.reverse();
        result
    }
}
// @lc code=end

#[cfg(test)]
mod test_squares_of_a_sorted_array {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let result = Solution::sorted_squares(nums);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn test2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let result = Solution::sorted_squares(nums);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);
    }
}
