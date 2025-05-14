#
# @lc app=leetcode.cn id=3548 lang=python3
#
# [3548] Equal Sum Grid Partition II
#
# https://leetcode.cn/problems/equal-sum-grid-partition-ii/description/
#
# algorithms
# Hard (26.04%)
# Likes:    1
# Dislikes: 0
# Total Accepted:    1.1K
# Total Submissions: 4.1K
# Testcase Example:  '[[1,4],[2,3]]'
#
# You are given an m x n matrix grid of positive integers. Your task is to
# determine if it is possible to make either one horizontal or one vertical cut
# on the grid such that:
#
#
# Each of the two resulting sections formed by the cut is non-empty.
# The sum of elements in both sections is equal, or can be made equal by
# discounting at most one single cell in total (from either section).
# If a cell is discounted, the rest of the section must remain connected.
#
#
# Return true if such a partition exists; otherwise, return false.
#
# Note: A section is connected if every cell in it can be reached from any
# other cell by moving up, down, left, or right through other cells in the
# section.
#
#
# Example 1:
#
#
# Input: grid = [[1,4],[2,3]]
#
# Output: true
#
# Explanation:
#
#
#
#
# A horizontal cut after the first row gives sums 1 + 4 = 5 and 2 + 3 = 5,
# which are equal. Thus, the answer is true.
#
#
#
# Example 2:
#
#
# Input: grid = [[1,2],[3,4]]
#
# Output: true
#
# Explanation:
#
#
#
#
# A vertical cut after the first column gives sums 1 + 3 = 4 and 2 + 4 = 6.
# By discounting 2 from the right section (6 - 2 = 4), both sections have equal
# sums and remain connected. Thus, the answer is true.
#
#
#
# Example 3:
#
#
# Input: grid = [[1,2,4],[2,3,5]]
#
# Output: false
#
# Explanation:
#
#
#
#
# A horizontal cut after the first row gives 1 + 2 + 4 = 7 and 2 + 3 + 5 =
# 10.
# By discounting 3 from the bottom section (10 - 3 = 7), both sections have
# equal sums, but they do not remain connected as it splits the bottom section
# into two parts ([2] and [5]). Thus, the answer is false.
#
#
#
# Example 4:
#
#
# Input: grid = [[4,1,8],[3,2,6]]
#
# Output: false
#
# Explanation:
#
# No valid cut exists, so the answer is false.
#
#
#
# Constraints:
#
#
# 1 <= m == grid.length <= 10^5
# 1 <= n == grid[i].length <= 10^5
# 2 <= m * n <= 10^5
# 1 <= grid[i][j] <= 10^5
#
#
#

# @lc code=start
class Solution:
  def canPartitionGrid(self, grid: list[list[int]]) -> bool:
    return self.can_partition_horizontal(grid) or self.can_partition_vertical(grid)

  @staticmethod
  def can_partition_vertical(grid: list[list[int]]) -> bool:
    # Rotate the grid to reuse the horizontal partition logic
    rotated_grid = [list(row) for row in zip(*grid)]
    return Solution.can_partition_horizontal(rotated_grid)

  @staticmethod
  def can_partition_horizontal(grid: list[list[int]]) -> bool:
    from collections import Counter

    # Get the dimensions of the grid
    m = len(grid)
    n = len(grid[0]) if m > 0 else 0

    # Calculate the total sum of the grid
    total_sum = sum(sum(row) for row in grid)

    # Count the occurrences of each number in the grid
    total_counter = Counter([i for row in grid for i in row])

    # Try to cut horizontally
    upper_sum = 0
    upper_counter = Counter()
    for i in range(m - 1):
      upper_sum += sum(grid[i])
      upper_counter.update(Counter(grid[i]))
      lower_sum = total_sum - upper_sum
      # If the sums are equal, return True
      if upper_sum == lower_sum:
        return True
      # Try to discount one cell from the upper section
      upper_discount = upper_sum - lower_sum
      if upper_counter[upper_discount] > 0:
        # Make sure the remaining cells in the upper section are connected
        if i == 0:
          if grid[i][0] == upper_discount or grid[i][n - 1] == upper_discount:
            return True
        elif n == 1:
          if grid[0][0] == upper_discount or grid[i][0] == upper_discount:
            return True
        else:
          return True
      # Try to discount one cell from the lower section
      lower_discount = lower_sum - upper_sum
      if total_counter[lower_discount] > upper_counter[lower_discount]:
        # Make sure the remaining cells in the lower section are connected
        if i == m - 2:
          if grid[i + 1][0] == lower_discount or grid[i + 1][n - 1] == lower_discount:
            return True
        elif n == 1:
          if grid[i + 1][0] == lower_discount or grid[m - 1][0] == lower_discount:
            return True
        else:
          return True

    # If no valid cut was found, return False
    return False


# @lc code=end


def test_example_1():
  grid = [[1, 4], [2, 3]]
  assert Solution().canPartitionGrid(grid) == True


def test_example_2():
  grid = [[1, 2], [3, 4]]
  assert Solution().canPartitionGrid(grid) == True


def test_example_3():
  grid = [[1, 2, 4], [2, 3, 5]]
  assert Solution().canPartitionGrid(grid) == False


def test_example_4():
  grid = [[4, 1, 8], [3, 2, 6]]
  assert Solution().canPartitionGrid(grid) == False


def test_example_5():
  grid = [[10, 5, 4, 5]]
  assert Solution().canPartitionGrid(grid) == False
