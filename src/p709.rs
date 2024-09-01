/*
 * @lc app=leetcode.cn id=709 lang=rust
 *
 * [709] 转换成小写字母
 *
 * https://leetcode.cn/problems/to-lower-case/description/
 *
 * algorithms
 * Easy (76.50%)
 * Likes:    242
 * Dislikes: 0
 * Total Accepted:    156.3K
 * Total Submissions: 204.3K
 * Testcase Example:  '"Hello"'
 *
 * 给你一个字符串 s ，将该字符串中的大写字母转换成相同的小写字母，返回新的字符串。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "Hello"
 * 输出："hello"
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "here"
 * 输出："here"
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "LOVELY"
 * 输出："lovely"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * s 由 ASCII 字符集中的可打印字符组成
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::to_lower_case("Hello".to_string()),
            "hello".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::to_lower_case("here".to_string()),
            "here".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::to_lower_case("LOVELY".to_string()),
            "lovely".to_string()
        );
    }
}

fn main() {
    println!("{}", Solution::to_lower_case("Hello".to_string()));
    println!("{}", Solution::to_lower_case("here".to_string()));
    println!("{}", Solution::to_lower_case("LOVELY".to_string()));
}
