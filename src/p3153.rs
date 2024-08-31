/*
 * @lc app=leetcode.cn id=3153 lang=rust
 *
 * [3153] 所有数对中数位差之和
 *
 * https://leetcode.cn/problems/sum-of-digit-differences-of-all-pairs/description/
 *
 * algorithms
 * Medium (55.25%)
 * Likes:    44
 * Dislikes: 0
 * Total Accepted:    16.6K
 * Total Submissions: 30K
 * Testcase Example:  '[13,23,12]'
 *
 * 你有一个数组 nums ，它只包含 正 整数，所有正整数的数位长度都 相同 。
 *
 * 两个整数的 数位差 指的是两个整数 相同 位置上不同数字的数目。
 *
 * 请你返回 nums 中 所有 整数对里，数位差之和。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [13,23,12]
 *
 * 输出：4
 *
 * 解释：
 * 计算过程如下：
 * - 13 和 23 的数位差为 1 。
 * - 13 和 12 的数位差为 1 。
 * - 23 和 12 的数位差为 2 。
 * 所以所有整数数对的数位差之和为 1 + 1 + 2 = 4 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [10,10,10,10]
 *
 * 输出：0
 *
 * 解释：
 * 数组中所有整数都相同，所以所有整数数对的数位不同之和为 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= nums.length <= 10^5
 * 1 <= nums[i] < 10^9
 * nums 中的整数都有相同的数位长度。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        (0..10)
            .fold((0i64, nums), |(ans, nums), _| {
                (
                    ans + ((nums.len() as i64).pow(2)
                        - ((0..10)
                            .into_iter()
                            .map(|i| nums.iter().filter(|&x| x % 10 == i).count() as i64)
                            .map(|n| n * n)
                            .sum::<i64>()))
                        / 2,
                    nums.iter().map(|&x| x / 10).collect(),
                )
            })
            .0
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![13, 23, 12];
        assert_eq!(Solution::sum_digit_differences(nums), 4);
    }

    #[test]
    fn case2() {
        let nums = vec![10, 10, 10, 10];
        assert_eq!(Solution::sum_digit_differences(nums), 0);
    }
}

fn main() {
    let nums = vec![13, 23, 12];
    println!("{}", Solution::sum_digit_differences(nums));
    let nums = vec![10, 10, 10, 10];
    println!("{}", Solution::sum_digit_differences(nums));
}
