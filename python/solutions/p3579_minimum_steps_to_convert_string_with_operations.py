#
# @lc app=leetcode.cn id=3579 lang=python3
#
# [3579] Minimum Steps to Convert String with Operations
#
# https://leetcode.cn/problems/minimum-steps-to-convert-string-with-operations/description/
#
# algorithms
# Hard (66.19%)
# Likes:    4
# Dislikes: 0
# Total Accepted:    552
# Total Submissions: 834
# Testcase Example:  '"abcdf"\n"dacbe"'
#
# You are given two strings, word1 and word2, of equal length. You need to
# transform word1 into word2.
#
# For this, divide word1 into one or more contiguous substrings. For each
# substring substr you can perform the following operations:
#
#
#
# Replace: Replace the character at any one index of substr with another
# lowercase English letter.
#
#
# Swap: Swap any two characters in substr.
#
#
# Reverse Substring: Reverse substr.
#
#
#
# Each of these counts as one operation and each character of each substring
# can be used in each type of operation at most once (i.e. no single index may
# be involved in more than one replace, one swap, or one reverse).
#
# Return the minimum number of operations required to transform word1 into
# word2.
#
#
# Example 1:
#
#
# Input: word1 = "abcdf", word2 = "dacbe"
#
# Output: 4
#
# Explanation:
#
# Divide word1 into "ab", "c", and "df". The operations are:
#
#
# For the substring "ab",
#
#
# Perform operation of type 3 on "ab" -> "ba".
# Perform operation of type 1 on "ba" -> "da".
#
#
# For the substring "c" do no operations.
# For the substring "df",
#
# Perform operation of type 1 on "df" -> "bf".
# Perform operation of type 1 on "bf" -> "be".
#
#
#
#
#
# Example 2:
#
#
# Input: word1 = "abceded", word2 = "baecfef"
#
# Output: 4
#
# Explanation:
#
# Divide word1 into "ab", "ce", and "ded". The operations are:
#
#
# For the substring "ab",
#
#
# Perform operation of type 2 on "ab" -> "ba".
#
#
# For the substring "ce",
#
# Perform operation of type 2 on "ce" -> "ec".
#
#
# For the substring "ded",
#
# Perform operation of type 1 on "ded" -> "fed".
# Perform operation of type 1 on "fed" -> "fef".
#
#
#
#
#
# Example 3:
#
#
# Input: word1 = "abcdef", word2 = "fedabc"
#
# Output: 2
#
# Explanation:
#
# Divide word1 into "abcdef". The operations are:
#
#
# For the substring "abcdef",
#
#
# Perform operation of type 3 on "abcdef" -> "fedcba".
# Perform operation of type 2 on "fedcba" -> "fedabc".
#
#
#
#
#
#
# Constraints:
#
#
# 1 <= word1.length == word2.length <= 100
# word1 and word2 consist only of lowercase English letters.
#
#
#

# @lc code=start
class Solution:
  def minOperations(self, word1: str, word2: str) -> int:
    from collections import defaultdict
    from functools import cache

    @cache
    def min_operations(i: int, j: int) -> int:
      """Minimum operations to convert word1[i:j] to word2[i:j]"""
      return min(
        min_operations_without_reverse(word1[i:j], word2[i:j]),
        min_operations_with_reverse(word1[i:j], word2[i:j]),
      )

    def min_operations_without_reverse(s1: str, s2: str) -> int:
      mismatch = defaultdict(int)
      total_mismatch = 0
      for a, b in zip(s1, s2):
        if a != b:
          mismatch[(a, b)] += 1
          total_mismatch += 1

      max_switch = 0
      for a, b in list(mismatch.keys()):
        if ord(a) < ord(b):
          max_switch += min(mismatch[(a, b)], mismatch[(b, a)])

      return total_mismatch - max_switch

    def min_operations_with_reverse(s1: str, s2: str) -> int:
      return min_operations_without_reverse(s1[::-1], s2) + 1

    @cache
    def min_operations_with_split(i: int) -> int:
      """Minimum operations to convert word1[:i] to word2[:i]"""
      if i == 0:
        return 0
      return min(min_operations_with_split(j) + min_operations(j, i) for j in range(i))

    return min_operations_with_split(len(word1))


# @lc code=end


def test_example_1():
  assert Solution().minOperations("abcdf", "dacbe") == 4


def test_example_2():
  assert Solution().minOperations("abceded", "baecfef") == 4


def test_example_3():
  assert Solution().minOperations("abcdef", "fedabc") == 2
