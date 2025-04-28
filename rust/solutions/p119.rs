/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 *
 * https://leetcode.cn/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (69.44%)
 * Likes:    568
 * Dislikes: 0
 * Total Accepted:    333.1K
 * Total Submissions: 479.8K
 * Testcase Example:  '3'
 *
 * 给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。
 *
 * 在「杨辉三角」中，每个数是它左上方和右上方的数的和。
 *
 *
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: rowIndex = 3
 * 输出: [1,3,3,1]
 *
 *
 * 示例 2:
 *
 *
 * 输入: rowIndex = 0
 * 输出: [1]
 *
 *
 * 示例 3:
 *
 *
 * 输入: rowIndex = 1
 * 输出: [1,1]
 *
 *
 *
 *
 * 提示:
 *
 *
 * 0
 *
 *
 *
 *
 * 进阶：
 *
 * 你可以优化你的算法到 O(rowIndex) 空间复杂度吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![0; (row_index + 1) as usize];
        res[0] = 1;
        for i in 1..=row_index as usize {
            for j in (1..=i).rev() {
                res[j] += res[j - 1];
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}

fn main() {
    println!("{:?}", Solution::get_row(3)); // [1,3,3,1]
    println!("{:?}", Solution::get_row(0)); // [1]
    println!("{:?}", Solution::get_row(1)); // [1,1]
}
