#
# @lc app=leetcode.cn id=3531 lang=python3
#
# [3531] Count Covered Buildings
#
# https://leetcode.cn/problems/count-covered-buildings/description/
#
# algorithms
# Medium (43.04%)
# Likes:    4
# Dislikes: 0
# Total Accepted:    1.8K
# Total Submissions: 4.2K
# Testcase Example:  '3\n[[1,2],[2,2],[3,2],[2,1],[2,3]]'
#
# You are given a positive integer n, representing an n x n city. You are also
# given a 2D grid buildings, where buildings[i] = [x, y] denotes a unique
# building located at coordinates [x, y].
#
# A building is covered if there is at least one building in all four
# directions: left, right, above, and below.
#
# Return the number of covered buildings.
#
#
# Example 1:
#
#
#
#
# Input: n = 3, buildings = [[1,2],[2,2],[3,2],[2,1],[2,3]]
#
# Output: 1
#
# Explanation:
#
#
# Only building [2,2] is covered as it has at least one building:
#
#
# above ([1,2])
# below ([3,2])
# left ([2,1])
# right ([2,3])
#
#
# Thus, the count of covered buildings is 1.
#
#
#
# Example 2:
#
#
#
#
# Input: n = 3, buildings = [[1,1],[1,2],[2,1],[2,2]]
#
# Output: 0
#
# Explanation:
#
#
# No building has at least one building in all four directions.
#
#
#
# Example 3:
#
#
#
#
# Input: n = 5, buildings = [[1,3],[3,2],[3,3],[3,5],[5,3]]
#
# Output: 1
#
# Explanation:
#
#
# Only building [3,3] is covered as it has at least one building:
#
#
# above ([1,3])
# below ([5,3])
# left ([3,2])
# right ([3,5])
#
#
# Thus, the count of covered buildings is 1.
#
#
#
#
# Constraints:
#
#
# 2 <= n <= 10^5
# 1 <= buildings.length <= 10^5
# buildings[i] = [x, y]
# 1 <= x, y <= n
# All coordinates of buildings are unique.
#
#
#

# @lc code=start
class Solution:
  def countCoveredBuildings(self, n: int, buildings: list[list[int]]) -> int:
    buildings_x_max = [0] * (n + 1)
    buildings_x_min = [n + 1] * (n + 1)
    buildings_y_max = [0] * (n + 1)
    buildings_y_min = [n + 1] * (n + 1)
    for x, y in buildings:
      buildings_x_max[y] = max(buildings_x_max[y], x)
      buildings_x_min[y] = min(buildings_x_min[y], x)
      buildings_y_max[x] = max(buildings_y_max[x], y)
      buildings_y_min[x] = min(buildings_y_min[x], y)
    return sum(
      [
        buildings_x_min[y] < x < buildings_x_max[y]
        and buildings_y_min[x] < y < buildings_y_max[x]
        for x, y in buildings
      ]
    )


# @lc code=end


def test_example_1():
  assert (
    Solution().countCoveredBuildings(3, [[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]) == 1
  )


def test_example_2():
  assert Solution().countCoveredBuildings(3, [[1, 1], [1, 2], [2, 1], [2, 2]]) == 0
