#
# @lc app=leetcode.cn id=3337 lang=python3
#
# [3337] Total Characters in String After Transformations II
#
# https://leetcode.cn/problems/total-characters-in-string-after-transformations-ii/description/
#
# algorithms
# Hard (45.36%)
# Likes:    15
# Dislikes: 0
# Total Accepted:    5.7K
# Total Submissions: 10.1K
# Testcase Example:  '"abcyy"\n2\n[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]'
#
# You are given a string s consisting of lowercase English letters, an integer
# t representing the number of transformations to perform, and an array nums of
# size 26. In one transformation, every character in s is replaced according to
# the following rules:
#
#
# Replace s[i] with the next nums[s[i] - 'a'] consecutive characters in the
# alphabet. For example, if s[i] = 'a' and nums[0] = 3, the character 'a'
# transforms into the next 3 consecutive characters ahead of it, which results
# in "bcd".
# The transformation wraps around the alphabet if it exceeds 'z'. For example,
# if s[i] = 'y' and nums[24] = 3, the character 'y' transforms into the next 3
# consecutive characters ahead of it, which results in "zab".
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
# Input: s = "abcyy", t = 2, nums =
# [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]
#
# Output: 7
#
# Explanation:
#
#
#
# First Transformation (t = 1):
#
#
# 'a' becomes 'b' as nums[0] == 1
# 'b' becomes 'c' as nums[1] == 1
# 'c' becomes 'd' as nums[2] == 1
# 'y' becomes 'z' as nums[24] == 1
# 'y' becomes 'z' as nums[24] == 1
# String after the first transformation: "bcdzz"
#
#
#
# Second Transformation (t = 2):
#
#
# 'b' becomes 'c' as nums[1] == 1
# 'c' becomes 'd' as nums[2] == 1
# 'd' becomes 'e' as nums[3] == 1
# 'z' becomes 'ab' as nums[25] == 2
# 'z' becomes 'ab' as nums[25] == 2
# String after the second transformation: "cdeabab"
#
#
#
# Final Length of the string: The string is "cdeabab", which has 7
# characters.
#
#
#
#
# Example 2:
#
#
# Input: s = "azbk", t = 1, nums =
# [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]
#
# Output: 8
#
# Explanation:
#
#
#
# First Transformation (t = 1):
#
#
# 'a' becomes 'bc' as nums[0] == 2
# 'z' becomes 'ab' as nums[25] == 2
# 'b' becomes 'cd' as nums[1] == 2
# 'k' becomes 'lm' as nums[10] == 2
# String after the first transformation: "bcabcdlm"
#
#
#
# Final Length of the string: The string is "bcabcdlm", which has 8
# characters.
#
#
#
#
#
# Constraints:
#
#
# 1 <= s.length <= 10^5
# s consists only of lowercase English letters.
# 1 <= t <= 10^9
# nums.length == 26
# 1 <= nums[i] <= 25
#
#
#

# @lc code=start
from math import exp


class Solution:
  mod: int = int(1e9 + 7)

  def lengthAfterTransformations(self, s: str, t: int, nums: list[int]) -> int:
    from collections import Counter

    # Create the transformation matrix
    m = 26
    trans = [[0] * m for _ in range(m)]
    for i in range(m):
      for j in range(1, nums[i] + 1):
        trans[i][(i + j) % m] += 1

    # Calculate the transformation matrix raised to the power of t
    trans_t = Solution.matrix_power(trans, t)

    # Calculate the length of the resulting string
    result = 0
    s = [ord(c) - ord("a") for c in s]
    for k, v in Counter(s).items():
      result = (result + v * sum(trans_t[k])) % Solution.mod

    return result

  @staticmethod
  def matrix_mult(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    n = len(a)
    m = len(b[0])
    p = len(b)
    c = [[0] * m for _ in range(n)]
    for i in range(n):
      for j in range(m):
        for k in range(p):
          c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % Solution.mod
    return c

  @staticmethod
  def matrix_power(a: list[list[int]], n: int) -> list[list[int]]:
    m = len(a)
    res = [[1 if i == j else 0 for j in range(m)] for i in range(m)]
    while n:
      if n & 1:
        res = Solution.matrix_mult(res, a)
      a = Solution.matrix_mult(a, a)
      n >>= 1
    return res


# @lc code=end


def test_example_1():
  s = "abcyy"
  t = 2
  nums = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
  solution = Solution()
  output = solution.lengthAfterTransformations(s, t, nums)
  expected = 7
  assert output == expected


def test_example_2():
  s = "azbk"
  t = 1
  nums = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
  solution = Solution()
  output = solution.lengthAfterTransformations(s, t, nums)
  expected = 8
  assert output == expected
