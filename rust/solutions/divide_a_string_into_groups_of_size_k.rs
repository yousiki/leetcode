/*
 * @lc app=leetcode.com id=2138 lang=rust
 *
 * [2138] Divide a String Into Groups of Size k
 *
 * https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/
 *
 * algorithms
 * Easy (72.65%)
 * Likes:    33
 * Dislikes: 0
 * Total Accepted:    24.3K
 * Total Submissions: 33.4K
 * Testcase Example:  '"abcdefghi"\n3\n"x"'
 *
 * A string s can be partitioned into groups of size k using the following
 * procedure:
 *
 *
 * The first group consists of the first k characters of the string, the second
 * group consists of the next k characters of the string, and so on. Each
 * element can be a part of exactly one group.
 * For the last group, if the string does not have k characters remaining, a
 * character fill is used to complete the group.
 *
 *
 * Note that the partition is done so that after removing the fill character
 * from the last group (if it exists) and concatenating all the groups in
 * order, the resultant string should be s.
 *
 * Given the string s, the size of each group k and the character fill, return
 * a string array denoting the composition of every group s has been divided
 * into, using the above procedure.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abcdefghi", k = 3, fill = "x"
 * Output: ["abc","def","ghi"]
 * Explanation:
 * The first 3 characters "abc" form the first group.
 * The next 3 characters "def" form the second group.
 * The last 3 characters "ghi" form the third group.
 * Since all groups can be completely filled by characters from the string, we
 * do not need to use fill.
 * Thus, the groups formed are "abc", "def", and "ghi".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "abcdefghij", k = 3, fill = "x"
 * Output: ["abc","def","ghi","jxx"]
 * Explanation:
 * Similar to the previous example, we are forming the first three groups
 * "abc", "def", and "ghi".
 * For the last group, we can only use the character 'j' from the string. To
 * complete this group, we add 'x' twice.
 * Thus, the 4 groups formed are "abc", "def", "ghi", and "jxx".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 100
 * s consists of lowercase English letters only.
 * 1 <= k <= 100
 * fill is a lowercase English letter.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        (0..s.len())
            .step_by(k as usize)
            .map(|i| {
                if i + k as usize <= s.len() {
                    s[i..i + k as usize].to_string()
                } else {
                    s[i..].to_string()
                        + std::iter::repeat(fill)
                            .take(k as usize - (s.len() - i))
                            .collect::<String>()
                            .as_str()
                }
            })
            .collect::<Vec<String>>()
    }
}
// @lc code=end

#[cfg(test)]
mod test_divide_a_string_into_groups_of_size_k {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string()
            ]
        );
    }
}
