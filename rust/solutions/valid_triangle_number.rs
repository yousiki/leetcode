/*
 * @lc app=leetcode.cn id=611 lang=rust
 *
 * [611] Valid Triangle Number
 *
 * https://leetcode.cn/problems/valid-triangle-number/description/
 *
 * algorithms
 * Medium (54.86%)
 * Likes:    667
 * Dislikes: 0
 * Total Accepted:    140.2K
 * Total Submissions: 254.4K
 * Testcase Example:  '[2,2,3,4]'
 *
 * Given an integer array nums, return the number of triplets chosen from the
 * array that can make triangles if we take them as side lengths of a
 * triangle.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,2,3,4]
 * Output: 3
 * Explanation: Valid combinations are:
 * 2,3,4 (using the first 2)
 * 2,3,4 (using the second 2)
 * 2,2,3
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,2,3,4]
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 1000
 * 0 <= nums[i] <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums
            .iter()
            .filter(|&&x| x > 0)
            .cloned()
            .collect::<Vec<i32>>();
        if nums.len() < 3 {
            return 0;
        }
        nums.sort();
        let mut answer = 0;
        for i in 0..nums.len() - 2 {
            let mut ptr = i + 1;
            for j in i + 1..nums.len() - 1 {
                while ptr + 1 < nums.len() && nums[ptr + 1] < nums[i] + nums[j] {
                    ptr += 1;
                }
                answer += (ptr - j) as i32;
            }
        }
        answer
    }
}
// @lc code=end

#[cfg(test)]
mod test_valid_triangle_number {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![2, 2, 3, 4];
        let result = Solution::triangle_number(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 2, 3, 4];
        let result = Solution::triangle_number(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let result = Solution::triangle_number(nums);
        assert_eq!(result, 0);
    }
}
