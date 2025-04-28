/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 *
 * https://leetcode.cn/problems/squares-of-a-sorted-array/description/
 *
 * algorithms
 * Easy (68.29%)
 * Likes:    1028
 * Dislikes: 0
 * Total Accepted:    731.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '[-4,-1,0,3,10]'
 *
 * 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-4,-1,0,3,10]
 * 输出：[0,1,9,16,100]
 * 解释：平方后，数组变为 [16,1,0,9,100]
 * 排序后，数组变为 [0,1,9,16,100]
 *
 * 示例 2：
 *
 *
 * 输入：nums = [-7,-3,2,3,11]
 * 输出：[4,9,9,49,121]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 10^4
 * -10^4
 * nums 已按 非递减顺序 排序
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 请你设计时间复杂度为 O(n) 的算法解决本问题
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let nums: Vec<i32> = nums.into_iter().map(|x| x.pow(2)).collect();
        let mut results: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut j = nums.len();
        while i < j {
            if nums[i] > nums[j - 1] {
                results.insert(0, nums[i]);
                i += 1;
            } else {
                results.insert(0, nums[j - 1]);
                j -= 1;
            }
        }
        results
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let expected = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), expected);
    }

    #[test]
    fn case2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let expected = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), expected);
    }

    #[test]
    fn case3() {
        let nums = vec![-7, -3, -2, 3, 11];
        let expected = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), expected);
    }
}

fn main() {
    println!("{:?}", Solution::sorted_squares(vec![-4, -1, 0, 3, 10]));
    println!("{:?}", Solution::sorted_squares(vec![-7, -3, 2, 3, 11]));
    println!("{:?}", Solution::sorted_squares(vec![-7, -3, -2, 3, 11]));
}
