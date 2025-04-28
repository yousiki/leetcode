/*
 * @lc app=leetcode.cn id=1641 lang=rust
 *
 * [1641] 统计字典序元音字符串的数目
 *
 * https://leetcode.cn/problems/count-sorted-vowel-strings/description/
 *
 * algorithms
 * Medium (82.49%)
 * Likes:    165
 * Dislikes: 0
 * Total Accepted:    40.5K
 * Total Submissions: 49.1K
 * Testcase Example:  '1'
 *
 * 给你一个整数 n，请返回长度为 n 、仅由元音 (a, e, i, o, u) 组成且按 字典序排列 的字符串数量。
 *
 * 字符串 s 按 字典序排列 需要满足：对于所有有效的 i，s[i] 在字母表中的位置总是与 s[i+1] 相同或在 s[i+1] 之前。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 1
 * 输出：5
 * 解释：仅由元音组成的 5 个字典序字符串为 ["a","e","i","o","u"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 2
 * 输出：15
 * 解释：仅由元音组成的 15 个字典序字符串为
 * ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"]
 * 注意，"ea" 不是符合题意的字符串，因为 'e' 在字母表中的位置比 'a' 靠后
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 33
 * 输出：66045
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    fn combination(n: i32, k: i32) -> i32 {
        let mut res = 1;
        for i in 0..k {
            res *= n - i;
            res /= i + 1;
        }
        res
    }

    pub fn count_vowel_strings(n: i32) -> i32 {
        // \sum_{i=1}^{5} C_{5}^{i} C_{n-1}^{i-1}
        let mut res = 0;
        for i in 1..=5 {
            res += Self::combination(5, i) * Self::combination(n - 1, i - 1);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::count_vowel_strings(50), 316251);
    }
}

fn main() {
    println!("{}", Solution::count_vowel_strings(1));
    println!("{}", Solution::count_vowel_strings(2));
    println!("{}", Solution::count_vowel_strings(33));
    println!("{}", Solution::count_vowel_strings(50));
}
