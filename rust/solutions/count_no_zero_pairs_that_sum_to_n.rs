/*
 * @lc app=leetcode id=3704 lang=rust
 *
 * [3704] Count No-Zero Pairs That Sum to N
 *
 * https://leetcode.com/problems/count-no-zero-pairs-that-sum-to-n/description/
 *
 * algorithms
 * Hard (9.69%)
 * Likes:    19
 * Dislikes: 1
 * Total Accepted:    2.3K
 * Total Submissions: 24.1K
 * Testcase Example:  '2'
 *
 * A no-zero integer is a positive integer that does not contain the digit 0 in
 * its decimal representation.
 *
 * Given an integer n, count the number of pairs (a, b) where:
 *
 *
 * a and b are no-zero integers.
 * a + b = n
 *
 *
 * Return an integer denoting the number of such pairs.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 2
 *
 * Output: 1
 *
 * Explanation:
 *
 * The only pair is (1, 1).
 *
 *
 * Example 2:
 *
 *
 * Input: n = 3
 *
 * Output: 2
 *
 * Explanation:
 *
 * The pairs are (1, 2) and (2, 1).
 *
 *
 * Example 3:
 *
 *
 * Input: n = 11
 *
 * Output: 8
 *
 * Explanation:
 *
 * The pairs are (2, 9), (3, 8), (4, 7), (5, 6), (6, 5), (7, 4), (8, 3), and
 * (9, 2). Note that (1, 10) and (10, 1) do not satisfy the conditions because
 * 10 contains 0 in its decimal representation.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= n <= 10^15
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_no_zero_pairs(n: i64) -> i64 {
        let digits = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let mut dp = vec![vec![0; 8]; digits.len() + 1];
        dp[0][Self::encode(false, true, true)] = 1;
        for pos in 1..=digits.len() {
            let digit = digits[pos - 1];
            for code in 0..8 {
                let (carry, ended_a, ended_b) = Self::decode(code);
                for digit_a in if ended_a { 0..=0 } else { 1..=9 } {
                    for digit_b in if ended_b { 0..=0 } else { 1..=9 } {
                        let digit_sum = digit_a + digit_b + carry as usize;
                        if digit_sum % 10 == digit {
                            let next_carry = digit_sum >= 10;
                            for next_ended_a in (ended_a || digit_a == 0) as usize..=1 {
                                for next_ended_b in (ended_b || digit_b == 0) as usize..=1 {
                                    let next_code = Self::encode(
                                        next_carry,
                                        next_ended_a == 1,
                                        next_ended_b == 1,
                                    );
                                    dp[pos][code] += dp[pos - 1][next_code];
                                }
                            }
                        }
                    }
                }
            }
        }
        dp[digits.len()][Self::encode(false, false, false)]
    }

    fn encode(carry: bool, ended_a: bool, ended_b: bool) -> usize {
        (carry as usize) + ((ended_a as usize) << 1) + ((ended_b as usize) << 2)
    }

    fn decode(code: usize) -> (bool, bool, bool) {
        (code & 1 == 1, (code >> 1) & 1 == 1, (code >> 2) & 1 == 1)
    }
}
// @lc code=end

#[cfg(test)]
mod test_count_no_zero_pairs_that_sum_to_n {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_no_zero_pairs(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_no_zero_pairs(3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_no_zero_pairs(11), 8);
    }
}
