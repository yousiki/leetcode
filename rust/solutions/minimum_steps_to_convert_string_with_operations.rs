/*
 * @lc app=leetcode.cn id=3579 lang=rust
 *
 * [3579] Minimum Steps to Convert String with Operations
 *
 * https://leetcode.cn/problems/minimum-steps-to-convert-string-with-operations/description/
 *
 * algorithms
 * Hard (66.14%)
 * Likes:    6
 * Dislikes: 0
 * Total Accepted:    914
 * Total Submissions: 1.4K
 * Testcase Example:  '"abcdf"\n"dacbe"'
 *
 * You are given two strings, word1 and word2, of equal length. You need to
 * transform word1 into word2.
 *
 * For this, divide word1 into one or more contiguous substrings. For each
 * substring substr you can perform the following operations:
 *
 *
 *
 * Replace: Replace the character at any one index of substr with another
 * lowercase English letter.
 *
 *
 * Swap: Swap any two characters in substr.
 *
 *
 * Reverse Substring: Reverse substr.
 *
 *
 *
 * Each of these counts as one operation and each character of each substring
 * can be used in each type of operation at most once (i.e. no single index may
 * be involved in more than one replace, one swap, or one reverse).
 *
 * Return the minimum number of operations required to transform word1 into
 * word2.
 *
 *
 * Example 1:
 *
 *
 * Input: word1 = "abcdf", word2 = "dacbe"
 *
 * Output: 4
 *
 * Explanation:
 *
 * Divide word1 into "ab", "c", and "df". The operations are:
 *
 *
 * For the substring "ab",
 *
 *
 * Perform operation of type 3 on "ab" -> "ba".
 * Perform operation of type 1 on "ba" -> "da".
 *
 *
 * For the substring "c" do no operations.
 * For the substring "df",
 *
 * Perform operation of type 1 on "df" -> "bf".
 * Perform operation of type 1 on "bf" -> "be".
 *
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "abceded", word2 = "baecfef"
 *
 * Output: 4
 *
 * Explanation:
 *
 * Divide word1 into "ab", "ce", and "ded". The operations are:
 *
 *
 * For the substring "ab",
 *
 *
 * Perform operation of type 2 on "ab" -> "ba".
 *
 *
 * For the substring "ce",
 *
 * Perform operation of type 2 on "ce" -> "ec".
 *
 *
 * For the substring "ded",
 *
 * Perform operation of type 1 on "ded" -> "fed".
 * Perform operation of type 1 on "fed" -> "fef".
 *
 *
 *
 *
 *
 * Example 3:
 *
 *
 * Input: word1 = "abcdef", word2 = "fedabc"
 *
 * Output: 2
 *
 * Explanation:
 *
 * Divide word1 into "abcdef". The operations are:
 *
 *
 * For the substring "abcdef",
 *
 *
 * Perform operation of type 3 on "abcdef" -> "fedcba".
 * Perform operation of type 2 on "fedcba" -> "fedabc".
 *
 *
 *
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= word1.length == word2.length <= 100
 * word1 and word2 consist only of lowercase English letters.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let mut dp = vec![i32::MAX / 2; n + 1];
        dp[0] = 0; // Initialize the base case
        for i in 1..=n {
            for j in 1..=i {
                dp[i] = dp[i].min(
                    dp[j - 1]
                        + Self::min_operations_substr(
                            &word1.get(j - 1..i).unwrap(),
                            &word2.get(j - 1..i).unwrap(),
                        ),
                );
            }
        }
        dp[n]
    }

    fn min_operations_substr(substr1: &str, substr2: &str) -> i32 {
        std::cmp::min(
            Self::min_operations_greedy(substr1, substr2),
            Self::min_operations_greedy(
                substr1,
                substr2.chars().rev().collect::<String>().as_str(),
            ) + 1,
        )
    }

    fn min_operations_greedy(substr1: &str, substr2: &str) -> i32 {
        let mut mismatch = std::collections::HashMap::new();
        for (c1, c2) in substr1.chars().zip(substr2.chars()) {
            if c1 != c2 {
                *mismatch.entry((c1, c2)).or_insert(0) += 1;
            }
        }
        let mut switch_operations = 0;
        let mut replace_operations = 0;
        for ((c1, c2), &m) in mismatch.iter() {
            if let Some(&n) = mismatch.get(&(*c2, *c1)) {
                let min = std::cmp::min(m, n);
                switch_operations += min;
                replace_operations += m - min;
            } else {
                replace_operations += m;
            }
        }
        assert_eq!(switch_operations % 2, 0);
        switch_operations / 2 + replace_operations
    }
}
// @lc code=end

mod test_minimum_steps_to_convert_string_with_operations {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_operations(String::from("abcdf"), String::from("dacbe")),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_operations(String::from("abceded"), String::from("baecfef"),),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_operations(String::from("abcdef"), String::from("fedabc")),
            2
        );
    }
}
