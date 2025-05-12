#
# @lc app=leetcode.cn id=2094 lang=python3
#
# [2094] Finding 3-Digit Even Numbers
#
# https://leetcode.cn/problems/finding-3-digit-even-numbers/description/
#
# algorithms
# Easy (57.98%)
# Likes:    58
# Dislikes: 0
# Total Accepted:    23.8K
# Total Submissions: 37K
# Testcase Example:  '[2,1,3,0]'
#
# You are given an integer array digits, where each element is a digit. The
# array may contain duplicates.
#
# You need to find all the unique integers that follow the given
# requirements:
#
#
# The integer consists of the concatenation of three elements from digits in
# any arbitrary order.
# The integer does not have leading zeros.
# The integer is even.
#
#
# For example, if the given digits were [1, 2, 3], integers 132 and 312 follow
# the requirements.
#
# Return a sorted array of the unique integers.
#
#
# Example 1:
#
#
# Input: digits = [2,1,3,0]
# Output: [102,120,130,132,210,230,302,310,312,320]
# Explanation: All the possible integers that follow the requirements are in
# the output array.
# Notice that there are no odd integers or integers with leading zeros.
#
#
# Example 2:
#
#
# Input: digits = [2,2,8,8,2]
# Output: [222,228,282,288,822,828,882]
# Explanation: The same digit can be used as many times as it appears in
# digits.
# In this example, the digit 8 is used twice each time in 288, 828, and 882.
#
#
# Example 3:
#
#
# Input: digits = [3,7,5]
# Output: []
# Explanation: No even integers can be formed using the given digits.
#
#
#
# Constraints:
#
#
# 3 <= digits.length <= 100
# 0 <= digits[i] <= 9
#
#
#

# @lc code=start
class Solution:
  def findEvenNumbers(self, digits: list[int]) -> list[int]:
    return list(
      sorted(
        set(
          int(f"{a}{b}{c}")
          for i, a in enumerate(digits)
          for j, b in enumerate(digits)
          for k, c in enumerate(digits)
          if a != 0 and c % 2 == 0 and len(set([i, j, k])) == 3
        )
      )
    )


# @lc code=end


def test_example_1():
  digits = [2, 1, 3, 0]
  expected = [102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
  assert Solution().findEvenNumbers(digits) == expected


def test_example_2():
  digits = [2, 2, 8, 8, 2]
  expected = [222, 228, 282, 288, 822, 828, 882]
  assert Solution().findEvenNumbers(digits) == expected


def test_example_3():
  digits = [3, 7, 5]
  expected = []
  assert Solution().findEvenNumbers(digits) == expected
