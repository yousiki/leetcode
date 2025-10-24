/*
 * @lc app=leetcode id=2048 lang=rust
 *
 * [2048] Next Greater Numerically Balanced Number
 *
 * https://leetcode.com/problems/next-greater-numerically-balanced-number/description/
 *
 * algorithms
 * Medium (59.30%)
 * Likes:    382
 * Dislikes: 322
 * Total Accepted:    53.8K
 * Total Submissions: 90.8K
 * Testcase Example:  '1'
 *
 * An integer x is numerically balanced if for every digit d in the number x,
 * there are exactly d occurrences of that digit in x.
 *
 * Given an integer n, return the smallest numerically balanced number strictly
 * greater than n.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 1
 * Output: 22
 * Explanation:
 * 22 is numerically balanced since:
 * - The digit 2 occurs 2 times.
 * It is also the smallest numerically balanced number strictly greater than
 * 1.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 1000
 * Output: 1333
 * Explanation:
 * 1333 is numerically balanced since:
 * - The digit 1 occurs 1 time.
 * - The digit 3 occurs 3 times.
 * It is also the smallest numerically balanced number strictly greater than
 * 1000.
 * Note that 1022 cannot be the answer because 0 appeared more than 0 times.
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3000
 * Output: 3133
 * Explanation:
 * 3133 is numerically balanced since:
 * - The digit 1 occurs 1 time.
 * - The digit 3 occurs 3 times.
 * It is also the smallest numerically balanced number strictly greater than
 * 3000.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= n <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        for i in n + 1..=10000000 {
            if Self::is_beautiful_number(i) {
                return i;
            }
        }
        panic!()
    }

    fn is_beautiful_number(x: i32) -> bool {
        let mut cnt = [0; 10];
        for c in x.to_string().chars() {
            let c = c.to_digit(10).unwrap() as usize;
            cnt[c] += 1;
            if cnt[c] > c {
                return false;
            }
        }
        for c in 0..10 {
            if cnt[c] != 0 && cnt[c] != c {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod test_next_greater_numberically_balanced_number {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::next_beautiful_number(1), 22);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::next_beautiful_number(1000), 1333);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::next_beautiful_number(3000), 3133);
    }
}
