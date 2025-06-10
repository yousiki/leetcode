#
# @lc app=leetcode.cn id=3442 lang=python3
#
# [3442] Maximum Difference Between Even and Odd Frequency I
#
# https://leetcode.cn/problems/maximum-difference-between-even-and-odd-frequency-i/description/
#
# algorithms
# Easy (61.14%)
# Likes:    10
# Dislikes: 0
# Total Accepted:    13.2K
# Total Submissions: 20.3K
# Testcase Example:  '"aaaaabbc"'
#
# You are given a string s consisting of lowercase English letters.
#
# Your task is to find the maximum difference diff = freq(a1) - freq(a2)
# between the frequency of characters a1 and a2 in the string such that:
#
#
# a1 has an odd frequency in the string.
# a2 has an even frequency in the string.
#
#
# Return this maximum difference.
#
#
# Example 1:
#
#
# Input: s = "aaaaabbc"
#
# Output: 3
#
# Explanation:
#
#
# The character 'a' has an odd frequency of 5, and 'b' has an even frequency of
# 2.
# The maximum difference is 5 - 2 = 3.
#
#
#
# Example 2:
#
#
# Input: s = "abcabcab"
#
# Output: 1
#
# Explanation:
#
#
# The character 'a' has an odd frequency of 3, and 'c' has an even frequency of
# 2.
# The maximum difference is 3 - 2 = 1.
#
#
#
#
# Constraints:
#
#
# 3 <= s.length <= 100
# s consists only of lowercase English letters.
# s contains at least one character with an odd frequency and one with an even
# frequency.
#
#
#

# @lc code=start
class Solution:
  def maxDifference(self, s: str) -> int:
    from collections import Counter

    counter = Counter(s)
    counter_values = list(counter.values())

    max_odd = max(filter(lambda x: x % 2 == 1, counter_values))
    min_even = min(filter(lambda x: x % 2 == 0, counter_values))

    return max_odd - min_even


# @lc code=end


def test_example_1():
  assert Solution().maxDifference("aaaaabbc") == 3


def test_example_2():
  assert Solution().maxDifference("abcabcab") == 1
