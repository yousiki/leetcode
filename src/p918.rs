/*
 * @lc app=leetcode.cn id=918 lang=rust
 *
 * [918] 环形子数组的最大和
 *
 * https://leetcode.cn/problems/maximum-sum-circular-subarray/description/
 *
 * algorithms
 * Medium (41.07%)
 * Likes:    678
 * Dislikes: 0
 * Total Accepted:    80.4K
 * Total Submissions: 195.3K
 * Testcase Example:  '[1,-2,3,-2]'
 *
 * 给定一个长度为 n 的环形整数数组 nums ，返回 nums 的非空 子数组 的最大可能和 。
 *
 * 环形数组 意味着数组的末端将会与开头相连呈环状。形式上， nums[i] 的下一个元素是 nums[(i + 1) % n] ， nums[i]
 * 的前一个元素是 nums[(i - 1 + n) % n] 。
 *
 * 子数组 最多只能包含固定缓冲区 nums 中的每个元素一次。形式上，对于子数组 nums[i], nums[i + 1], ..., nums[j]
 * ，不存在 i <= k1, k2 <= j 其中 k1 % n == k2 % n 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,-2,3,-2]
 * 输出：3
 * 解释：从子数组 [3] 得到最大和 3
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [5,-3,5]
 * 输出：10
 * 解释：从子数组 [5,5] 得到最大和 5 + 5 = 10
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [3,-2,2,-3]
 * 输出：3
 * 解释：从子数组 [3] 和 [3,-2,2] 都可以得到最大和 3
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == nums.length
 * 1 <= n <= 3 * 10^4
 * -3 * 10^4 <= nums[i] <= 3 * 10^4​​​​​​​
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let n = nums.len();
        // Double the array.
        let mut nums = nums.clone();
        nums.extend(nums.clone());
        // Calculate the prefix sum.
        let sums = nums
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<i32>>();
        // Create a queue for minimum prefix sum and index.
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        let mut ans = i32::MIN;
        for i in 0..sums.len() {
            while !queue.is_empty() && queue.front().unwrap().0 + n < i {
                queue.pop_front();
            }
            if !queue.is_empty() {
                ans = ans.max(sums[i] - queue.front().unwrap().1);
            }
            while !queue.is_empty() && sums[i] <= queue.back().unwrap().1 {
                queue.pop_back();
            }
            queue.push_back((i, sums[i]));
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
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, -3]), 3);
    }
}

fn main() {
    let nums = vec![1, -2, 3, -2];
    println!("{}", Solution::max_subarray_sum_circular(nums));

    let nums = vec![5, -3, 5];
    println!("{}", Solution::max_subarray_sum_circular(nums));

    let nums = vec![3, -2, 2, -3];
    println!("{}", Solution::max_subarray_sum_circular(nums));
}
