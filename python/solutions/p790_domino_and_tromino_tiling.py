#
# @lc app=leetcode.cn id=790 lang=python3
#
# [790] Domino and Tromino Tiling
#
# https://leetcode.cn/problems/domino-and-tromino-tiling/description/
#
# algorithms
# Medium (51.00%)
# Likes:    419
# Dislikes: 0
# Total Accepted:    46.1K
# Total Submissions: 88.7K
# Testcase Example:  '3'
#
# You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You
# may rotate these shapes.
#
# Given an integer n, return the number of ways to tile an 2 x n board. Since
# the answer may be very large, return it modulo 10^9 + 7.
#
# In a tiling, every square must be covered by a tile. Two tilings are
# different if and only if there are two 4-directionally adjacent cells on the
# board such that exactly one of the tilings has both squares occupied by a
# tile.
#
#
# Example 1:
#
#
# Input: n = 3
# Output: 5
# Explanation: The five different ways are show above.
#
#
# Example 2:
#
#
# Input: n = 1
# Output: 1
#
#
#
# Constraints:
#
#
# 1 <= n <= 1000
#
#
#

# @lc code=start
class Solution:
  def numTilings(self, n: int) -> int:
    from functools import cache

    MOD = int(10**9 + 7)

    # All the possible states of the tailing column
    # 0: full
    # 1: half is filled

    @cache
    def dp(remain: int, state: int) -> int:
      """ "Return the number of ways to fill the remaining columns.

      Args:
        remain: the number of remaining columns to fill
        state: the state of the last column, 0 for full, 1 for half filled

      Returns:
        The number of ways to fill the remaining columns.
      """
      if remain < 0:  # Invalid state
        return 0
      if remain == 0:
        return 1 if state == 0 else 0

      match state:
        case 0:  # The last column is full
          return (dp(remain - 1, 0) + dp(remain - 2, 0) + dp(remain - 2, 1) * 2) % MOD
        case 1:  # The last column is half filled
          return (dp(remain - 1, 1) + dp(remain - 1, 0)) % MOD
        case _:
          raise ValueError("Invalid state")

    return dp(n, 0) % MOD


# @lc code=end


def test_example_1():
  assert Solution().numTilings(3) == 5


def test_example_2():
  assert Solution().numTilings(1) == 1
