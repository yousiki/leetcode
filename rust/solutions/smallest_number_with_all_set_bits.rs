/*
 * @lc app=leetcode id=3370 lang=rust
 *
 * [3370] Smallest Number With All Set Bits
 *
 * https://leetcode.com/problems/smallest-number-with-all-set-bits/description/
 *
 * algorithms
 * Easy (76.19%)
 * Likes:    155
 * Dislikes: 2
 * Total Accepted:    80K
 * Total Submissions: 101.3K
 * Testcase Example:  '5'
 *
 * You are given a positive number n.
 *
 * Return the smallest number x greater than or equal to n, such that the
 * binary representation of x contains only set bits
 *
 *
 * Example 1:
 *
 *
 * Input: n = 5
 *
 * Output: 7
 *
 * Explanation:
 *
 * The binary representation of 7 is "111".
 *
 *
 * Example 2:
 *
 *
 * Input: n = 10
 *
 * Output: 15
 *
 * Explanation:
 *
 * The binary representation of 15 is "1111".
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3
 *
 * Output: 3
 *
 * Explanation:
 *
 * The binary representation of 3 is "11".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut x = n;
        let mut i = 1;
        while i <= x {
            x = x | i;
            i = i << 1;
        }
        x
    }
}
// @lc code=end

#[cfg(test)]
mod test_smallest_number_with_all_set_bits {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_number(5), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::smallest_number(10), 15);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::smallest_number(3), 3);
    }
}
