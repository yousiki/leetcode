#
# @lc app=leetcode.cn id=3527 lang=python3
#
# [3527] Find the Most Common Response
#
# https://leetcode.cn/problems/find-the-most-common-response/description/
#
# algorithms
# Medium (68.27%)
# Likes:    1
# Dislikes: 0
# Total Accepted:    1.5K
# Total Submissions: 2.1K
# Testcase Example:  '[["good","ok","good","ok"],["ok","bad","good","ok","ok"],["good"],["bad"]]'
#
# You are given a 2D string array responses where each responses[i] is an array
# of strings representing survey responses from the i^th day.
#
# Return the most common response across all days after removing duplicate
# responses within each responses[i]. If there is a tie, return the
# lexicographically smallest response.
#
#
# Example 1:
#
#
# Input: responses =
# [["good","ok","good","ok"],["ok","bad","good","ok","ok"],["good"],["bad"]]
#
# Output: "good"
#
# Explanation:
#
#
# After removing duplicates within each list, responses = [["good", "ok"],
# ["ok", "bad", "good"], ["good"], ["bad"]].
# "good" appears 3 times, "ok" appears 2 times, and "bad" appears 2 times.
# Return "good" because it has the highest frequency.
#
#
#
# Example 2:
#
#
# Input: responses =
# [["good","ok","good"],["ok","bad"],["bad","notsure"],["great","good"]]
#
# Output: "bad"
#
# Explanation:
#
#
# After removing duplicates within each list we have responses = [["good",
# "ok"], ["ok", "bad"], ["bad", "notsure"], ["great", "good"]].
# "bad", "good", and "ok" each occur 2 times.
# The output is "bad" because it is the lexicographically smallest amongst the
# words with the highest frequency.
#
#
#
#
# Constraints:
#
#
# 1 <= responses.length <= 1000
# 1 <= responses[i].length <= 1000
# 1 <= responses[i][j].length <= 10
# responses[i][j] consists of only lowercase English letters
#
#
#

# @lc code=start
class Solution:
  def findCommonResponse(self, responses: list[list[str]]) -> str:
    from collections import Counter
    from itertools import chain

    return min(
      Counter(chain(*[set(response) for response in responses])).items(),
      key=lambda x: (-x[1], x[0]),
    )[0]


# @lc code=end


def test_example_1():
  assert (
    Solution().findCommonResponse(
      [
        ["good", "ok", "good", "ok"],
        ["ok", "bad", "good", "ok", "ok"],
        ["good"],
        ["bad"],
      ]
    )
    == "good"
  )


def test_example_2():
  assert (
    Solution().findCommonResponse(
      [["good", "ok", "good"], ["ok", "bad"], ["bad", "notsure"], ["great", "good"]]
    )
    == "bad"
  )
