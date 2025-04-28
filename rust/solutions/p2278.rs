/*
 * @lc app=leetcode.cn id=2278 lang=rust
 *
 * [2278] 字母在字符串中的百分比
 *
 * https://leetcode.cn/problems/percentage-of-letter-in-string/description/
 *
 * algorithms
 * Easy (82.76%)
 * Likes:    37
 * Dislikes: 0
 * Total Accepted:    34.8K
 * Total Submissions: 42K
 * Testcase Example:  '"foobar"\n"o"'
 *
 * 给你一个字符串 s 和一个字符 letter ，返回在 s 中等于 letter 字符所占的 百分比 ，向下取整到最接近的百分比。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "foobar", letter = "o"
 * 输出：33
 * 解释：
 * 等于字母 'o' 的字符在 s 中占到的百分比是 2 / 6 * 100% = 33% ，向下取整，所以返回 33 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "jjjj", letter = "k"
 * 输出：0
 * 解释：
 * 等于字母 'k' 的字符在 s 中占到的百分比是 0% ，所以返回 0 。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 100
 * s 由小写英文字母组成
 * letter 是一个小写英文字母
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let total_chars = s.len() as i32;
        let letter_count = s.chars().filter(|&c| c == letter).count() as i32;
        
        // Calculate percentage and floor the result
        (letter_count * 100) / total_chars
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}

fn main() {
    println!("{}", Solution::percentage_letter("foobar".to_string(), 'o'));
    println!("{}", Solution::percentage_letter("jjjj".to_string(), 'k'));
}

