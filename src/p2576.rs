/*
 * @lc app=leetcode.cn id=2576 lang=rust
 *
 * [2576] 求出最多标记下标
 *
 * https://leetcode.cn/problems/find-the-maximum-number-of-marked-indices/description/
 *
 * algorithms
 * Medium (45.71%)
 * Likes:    85
 * Dislikes: 0
 * Total Accepted:    16.2K
 * Total Submissions: 35.4K
 * Testcase Example:  '[3,5,2,4]'
 *
 * 给你一个下标从 0 开始的整数数组 nums 。
 *
 * 一开始，所有下标都没有被标记。你可以执行以下操作任意次：
 *
 *
 * 选择两个 互不相同且未标记 的下标 i 和 j ，满足 2 * nums[i] <= nums[j] ，标记下标 i 和 j 。
 *
 *
 * 请你执行上述操作任意次，返回 nums 中最多可以标记的下标数目。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [3,5,2,4]
 * 输出：2
 * 解释：第一次操作中，选择 i = 2 和 j = 1 ，操作可以执行的原因是 2 * nums[2] <= nums[1] ，标记下标 2 和 1 。
 * 没有其他更多可执行的操作，所以答案为 2 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [9,2,5,4]
 * 输出：4
 * 解释：第一次操作中，选择 i = 3 和 j = 0 ，操作可以执行的原因是 2 * nums[3] <= nums[0] ，标记下标 3 和 0 。
 * 第二次操作中，选择 i = 1 和 j = 2 ，操作可以执行的原因是 2 * nums[1] <= nums[2] ，标记下标 1 和 2 。
 * 没有其他更多可执行的操作，所以答案为 4 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,6,8]
 * 输出：0
 * 解释：没有任何可以执行的操作，所以答案为 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut l = 0;
        let mut r = nums.len() / 2;
        let mut ans = 0;
        while l <= r {
            let mid = (l + r) / 2;
            if Self::check(&nums, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        2 * ans as i32
    }

    fn check(nums: &Vec<i32>, mid: usize) -> bool {
        for (&s, &l) in nums
            .iter()
            .take(mid)
            .zip(nums.iter().skip(nums.len() - mid))
        {
            if 2 * s > l {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 5, 2, 4];
        let ans = 2;
        assert_eq!(Solution::max_num_of_marked_indices(nums), ans);
    }

    #[test]
    fn case2() {
        let nums = vec![9, 2, 5, 4];
        let ans = 4;
        assert_eq!(Solution::max_num_of_marked_indices(nums), ans);
    }

    #[test]
    fn case3() {
        let nums = vec![7, 6, 8];
        let ans = 0;
        assert_eq!(Solution::max_num_of_marked_indices(nums), ans);
    }
}

fn main() {
    println!("{}", Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]));
    println!("{}", Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]));
    println!("{}", Solution::max_num_of_marked_indices(vec![7, 6, 8]));
}
