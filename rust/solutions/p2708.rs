/*
 * @lc app=leetcode.cn id=2708 lang=rust
 *
 * [2708] 一个小组的最大实力值
 *
 * https://leetcode.cn/problems/maximum-strength-of-a-group/description/
 *
 * algorithms
 * Medium (31.03%)
 * Likes:    54
 * Dislikes: 0
 * Total Accepted:    20.2K
 * Total Submissions: 62.7K
 * Testcase Example:  '[3,-1,-5,2,5,-9]'
 *
 * 给你一个下标从 0 开始的整数数组 nums ，它表示一个班级中所有学生在一次考试中的成绩。老师想选出一部分同学组成一个 非空 小组，且这个小组的
 * 实力值 最大，如果这个小组里的学生下标为 i0, i1, i2, ... , ik ，那么这个小组的实力值定义为 nums[i0] * nums[i1]
 * * nums[i2] * ... * nums[ik​] 。
 *
 * 请你返回老师创建的小组能得到的最大实力值为多少。
 *
 *
 *
 * 示例 1：
 *
 * 输入：nums = [3,-1,-5,2,5,-9]
 * 输出：1350
 * 解释：一种构成最大实力值小组的方案是选择下标为 [0,2,3,4,5] 的学生。实力值为 3 * (-5) * 2 * 5 * (-9) = 1350
 * ，这是可以得到的最大实力值。
 *
 *
 * 示例 2：
 *
 * 输入：nums = [-4,-5,-4]
 * 输出：20
 * 解释：选择下标为 [0, 1] 的学生。得到的实力值为 20 。我们没法得到更大的实力值。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 13
 * -9 <= nums[i] <= 9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }
        nums.iter()
            .map(|x| *x as i64)
            .fold((0i64, 0i64), |(max, min), x| {
                (
                    max.max(x).max(x * max).max(x * min),
                    min.min(x).min(x * max).min(x * min),
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
        let nums = vec![3, -1, -5, 2, 5, -9];
        assert_eq!(Solution::max_strength(nums), 1350);
    }

    #[test]
    fn case2() {
        let nums = vec![-4, -5, -4];
        assert_eq!(Solution::max_strength(nums), 20);
    }

    #[test]
    fn case3() {
        let nums = vec![0, -1];
        assert_eq!(Solution::max_strength(nums), 0);
    }
}

fn main() {
    println!("{}", Solution::max_strength(vec![3, -1, -5, 2, 5, -9]));
    println!("{}", Solution::max_strength(vec![-4, -5, -4]));
}
