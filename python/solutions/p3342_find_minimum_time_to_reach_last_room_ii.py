#
# @lc app=leetcode.cn id=3342 lang=python3
#
# [3342] Find Minimum Time to Reach Last Room II
#
# https://leetcode.cn/problems/find-minimum-time-to-reach-last-room-ii/description/
#
# algorithms
# Medium (46.19%)
# Likes:    18
# Dislikes: 0
# Total Accepted:    5.9K
# Total Submissions: 11.5K
# Testcase Example:  '[[0,4],[4,4]]'
#
# There is a dungeon with n x m rooms arranged as a grid.
#
# You are given a 2D array moveTime of size n x m, where moveTime[i][j]
# represents the minimum time in seconds when you can start moving to that
# room. You start from the room (0, 0) at time t = 0 and can move to an
# adjacent room. Moving between adjacent rooms takes one second for one move
# and two seconds for the next, alternating between the two.
#
# Return the minimum time to reach the room (n - 1, m - 1).
#
# Two rooms are adjacent if they share a common wall, either horizontally or
# vertically.
#
#
# Example 1:
#
#
# Input: moveTime = [[0,4],[4,4]]
#
# Output: 7
#
# Explanation:
#
# The minimum time required is 7 seconds.
#
#
# At time t == 4, move from room (0, 0) to room (1, 0) in one second.
# At time t == 5, move from room (1, 0) to room (1, 1) in two seconds.
#
#
#
# Example 2:
#
#
# Input: moveTime = [[0,0,0,0],[0,0,0,0]]
#
# Output: 6
#
# Explanation:
#
# The minimum time required is 6 seconds.
#
#
# At time t == 0, move from room (0, 0) to room (1, 0) in one second.
# At time t == 1, move from room (1, 0) to room (1, 1) in two seconds.
# At time t == 3, move from room (1, 1) to room (1, 2) in one second.
# At time t == 4, move from room (1, 2) to room (1, 3) in two seconds.
#
#
#
# Example 3:
#
#
# Input: moveTime = [[0,1],[1,2]]
#
# Output: 4
#
#
#
# Constraints:
#
#
# 2 <= n == moveTime.length <= 750
# 2 <= m == moveTime[i].length <= 750
# 0 <= moveTime[i][j] <= 10^9
#
#
#

# @lc code=start
class Solution:
  def minTimeToReach(self, moveTime: list[list[int]]) -> int:
    from collections import defaultdict
    from math import inf
    from queue import PriorityQueue

    n, m = len(moveTime), len(moveTime[0])
    distance = defaultdict(lambda: inf)
    distance[(0, 0, 0)] = 0
    queue = PriorityQueue()
    queue.put((0, 0, 0, 0))  # (dis, x, y, t)

    while not queue.empty():
      dis, x, y, t = queue.get()
      if (x, y) == (n - 1, m - 1):
        return dis
      if dis != distance[(x, y, t)]:
        continue
      for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
        nx, ny = x + dx, y + dy
        if nx < 0 or nx >= n or ny < 0 or ny >= m:
          continue
        ndis = max(dis, moveTime[nx][ny]) + t + 1
        nt = 1 - t
        if ndis < distance[(nx, ny, nt)]:
          distance[(nx, ny, nt)] = ndis
          queue.put((ndis, nx, ny, nt))

    raise RuntimeError("Should not reach here")


# @lc code=end


def test_example_1():
  moveTime = [[0, 4], [4, 4]]
  expected = 7
  assert Solution().minTimeToReach(moveTime) == expected


def test_example_2():
  moveTime = [[0, 0, 0, 0], [0, 0, 0, 0]]
  expected = 6
  assert Solution().minTimeToReach(moveTime) == expected


def test_example_3():
  moveTime = [[0, 1], [1, 2]]
  expected = 4
  assert Solution().minTimeToReach(moveTime) == expected
