/*
 * @lc app=leetcode.cn id=3144 lang=rust
 *
 * [3144] 分割字符频率相等的最少子字符串
 *
 * https://leetcode.cn/problems/minimum-substring-partition-of-equal-character-frequency/description/
 *
 * algorithms
 * Medium (66.27%)
 * Likes:    39
 * Dislikes: 0
 * Total Accepted:    14K
 * Total Submissions: 21.1K
 * Testcase Example:  '"fabccddg"'
 *
 * 给你一个字符串 s ，你需要将它分割成一个或者更多的 平衡 子字符串。比方说，s == "ababcc" 那么 ("abab", "c", "c")
 * ，("ab", "abc", "c") 和 ("ababcc") 都是合法分割，但是 ("a", "bab", "cc") ，("aba", "bc",
 * "c") 和 ("ab", "abcc") 不是，不平衡的子字符串用粗体表示。
 *
 * 请你返回 s 最少 能分割成多少个平衡子字符串。
 *
 * 注意：一个 平衡 字符串指的是字符串中所有字符出现的次数都相同。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "fabccddg"
 *
 * 输出：3
 *
 * 解释：
 *
 * 我们可以将 s 分割成 3 个子字符串：("fab, "ccdd", "g") 或者 ("fabc", "cd", "dg") 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "abababaccddb"
 *
 * 输出：2
 *
 * 解释：
 *
 * 我们可以将 s 分割成 2 个子字符串：("abab", "abaccddb") 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 1000
 * s 只包含小写英文字母。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let pre = s.chars().fold(vec![[0; 26]], |mut pre, c| {
            pre.push(pre.last().unwrap().clone());
            pre.last_mut().unwrap()[c as usize - 'a' as usize] += 1;
            pre
        });
        let mut dp: Vec<i32> = vec![0];
        for i in 1..pre.len() {
            let mut cur = i32::MAX;
            for j in 0..i {
                let cnt = (0..26)
                    .map(|k| (pre[i][k] - pre[j][k]))
                    .filter(|x| *x > 0)
                    .collect::<Vec<i32>>();
                if cnt.len() < 2 || cnt.iter().all(|&x| x == cnt[0]) {
                    cur = cur.min(dp[j] + 1);
                }
            }
            dp.push(cur);
        }
        *dp.last().unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("fabccddg".to_string()),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("abababaccddb".to_string()),
            2
        );
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_substrings_in_partition("fabccddg".to_string())
    );
    println!(
        "{}",
        Solution::minimum_substrings_in_partition("abababaccddb".to_string())
    );
}
