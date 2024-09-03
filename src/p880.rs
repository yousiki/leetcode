/*
 * @lc app=leetcode.cn id=880 lang=rust
 *
 * [880] 索引处的解码字符串
 *
 * https://leetcode.cn/problems/decoded-string-at-index/description/
 *
 * algorithms
 * Medium (27.81%)
 * Likes:    199
 * Dislikes: 0
 * Total Accepted:    9.8K
 * Total Submissions: 35.3K
 * Testcase Example:  '"leet2code3"\n10'
 *
 * 给定一个编码字符串 s 。请你找出 解码字符串 并将其写入磁带。解码时，从编码字符串中 每次读取一个字符 ，并采取以下步骤：
 *
 *
 * 如果所读的字符是字母，则将该字母写在磁带上。
 * 如果所读的字符是数字（例如 d），则整个当前磁带总共会被重复写 d-1 次。
 *
 *
 * 现在，对于给定的编码字符串 s 和索引 k，查找并返回解码字符串中的第 k 个字母。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "leet2code3", k = 10
 * 输出："o"
 * 解释：
 * 解码后的字符串为 "leetleetcodeleetleetcodeleetleetcode"。
 * 字符串中的第 10 个字母是 "o"。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "ha22", k = 5
 * 输出："h"
 * 解释：
 * 解码后的字符串为 "hahahaha"。第 5 个字母是 "h"。
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "a2345678999999999999999", k = 1
 * 输出："a"
 * 解释：
 * 解码后的字符串为 "a" 重复 8301530446056247680 次。第 1 个字母是 "a"。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= s.length <= 100
 * s 只包含小写字母与数字 2 到 9 。
 * s 以字母开头。
 * 1 <= k <= 10^9
 * 题目保证 k 小于或等于解码字符串的长度。
 * 解码后的字符串保证少于 2^63 个字母。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        s.chars()
            .fold(Vec::new(), |mut p, c| {
                if c.is_digit(10) {
                    p.push(*p.last().unwrap_or(&0) * (c.to_digit(10).unwrap() as i64));
                } else {
                    p.push(*p.last().unwrap_or(&0) + 1);
                }
                p
            })
            .windows(2)
            .rev()
            .zip(s.chars().rev())
            .fold((k as i64, s.chars().nth(0).unwrap()), |(k, a), (p, c)| {
                if c.is_digit(10) {
                    ((k - 1) % p[0] + 1, a)
                } else if k == p[1] {
                    (0, c)
                } else {
                    (k, a)
                }
            })
            .1
            .to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::decode_at_index("leet2code3".to_string(), 10),
            "o".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::decode_at_index("ha22".to_string(), 5),
            "h".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::decode_at_index("a2345678999999999999999".to_string(), 1),
            "a".to_string()
        );
    }
}

fn main() {
    println!(
        "{}",
        Solution::decode_at_index("leet2code3".to_string(), 10)
    );
    println!("{}", Solution::decode_at_index("ha22".to_string(), 5));
    println!(
        "{}",
        Solution::decode_at_index("a2345678999999999999999".to_string(), 1)
    );
}
