/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 *
 * https://leetcode.cn/problems/contains-duplicate-ii/description/
 *
 * algorithms
 * Easy (47.51%)
 * Likes:    764
 * Dislikes: 0
 * Total Accepted:    361.5K
 * Total Submissions: 742.9K
 * Testcase Example:  '[1,2,3,1]\n3'
 *
 * 给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，满足 nums[i] == nums[j] 且
 * abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3,1], k = 3
 * 输出：true
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,0,1,1], k = 1
 * 输出：true
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,2,3,1,2,3], k = 2
 * 输出：false
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 0 <= k <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                if (i as i32 - j) <= k {
                    return true;
                }
            }
            map.insert(num, i as i32);
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
    );
    println!(
        "{:?}",
        Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
    );
    println!(
        "{:?}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2)
    );
}
