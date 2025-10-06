/*
 * @lc app=leetcode.com id=1432 lang=rust
 *
 * [1432] Max Difference You Can Get From Changing an Integer
 *
 * https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/description/
 *
 * algorithms
 * Medium (44.48%)
 * Likes:    41
 * Dislikes: 0
 * Total Accepted:    18.3K
 * Total Submissions: 41.1K
 * Testcase Example:  '555'
 *
 * You are given an integer num. You will apply the following steps to num two
 * separate times:
 *
 *
 * Pick a digit x (0 <= x <= 9).
 * Pick another digit y (0 <= y <= 9). Note y can be equal to x.
 * Replace all the occurrences of x in the decimal representation of num by
 * y.
 *
 *
 * Let a and b be the two results from applying the operation to num
 * independently.
 *
 * Return the max difference between a and b.
 *
 * Note that neither a nor b may have any leading zeros, and must not be 0.
 *
 *
 * Example 1:
 *
 *
 * Input: num = 555
 * Output: 888
 * Explanation: The first time pick x = 5 and y = 9 and store the new integer
 * in a.
 * The second time pick x = 5 and y = 1 and store the new integer in b.
 * We have now a = 999 and b = 111 and max difference = 888
 *
 *
 * Example 2:
 *
 *
 * Input: num = 9
 * Output: 8
 * Explanation: The first time pick x = 9 and y = 9 and store the new integer
 * in a.
 * The second time pick x = 9 and y = 1 and store the new integer in b.
 * We have now a = 9 and b = 1 and max difference = 8
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= num <= 10^8
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let all_changed_nums = Self::all_changed_nums(num);
        let min_changed = all_changed_nums.iter().min().unwrap_or(&num);
        let max_changed = all_changed_nums.iter().max().unwrap_or(&num);
        max_changed - min_changed
    }

    fn all_changed_nums(num: i32) -> Vec<i32> {
        let mut all_changed_nums = Vec::new();

        for i in 0..=9 {
            for j in 0..=9 {
                let changed = num
                    .to_string()
                    .replace(i.to_string().as_str(), j.to_string().as_str());
                if changed.chars().take(1).next() == Some('0') {
                    continue; // Skip if the first character is '0'
                }
                let changed = changed.parse::<i32>().unwrap_or(0);
                all_changed_nums.push(changed);
            }
        }

        all_changed_nums
    }
}
// @lc code=end

#[cfg(test)]
mod test_max_difference_you_can_get_from_changing_an_integer {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_diff(555), 888);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_diff(9), 8);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_diff(123456), 820000);
    }
}
