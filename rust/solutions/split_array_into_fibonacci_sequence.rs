/*
 * @lc app=leetcode.com id=842 lang=rust
 *
 * [842] Split Array into Fibonacci Sequence
 *
 * https://leetcode.com/problems/split-array-into-fibonacci-sequence/description/
 *
 * algorithms
 * Medium (48.32%)
 * Likes:    307
 * Dislikes: 0
 * Total Accepted:    37.5K
 * Total Submissions: 77.6K
 * Testcase Example:  '"1101111"'
 *
 * You are given a string of digits num, such as "123456579". We can split it
 * into a Fibonacci-like sequence [123, 456, 579].
 *
 * Formally, a Fibonacci-like sequence is a list f of non-negative integers
 * such that:
 *
 *
 * 0 <= f[i] < 2^31, (that is, each integer fits in a 32-bit signed integer
 * type),
 * f.length >= 3, and
 * f[i] + f[i + 1] == f[i + 2] for all 0 <= i < f.length - 2.
 *
 *
 * Note that when splitting the string into pieces, each piece must not have
 * extra leading zeroes, except if the piece is the number 0 itself.
 *
 * Return any Fibonacci-like sequence split from num, or return [] if it cannot
 * be done.
 *
 *
 * Example 1:
 *
 *
 * Input: num = "1101111"
 * Output: [11,0,11,11]
 * Explanation: The output [110, 1, 111] would also be accepted.
 *
 *
 * Example 2:
 *
 *
 * Input: num = "112358130"
 * Output: []
 * Explanation: The task is impossible.
 *
 *
 * Example 3:
 *
 *
 * Input: num = "0123"
 * Output: []
 * Explanation: Leading zeroes are not allowed, so "01", "2", "3" is not
 * valid.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= num.length <= 200
 * num contains only digits.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        // Check if the string is too short to form 3 numbers
        if num.len() < 3 {
            return vec![];
        }
        // Iterate over all possible lengths for the first two numbers
        for num1_len in 1..=std::cmp::min(10, num.len() - 2) {
            for num2_len in 1..=std::cmp::min(10, num.len() - num1_len - 1) {
                if let Some(seq) = Self::try_split(&num, num1_len, num2_len) {
                    return seq;
                }
            }
        }
        vec![]
    }

    fn try_split(num: &str, num1_len: usize, num2_len: usize) -> Option<Vec<i32>> {
        // Extract the first two numbers based on the given lengths
        let num1 = Self::parse_number(&num[0..num1_len])?;
        let num2 = Self::parse_number(&num[num1_len..num1_len + num2_len])?;
        // Initialize the Fibonacci sequence with the first two numbers
        let mut fib_seq = vec![num1, num2];
        let mut index = num1_len + num2_len;
        // Continue to build the sequence until we reach the end of the string
        while index < num.len() {
            // Calculate the next number in the Fibonacci sequence
            let next_num = fib_seq[fib_seq.len() - 1] as i64 + fib_seq[fib_seq.len() - 2] as i64;
            // Count the length of the next number
            let next_num_str = next_num.to_string();
            let next_num_len = next_num_str.len();
            // Check if the length exceeds the remaining string length
            if index + next_num_len > num.len() {
                return None;
            }
            // Try to parse the next number from the string, according to the calculated length
            let next_num_parsed = Self::parse_number(&num[index..index + next_num_len])?;
            // If the parsed number does not match the expected next number, return None
            if next_num_parsed as i64 != next_num {
                return None;
            }
            // Add the next number to the Fibonacci sequence
            fib_seq.push(next_num_parsed);
            // Move the index forward by the length of the next number
            index += next_num_len;
        }
        // If we have successfully used the entire string, return the Fibonacci sequence
        if index == num.len() && fib_seq.len() >= 3 {
            Some(fib_seq)
        } else {
            None
        }
    }

    fn parse_number(s: &str) -> Option<i32> {
        if s.len() == 0 {
            return None;
        }
        // Check if the number fits in a 32-bit signed integer
        if let Ok(num) = s.parse::<i64>() {
            if num < (1i64 << 31) {
                // Check for leading zeros
                if num != 0 && s.starts_with('0') {
                    return None;
                }
                return Some(num as i32);
            }
        }
        None
    }
}
// @lc code=end

#[cfg(test)]
mod test_split_array_into_fibonacci_sequence {
    use super::Solution;

    #[test]
    fn test1() {
        let num = "1101111".to_string();
        let result = Solution::split_into_fibonacci(num);
        assert_eq!(result, vec![11, 0, 11, 11]);
    }

    #[test]
    fn test2() {
        let num = "112358130".to_string();
        let result = Solution::split_into_fibonacci(num);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test3() {
        let num = "0123".to_string();
        let result = Solution::split_into_fibonacci(num);
        assert_eq!(result, vec![]);
    }
}
