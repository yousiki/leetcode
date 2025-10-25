/*
 * @lc app=leetcode id=1716 lang=rust
 *
 * [1716] Calculate Money in Leetcode Bank
 *
 * https://leetcode.com/problems/calculate-money-in-leetcode-bank/description/
 *
 * algorithms
 * Easy (78.67%)
 * Likes:    1607
 * Dislikes: 59
 * Total Accepted:    208.6K
 * Total Submissions: 260.4K
 * Testcase Example:  '4'
 *
 * Hercy wants to save money for his first car. He puts money in the Leetcode
 * bank every day.
 *
 * He starts by putting in $1 on Monday, the first day. Every day from Tuesday
 * to Sunday, he will put in $1 more than the day before. On every subsequent
 * Monday, he will put in $1 more than the previous Monday.
 *
 * Given n, return the total amount of money he will have in the Leetcode bank
 * at the end of the n^th day.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 4
 * Output: 10
 * Explanation: After the 4^th day, the total is 1 + 2 + 3 + 4 = 10.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 10
 * Output: 37
 * Explanation: After the 10^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) +
 * (2 + 3 + 4) = 37. Notice that on the 2^nd Monday, Hercy only puts in $2.
 *
 *
 * Example 3:
 *
 *
 * Input: n = 20
 * Output: 96
 * Explanation: After the 20^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) +
 * (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.
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
    pub fn total_money(n: i32) -> i32 {
        let mut sum = 0;
        for day in 1..=n {
            let week = (day - 1) / 7;
            let today = week + (day - week * 7);
            sum += today;
        }
        sum
    }
}
// @lc code=end

#[cfg(test)]
mod test_calculate_money_in_leetcode_bank {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::total_money(20), 96);
    }
}
