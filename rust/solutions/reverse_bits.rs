/*
 * @lc app=leetcode id=190 lang=rust
 *
 * [190] Reverse Bits
 *
 * https://leetcode.com/problems/reverse-bits/description/
 *
 * algorithms
 * Easy (65.79%)
 * Likes:    5624
 * Dislikes: 1649
 * Total Accepted:    1.2M
 * Total Submissions: 1.8M
 * Testcase Example:  '43261596'
 *
 * Reverse bits of a given 32 bits signed integer.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 43261596
 *
 * Output: 964176192
 *
 * Explanation:
 *
 *
 *
 *
 * Integer
 * Binary
 *
 *
 * 43261596
 * 00000010100101000001111010011100
 *
 *
 * 964176192
 * 00111001011110000010100101000000
 *
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input: n = 2147483644
 *
 * Output: 1073741822
 *
 * Explanation:
 *
 *
 *
 *
 * Integer
 * Binary
 *
 *
 * 2147483644
 * 01111111111111111111111111111100
 *
 *
 * 1073741822
 * 00111111111111111111111111111110
 *
 *
 *
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= n <= 2^31 - 2
 * n is even.
 *
 *
 *
 * Follow up: If this function is called many times, how would you optimize it?
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        assert_eq!(n % 2, 0); // n must be even to avoid negative
        let mut m: i32 = 0;
        for i in 0..32 {
            if (n >> i) & 1 == 1 {
                m += 1 << (31 - i);
            }
        }
        return m;
    }
}
// @lc code=end

#[cfg(test)]
mod test_reverse_bits {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse_bits(2147483644), 1073741822);
    }
}
