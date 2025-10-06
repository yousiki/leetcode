/*
 * @lc app=leetcode.com id=3582 lang=rust
 *
 * [3582] Generate Tag for Video Caption
 *
 * https://leetcode.com/problems/generate-tag-for-video-caption/description/
 *
 * algorithms
 * Easy (33.69%)
 * Likes:    2
 * Dislikes: 0
 * Total Accepted:    3.1K
 * Total Submissions: 9.2K
 * Testcase Example:  '"Leetcode daily streak achieved"'
 *
 * You are given a string caption representing the caption for a video.
 *
 * The following actions must be performed in order to generate a valid tag for
 * the video:
 *
 *
 *
 * Combine all words in the string into a single camelCase string prefixed with
 * '#'. A camelCase string is one where the first letter of all words except
 * the first one is capitalized. All characters after the first character in
 * each word must be lowercase.
 *
 *
 * Remove all characters that are not an English letter, except the first
 * '#'.
 *
 *
 * Truncate the result to a maximum of 100 characters.
 *
 *
 *
 * Return the tag after performing the actions on caption.
 *
 *
 * Example 1:
 *
 *
 * Input: caption = "Leetcode daily streak achieved"
 *
 * Output: "#leetcodeDailyStreakAchieved"
 *
 * Explanation:
 *
 * The first letter for all words except "leetcode" should be capitalized.
 *
 *
 * Example 2:
 *
 *
 * Input: caption = "can I Go There"
 *
 * Output: "#canIGoThere"
 *
 * Explanation:
 *
 * The first letter for all words except "can" should be capitalized.
 *
 *
 * Example 3:
 *
 *
 * Input: caption =
 * "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
 *
 * Output:
 * "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
 *
 * Explanation:
 *
 * Since the first word has length 101, we need to truncate the last two
 * letters from the word.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= caption.length <= 150
 * caption consists only of English letters and ' '.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn generate_tag(caption: String) -> String {
        (String::from("#")
            + &caption
                .split_whitespace()
                .enumerate()
                .map(|(idx, word)| {
                    if idx == 0 {
                        word.to_lowercase().to_string()
                    } else {
                        word.to_lowercase()
                            .chars()
                            .enumerate()
                            .map(|(i, c)| {
                                if i == 0 {
                                    c.to_uppercase().to_string()
                                } else {
                                    c.to_lowercase().to_string()
                                }
                            })
                            .collect::<String>()
                    }
                })
                .collect::<String>())
            .chars()
            .take(100)
            .collect::<String>()
    }
}
// @lc code=end

#[cfg(test)]
mod test_generate_tag_for_video_caption {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_tag("Leetcode daily streak achieved".to_string()),
            "#leetcodeDailyStreakAchieved".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::generate_tag("can I Go There".to_string()),
            "#canIGoThere".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::generate_tag("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string()),
            "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string()
        );
    }
}
