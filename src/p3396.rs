/*
 * @lc app=leetcode.cn id=3396 lang=rust
 *
 * [3396] 使数组元素互不相同所需的最少操作次数
 *
 * https://leetcode.cn/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/description/
 *
 * algorithms
 * Easy (71.73%)
 * Likes:    26
 * Dislikes: 0
 * Total Accepted:    22.1K
 * Total Submissions: 30.9K
 * Testcase Example:  '[1,2,3,4,2,3,3,5,7]'
 *
 * 给你一个整数数组 nums，你需要确保数组中的元素 互不相同 。为此，你可以执行以下操作任意次：
 *
 *
 * 从数组的开头移除 3 个元素。如果数组中元素少于 3 个，则移除所有剩余元素。
 *
 *
 * 注意：空数组也视作为数组元素互不相同。返回使数组元素互不相同所需的 最少操作次数 。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入： nums = [1,2,3,4,2,3,3,5,7]
 *
 * 输出： 2
 *
 * 解释：
 *
 *
 * 第一次操作：移除前 3 个元素，数组变为 [4, 2, 3, 3, 5, 7]。
 * 第二次操作：再次移除前 3 个元素，数组变为 [3, 5, 7]，此时数组中的元素互不相同。
 *
 *
 * 因此，答案是 2。
 *
 *
 * 示例 2：
 *
 *
 * 输入： nums = [4,5,6,4,4]
 *
 * 输出： 2
 *
 * 解释：
 *
 *
 * 第一次操作：移除前 3 个元素，数组变为 [4, 4]。
 * 第二次操作：移除所有剩余元素，数组变为空。
 *
 *
 * 因此，答案是 2。
 *
 *
 * 示例 3：
 *
 *
 * 输入： nums = [6,7,8,9]
 *
 * 输出： 0
 *
 * 解释：
 *
 * 数组中的元素已经互不相同，因此不需要进行任何操作，答案是 0。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * 1 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::<i32>::new();
        for num in nums.iter().rev() {
            if set.contains(num) {
                break;
            } else {
                set.insert(*num);
            }
        }
        let max_distinct = set.len() as i32;
        let min_remove = nums.len() as i32 - max_distinct;
        let min_operations = (min_remove + 2) / 3;
        return min_operations;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]),
            2
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::minimum_operations(vec![4, 5, 6, 4, 4]), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::minimum_operations(vec![6, 7, 8, 9]), 0);
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7])
    );
    println!("{}", Solution::minimum_operations(vec![4, 5, 6, 4, 4]));
    println!("{}", Solution::minimum_operations(vec![6, 7, 8, 9]));
}
