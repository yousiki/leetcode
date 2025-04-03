/*
 * @lc app=leetcode.cn id=2874 lang=rust
 *
 * [2874] 有序三元组中的最大值 II
 *
 * https://leetcode.cn/problems/maximum-value-of-an-ordered-triplet-ii/description/
 *
 * algorithms
 * Medium (51.41%)
 * Likes:    45
 * Dislikes: 0
 * Total Accepted:    18.6K
 * Total Submissions: 31.4K
 * Testcase Example:  '[12,6,1,2,7]'
 *
 * 给你一个下标从 0 开始的整数数组 nums 。
 *
 * 请你从所有满足 i < j < k 的下标三元组 (i, j, k) 中，找出并返回下标三元组的最大值。如果所有满足条件的三元组的值都是负数，则返回 0
 * 。
 *
 * 下标三元组 (i, j, k) 的值等于 (nums[i] - nums[j]) * nums[k] 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [12,6,1,2,7]
 * 输出：77
 * 解释：下标三元组 (0, 2, 4) 的值是 (nums[0] - nums[2]) * nums[4] = 77 。
 * 可以证明不存在值大于 77 的有序下标三元组。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,10,3,4,19]
 * 输出：133
 * 解释：下标三元组 (1, 2, 4) 的值是 (nums[1] - nums[2]) * nums[4] = 133 。
 * 可以证明不存在值大于 133 的有序下标三元组。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,2,3]
 * 输出：0
 * 解释：唯一的下标三元组 (0, 1, 2) 的值是一个负数，(nums[0] - nums[1]) * nums[2] = -3 。因此，答案是 0
 * 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        // Suffix maximum
        let suffix_maximum = nums
            .iter()
            .rev()
            .fold(Vec::<i32>::new(), |mut acc, &x| {
                if let Some(&last) = acc.last() {
                    acc.push(last.max(x));
                } else {
                    acc.push(x);
                }
                acc
            })
            .iter()
            .rev()
            .copied()
            .collect::<Vec<i32>>();
        // Scan from the beginning
        let (ans, _, _) = nums.iter().zip(suffix_maximum.iter()).fold(
            (0i64, i32::MIN as i64, i32::MIN as i64),
            |(ans, max_sub, max_pre), (&x, &max_suf)| {
                (
                    ans.max(max_sub * max_suf as i64),
                    max_sub.max(max_pre - x as i64),
                    max_pre.max(x as i64),
                )
            },
        );
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![12, 6, 1, 2, 7];
        assert_eq!(Solution::maximum_triplet_value(nums), 77);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 10, 3, 4, 19];
        assert_eq!(Solution::maximum_triplet_value(nums), 133);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::maximum_triplet_value(nums), 0);
    }
}

fn main() {
    println!("{}", Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
    println!("{}", Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
    println!("{}", Solution::maximum_triplet_value(vec![1, 2, 3]));
}
