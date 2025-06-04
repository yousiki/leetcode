#
# @lc app=leetcode.cn id=3403 lang=python3
#
# [3403] Find the Lexicographically Largest String From the Box I
#
# https://leetcode.cn/problems/find-the-lexicographically-largest-string-from-the-box-i/description/
#
# algorithms
# Medium (40.48%)
# Likes:    25
# Dislikes: 0
# Total Accepted:    11.1K
# Total Submissions: 27.5K
# Testcase Example:  '"dbca"\n2'
#
# You are given a string word, and an integer numFriends.
#
# Alice is organizing a game for her numFriends friends. There are multiple
# rounds in the game, where in each round:
#
#
# word is split into numFriends non-empty strings, such that no previous round
# has had the exact same split.
# All the split words are put into a box.
#
#
# Find the lexicographically largest string from the box after all the rounds
# are finished.
#
#
# Example 1:
#
#
# Input: word = "dbca", numFriends = 2
#
# Output: "dbc"
#
# Explanation:
#
# All possible splits are:
#
#
# "d" and "bca".
# "db" and "ca".
# "dbc" and "a".
#
#
#
# Example 2:
#
#
# Input: word = "gggg", numFriends = 4
#
# Output: "g"
#
# Explanation:
#
# The only possible split is: "g", "g", "g", and "g".
#
#
#
# Constraints:
#
#
# 1 <= word.length <= 5 * 10^3
# word consists only of lowercase English letters.
# 1 <= numFriends <= word.length
#
#
#

# @lc code=start
class Solution:
  def answerString(self, word: str, numFriends: int) -> str:
    # Special case: only one friend
    if numFriends == 1:
      return word
    # Calculate the maximum length of the sub-string that can be splitted
    max_length = len(word) - numFriends + 1
    # Find the lexicographically largest sub-string of the maximum length
    largest_substring = max(word[i : i + max_length] for i in range(len(word)))
    # Return the largest sub-string
    return largest_substring


# @lc code=end


def test_example_1():
  assert Solution().answerString("dbca", 2) == "dbc"


def test_example_2():
  assert Solution().answerString("gggg", 4) == "g"


def test_example_3():
  assert Solution().answerString("aann", 2) == "nn"


def test_example_4():
  assert Solution().answerString("gh", 1) == "gh"
