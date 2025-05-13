#
# @lc app=leetcode.cn id=3335 lang=python3
#
# [3335] Total Characters in String After Transformations I
#
# https://leetcode.cn/problems/total-characters-in-string-after-transformations-i/description/
#
# algorithms
# Medium (45.59%)
# Likes:    34
# Dislikes: 0
# Total Accepted:    11.8K
# Total Submissions: 26K
# Testcase Example:  '"abcyy"\n2'
#
# You are given a string s and an integer t, representing the number of
# transformations to perform. In one transformation, every character in s is
# replaced according to the following rules:
#
#
# If the character is 'z', replace it with the string "ab".
# Otherwise, replace it with the next character in the alphabet. For example,
# 'a' is replaced with 'b', 'b' is replaced with 'c', and so on.
#
#
# Return the length of the resulting string after exactly t transformations.
#
# Since the answer may be very large, return it modulo 10^9 + 7.
#
#
# Example 1:
#
#
# Input: s = "abcyy", t = 2
#
# Output: 7
#
# Explanation:
#
#
# First Transformation (t = 1):
#
#
# 'a' becomes 'b'
# 'b' becomes 'c'
# 'c' becomes 'd'
# 'y' becomes 'z'
# 'y' becomes 'z'
# String after the first transformation: "bcdzz"
#
#
# Second Transformation (t = 2):
#
# 'b' becomes 'c'
# 'c' becomes 'd'
# 'd' becomes 'e'
# 'z' becomes "ab"
# 'z' becomes "ab"
# String after the second transformation: "cdeabab"
#
#
# Final Length of the string: The string is "cdeabab", which has 7
# characters.
#
#
#
# Example 2:
#
#
# Input: s = "azbk", t = 1
#
# Output: 5
#
# Explanation:
#
#
# First Transformation (t = 1):
#
#
# 'a' becomes 'b'
# 'z' becomes "ab"
# 'b' becomes 'c'
# 'k' becomes 'l'
# String after the first transformation: "babcl"
#
#
# Final Length of the string: The string is "babcl", which has 5
# characters.
#
#
#
#
# Constraints:
#
#
# 1 <= s.length <= 10^5
# s consists only of lowercase English letters.
# 1 <= t <= 10^5
#
#
#

# @lc code=start
class Solution:
  def lengthAfterTransformations(self, s: str, t: int) -> int:
    from collections import Counter

    mod = int(10**9 + 7)
    chars = dict(Counter(s))

    for _ in range(t):
      new_chars = {}
      for k, v in chars.items():
        if k == "z":
          new_chars["a"] = (new_chars.get("a", 0) + v) % mod
          new_chars["b"] = (new_chars.get("b", 0) + v) % mod
        else:
          new_chars[chr(ord(k) + 1)] = (new_chars.get(chr(ord(k) + 1), 0) + v) % mod
      chars = new_chars

    return sum(chars.values()) % mod


# @lc code=end


def test_example_1():
  s = "abcyy"
  t = 2
  expected = 7
  assert Solution().lengthAfterTransformations(s, t) == expected


def test_example_2():
  s = "azbk"
  t = 1
  expected = 5
  assert Solution().lengthAfterTransformations(s, t) == expected
