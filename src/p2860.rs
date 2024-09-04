/*
 * @lc app=leetcode.cn id=2860 lang=rust
 *
 * [2860] 让所有学生保持开心的分组方法数
 *
 * https://leetcode.cn/problems/happy-students/description/
 *
 * algorithms
 * Medium (56.57%)
 * Likes:    62
 * Dislikes: 0
 * Total Accepted:    17.3K
 * Total Submissions: 26.2K
 * Testcase Example:  '[1,1]'
 *
 * 给你一个下标从 0 开始、长度为 n 的整数数组 nums ，其中 n 是班级中学生的总数。班主任希望能够在让所有学生保持开心的情况下选出一组学生：
 *
 * 如果能够满足下述两个条件之一，则认为第 i 位学生将会保持开心：
 *
 *
 * 这位学生被选中，并且被选中的学生人数 严格大于 nums[i] 。
 * 这位学生没有被选中，并且被选中的学生人数 严格小于 nums[i] 。
 *
 *
 * 返回能够满足让所有学生保持开心的分组方法的数目。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,1]
 * 输出：2
 * 解释：
 * 有两种可行的方法：
 * 班主任没有选中学生。
 * 班主任选中所有学生形成一组。
 * 如果班主任仅选中一个学生来完成分组，那么两个学生都无法保持开心。因此，仅存在两种可行的方法。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [6,0,3,3,6,7,2,7]
 * 输出：3
 * 解释：
 * 存在三种可行的方法：
 * 班主任选中下标为 1 的学生形成一组。
 * 班主任选中下标为 1、2、3、6 的学生形成一组。
 * 班主任选中所有学生形成一组。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] < nums.length
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut ans = 0;
        // No student is selected
        if *nums.first().unwrap() > 0 {
            ans += 1;
        }
        // All students are selected
        if *nums.last().unwrap() < nums.len() as i32 {
            ans += 1;
        }
        // Some students are selected
        ans += nums
            .windows(2)
            .enumerate()
            .filter(|(i, n)| (n[0] < (i + 1) as i32) && (n[1] > (i + 1) as i32))
            .count() as i32;
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_ways(vec![1, 1]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
    }
}

fn main() {
    println!("{}", Solution::count_ways(vec![1, 1]));
    println!("{}", Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]));
}
