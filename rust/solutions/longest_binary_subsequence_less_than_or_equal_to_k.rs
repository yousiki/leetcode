/*
 * @lc app=leetcode.cn id=2311 lang=rust
 *
 * [2311] Longest Binary Subsequence Less Than or Equal to K
 *
 * https://leetcode.cn/problems/longest-binary-subsequence-less-than-or-equal-to-k/description/
 *
 * algorithms
 * Medium (46.90%)
 * Likes:    62
 * Dislikes: 0
 * Total Accepted:    19.6K
 * Total Submissions: 41.8K
 * Testcase Example:  '"1001010"\n5'
 *
 * You are given a binary string s and a positive integer k.
 *
 * Return the length of the longest subsequence of s that makes up a binary
 * number less than or equal to k.
 *
 * Note:
 *
 *
 * The subsequence can contain leading zeroes.
 * The empty string is considered to be equal to 0.
 * A subsequence is a string that can be derived from another string by
 * deleting some or no characters without changing the order of the remaining
 * characters.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "1001010", k = 5
 * Output: 5
 * Explanation: The longest subsequence of s that makes up a binary number less
 * than or equal to 5 is "00010", as this number is equal to 2 in decimal.
 * Note that "00100" and "00101" are also possible, which are equal to 4 and 5
 * in decimal, respectively.
 * The length of this subsequence is 5, so 5 is returned.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "00101001", k = 1
 * Output: 6
 * Explanation: "000001" is the longest subsequence of s that makes up a binary
 * number less than or equal to 1, as this number is equal to 1 in decimal.
 * The length of this subsequence is 6, so 6 is returned.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s[i] is either '0' or '1'.
 * 1 <= k <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut length = 0;
        let mut value = 0;
        for c in s.chars().rev() {
            match c {
                '0' => {
                    length += 1;
                }
                '1' => {
                    if length < 31 && value + (1 << length) <= k {
                        value += 1 << length;
                        length += 1;
                    }
                }
                _ => {}
            }
        }
        length
    }
}
// @lc code=end

#[cfg(test)]
mod test_longest_binary_subsequence_less_than_or_equal_to_k {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_subsequence("1001010".to_string(), 5), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_subsequence("00101001".to_string(), 1), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_subsequence("111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101".to_string(), 11713332), 96);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::longest_subsequence("100110111111000000010011101000111011000001000111010001010111100001111110110010100011100100111000011011000000100001011000000100110110001101011010011".to_string(), 522399436), 92);
    }
}
