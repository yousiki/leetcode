/*
 * @lc app=leetcode id=3703 lang=rust
 *
 * [3703] Remove K-Balanced Substrings
 *
 * https://leetcode.com/problems/remove-k-balanced-substrings/description/
 *
 * algorithms
 * Medium (29.98%)
 * Likes:    66
 * Dislikes: 10
 * Total Accepted:    12.1K
 * Total Submissions: 40.4K
 * Testcase Example:  '"(())"\n1'
 *
 * You are given a string s consisting of '(' and ')', and an integer k.
 *
 * A string is k-balanced if it is exactly k consecutive '(' followed by k
 * consecutive ')', i.e., '(' * k + ')' * k.
 *
 * For example, if k = 3, k-balanced is "((()))".
 *
 * You must repeatedly remove all non-overlapping k-balanced substrings from s,
 * and then join the remaining parts. Continue this process until no k-balanced
 * substring exists.
 *
 * Return the final string after all possible removals.
 *
 *
 * ​​​​​​​Example 1:
 *
 *
 * Input: s = "(())", k = 1
 *
 * Output: ""
 *
 * Explanation:
 *
 * k-balanced substring is "()"
 *
 *
 *
 *
 * Step
 * Current s
 * k-balanced
 * Result s
 *
 *
 *
 *
 * 1
 * (())
 * (())
 * ()
 *
 *
 * 2
 * ()
 * ()
 * Empty
 *
 *
 *
 *
 * Thus, the final string is "".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "(()(", k = 1
 *
 * Output: "(("
 *
 * Explanation:
 *
 * k-balanced substring is "()"
 *
 *
 *
 *
 * Step
 * Current s
 * k-balanced
 * Result s
 *
 *
 *
 *
 * 1
 * (()(
 * (()(
 * ((
 *
 *
 * 2
 * ((
 * -
 * ((
 *
 *
 *
 *
 * Thus, the final string is "((".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "((()))()()()", k = 3
 *
 * Output: "()()()"
 *
 * Explanation:
 *
 * k-balanced substring is "((()))"
 *
 *
 *
 *
 * Step
 * Current s
 * k-balanced
 * Result s
 *
 *
 *
 *
 * 1
 * ((()))()()()
 * ((()))()()()
 * ()()()
 *
 *
 * 2
 * ()()()
 * -
 * ()()()
 *
 *
 *
 *
 * Thus, the final string is "()()()".
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= s.length <= 10^5
 * s consists only of '(' and ')'.
 * 1 <= k <= s.length / 2
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_substring(s: String, k: i32) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some((last_c, last_count)) = stack.last_mut()
                && c == *last_c
            {
                *last_count += 1;
            } else {
                stack.push((c, 1));
            }
            while stack.len() >= 2 {
                let (c, count) = stack[stack.len() - 1];
                let (last_c, last_count) = stack[stack.len() - 2];
                if last_c == '(' && c == ')' && count >= k && last_count >= k {
                    stack.pop();
                    stack.pop();
                    if last_count > k {
                        stack.push((last_c, last_count - k));
                    }
                    if count > k {
                        stack.push((c, count - k));
                    }
                } else if last_c == c {
                    stack.pop();
                    stack.last_mut().unwrap().1 += count;
                } else {
                    break;
                }
            }
        }
        let mut answer = String::new();
        for (c, count) in stack {
            for _ in 0..count {
                answer.push(c);
            }
        }
        answer
    }
}
// @lc code=end

#[cfg(test)]
mod test_remove_k_balanced_substrings {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_substring("(())".to_string(), 1),
            "".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_substring("(()(".to_string(), 1),
            "((".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_substring("((()))()()()".to_string(), 3),
            "()()()".to_string()
        );
    }
}
