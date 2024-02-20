/*
 * @lc app=leetcode.cn id=2148 lang=rust
 *
 * [2148] 元素计数
 *
 * https://leetcode.cn/problems/count-elements-with-strictly-smaller-and-greater-elements/description/
 *
 * algorithms
 * Easy (57.37%)
 * Likes:    25
 * Dislikes: 0
 * Total Accepted:    14.8K
 * Total Submissions: 25.8K
 * Testcase Example:  '[11,7,2,15]'
 *
 * 给你一个整数数组 nums ，统计并返回在 nums 中同时至少具有一个严格较小元素和一个严格较大元素的元素数目。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [11,7,2,15]
 * 输出：2
 * 解释：元素 7 ：严格较小元素是元素 2 ，严格较大元素是元素 11 。
 * 元素 11 ：严格较小元素是元素 7 ，严格较大元素是元素 15 。
 * 总计有 2 个元素都满足在 nums 中同时存在一个严格较小元素和一个严格较大元素。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [-3,3,3,90]
 * 输出：2
 * 解释：元素 3 ：严格较小元素是元素 -3 ，严格较大元素是元素 90 。
 * 由于有两个元素的值为 3 ，总计有 2 个元素都满足在 nums 中同时存在一个严格较小元素和一个严格较大元素。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        // Sort nums in increasing order.
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] != nums[0] && nums[i] != nums[nums.len() - 1] {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
    assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
}
