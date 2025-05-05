#
# @lc app=leetcode.cn id=3536 lang=python3
#
# [3536] Maximum Product of Two Digits
#
# https://leetcode.cn/problems/maximum-product-of-two-digits/description/
#
# algorithms
# Easy (82.16%)
# Likes:    3
# Dislikes: 0
# Total Accepted:    1.9K
# Total Submissions: 2.3K
# Testcase Example:  '31'
#
# You are given a positive integer n.
#
# Return the maximum product of any two digits in n.
#
# Note: You may use the same digit twice if it appears more than once in n.
#
#
# Example 1:
#
#
# Input: n = 31
#
# Output: 3
#
# Explanation:
#
#
# The digits of n are [3, 1].
# The possible products of any two digits are: 3 * 1 = 3.
# The maximum product is 3.
#
#
#
# Example 2:
#
#
# Input: n = 22
#
# Output: 4
#
# Explanation:
#
#
# The digits of n are [2, 2].
# The possible products of any two digits are: 2 * 2 = 4.
# The maximum product is 4.
#
#
#
# Example 3:
#
#
# Input: n = 124
#
# Output: 8
#
# Explanation:
#
#
# The digits of n are [1, 2, 4].
# The possible products of any two digits are: 1 * 2 = 2, 1 * 4 = 4, 2 * 4 =
# 8.
# The maximum product is 8.
#
#
#
#
# Constraints:
#
#
# 10 <= n <= 10^9
#
#
#

# @lc code=start
class Solution:
  def maxProduct(self, n: int) -> int:
    from heapq import nlargest

    digits = [int(d) for d in str(n)]
    largest_2 = nlargest(2, digits)
    return largest_2[0] * largest_2[1]


# @lc code=end


def test_example_1():
  assert Solution().maxProduct(31) == 3


def test_example_2():
  assert Solution().maxProduct(22) == 4


def test_example_3():
  assert Solution().maxProduct(124) == 8
