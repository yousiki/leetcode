#
# @lc app=leetcode.cn id=3446 lang=python3
#
# [3446] Sort Matrix by Diagonals
#
# https://leetcode.cn/problems/sort-matrix-by-diagonals/description/
#
# algorithms
# Medium (79.38%)
# Likes:    10
# Dislikes: 0
# Total Accepted:    3.8K
# Total Submissions: 4.7K
# Testcase Example:  '[[1,7,3],[9,8,2],[4,5,6]]'
#
# You are given an n x n square matrix of integers grid. Return the matrix such
# that:
#
#
# The diagonals in the bottom-left triangle (including the middle diagonal) are
# sorted in non-increasing order.
# The diagonals in the top-right triangle are sorted in non-decreasing
# order.
#
#
#
# Example 1:
#
#
# Input: grid = [[1,7,3],[9,8,2],[4,5,6]]
#
# Output: [[8,2,3],[9,6,7],[4,5,1]]
#
# Explanation:
#
#
#
# The diagonals with a black arrow (bottom-left triangle) should be sorted in
# non-increasing order:
#
#
# [1, 8, 6] becomes [8, 6, 1].
# [9, 5] and [4] remain unchanged.
#
#
# The diagonals with a blue arrow (top-right triangle) should be sorted in
# non-decreasing order:
#
#
# [7, 2] becomes [2, 7].
# [3] remains unchanged.
#
#
#
# Example 2:
#
#
# Input: grid = [[0,1],[1,2]]
#
# Output: [[2,1],[1,0]]
#
# Explanation:
#
#
#
# The diagonals with a black arrow must be non-increasing, so [0, 2] is changed
# to [2, 0]. The other diagonals are already in the correct order.
#
#
# Example 3:
#
#
# Input: grid = [[1]]
#
# Output: [[1]]
#
# Explanation:
#
# Diagonals with exactly one element are already in order, so no changes are
# needed.
#
#
#
# Constraints:
#
#
# grid.length == grid[i].length == n
# 1 <= n <= 10
# -10^5 <= grid[i][j] <= 10^5
#
#
#

# @lc code=start
class Solution:
  def sortMatrix(self, grid: list[list[int]]) -> list[list[int]]:
    from collections import defaultdict

    diagonals = defaultdict(list)

    for i, row in enumerate(grid):
      for j, val in enumerate(row):
        diagonals[i - j].append(val)

    for k in diagonals:
      diagonals[k].sort(reverse=k >= 0)

    for i in range(len(grid)):
      for j in range(len(grid[i])):
        grid[i][j] = diagonals[i - j].pop(0)

    return grid


# @lc code=end


def test_example_1():
  # Test case based on Example 1 from problem description
  assert Solution().sortMatrix([[1, 7, 3], [9, 8, 2], [4, 5, 6]]) == [
    [8, 2, 3],
    [9, 6, 7],
    [4, 5, 1],
  ]


def test_example_2():
  # Test case based on Example 2 from problem description
  assert Solution().sortMatrix([[0, 1], [1, 2]]) == [[2, 1], [1, 0]]


def test_example_3():
  # Test case based on Example 3 from problem description
  assert Solution().sortMatrix([[1]]) == [[1]]
