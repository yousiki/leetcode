/*
 * @lc app=leetcode.cn id=2552 lang=rust
 *
 * [2552] 统计上升四元组
 *
 * https://leetcode.cn/problems/count-increasing-quadruplets/description/
 *
 * algorithms
 * Hard (40.34%)
 * Likes:    74
 * Dislikes: 0
 * Total Accepted:    10.4K
 * Total Submissions: 21.7K
 * Testcase Example:  '[1,3,2,4,5]'
 *
 * 给你一个长度为 n 下标从 0 开始的整数数组 nums ，它包含 1 到 n 的所有数字，请你返回上升四元组的数目。
 *
 * 如果一个四元组 (i, j, k, l) 满足以下条件，我们称它是上升的：
 *
 *
 * 0 <= i < j < k < l < n 且
 * nums[i] < nums[k] < nums[j] < nums[l] 。
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：nums = [1,3,2,4,5]
 * 输出：2
 * 解释：
 * - 当 i = 0 ，j = 1 ，k = 2 且 l = 3 时，有 nums[i] < nums[k] < nums[j] < nums[l] 。
 * - 当 i = 0 ，j = 1 ，k = 2 且 l = 4 时，有 nums[i] < nums[k] < nums[j] < nums[l] 。
 * 没有其他的四元组，所以我们返回 2 。
 *
 *
 * 示例 2：
 *
 * 输入：nums = [1,2,3,4]
 * 输出：0
 * 解释：只存在一个四元组 i = 0 ，j = 1 ，k = 2 ，l = 3 ，但是 nums[j] < nums[k] ，所以我们返回 0
 * 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 4 <= nums.length <= 4000
 * 1 <= nums[i] <= nums.length
 * nums 中所有数字 互不相同 ，nums 是一个排列。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let mut possible_l = vec![vec![0; nums.len()]; nums.len()];
        for j in 1..nums.len() - 2 {
            let mut p = 0i64;
            for l in (j + 2..nums.len()).rev() {
                if nums[j] < nums[l] {
                    p += 1;
                }
                possible_l[j][l - 1] = p;
            }
        }
        let mut possible_i = vec![vec![0; nums.len()]; nums.len()];
        for k in 2..nums.len() - 1 {
            let mut p = 0i64;
            for i in 0..k - 1 {
                if nums[i] < nums[k] {
                    p += 1;
                }
                possible_i[i + 1][k] = p;
            }
        }
        let mut ans = 0i64;
        for j in 1..nums.len() - 2 {
            for k in j + 1..nums.len() - 1 {
                if nums[k] < nums[j] {
                    ans += possible_i[j][k] * possible_l[j][k];
                }
            }
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_quadruplets(vec![1, 3, 2, 4, 5]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 4]), 0);
    }
}

fn main() {
    println!("{}", Solution::count_quadruplets(vec![1, 3, 2, 4, 5]));
    println!("{}", Solution::count_quadruplets(vec![1, 2, 3, 4]));
}
