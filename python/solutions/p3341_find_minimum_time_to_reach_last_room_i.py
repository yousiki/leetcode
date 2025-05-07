#
# @lc app=leetcode.cn id=3341 lang=python3
#
# [3341] Find Minimum Time to Reach Last Room I
#
# https://leetcode.cn/problems/find-minimum-time-to-reach-last-room-i/description/
#
# algorithms
# Medium (38.38%)
# Likes:    33
# Dislikes: 0
# Total Accepted:    8.5K
# Total Submissions: 19K
# Testcase Example:  '[[0,4],[4,4]]'
#
# There is a dungeon with n x m rooms arranged as a grid.
#
# You are given a 2D array moveTime of size n x m, where moveTime[i][j]
# represents the minimum time in seconds when you can start moving to that
# room. You start from the room (0, 0) at time t = 0 and can move to an
# adjacent room. Moving between adjacent rooms takes exactly one second.
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
# Output: 6
#
# Explanation:
#
# The minimum time required is 6 seconds.
#
#
# At time t == 4, move from room (0, 0) to room (1, 0) in one second.
# At time t == 5, move from room (1, 0) to room (1, 1) in one second.
#
#
#
# Example 2:
#
#
# Input: moveTime = [[0,0,0],[0,0,0]]
#
# Output: 3
#
# Explanation:
#
# The minimum time required is 3 seconds.
#
#
# At time t == 0, move from room (0, 0) to room (1, 0) in one second.
# At time t == 1, move from room (1, 0) to room (1, 1) in one second.
# At time t == 2, move from room (1, 1) to room (1, 2) in one second.
#
#
#
# Example 3:
#
#
# Input: moveTime = [[0,1],[1,2]]
#
# Output: 3
#
#
#
# Constraints:
#
#
# 2 <= n == moveTime.length <= 50
# 2 <= m == moveTime[i].length <= 50
# 0 <= moveTime[i][j] <= 10^9
#
#
#

# @lc code=start
class Solution:
  def minTimeToReach(self, moveTime: list[list[int]]) -> int:
    from queue import PriorityQueue

    distance = [[None] * len(moveTime[0]) for _ in range(len(moveTime))]
    distance[0][0] = 0
    queue = PriorityQueue()
    queue.put((0, (0, 0)))

    while not queue.empty():
      dis, (x, y) = queue.get()
      if dis != distance[x][y]:
        continue  # Skip if the distance is not the same as the one in the queue
      for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
        nx, ny = x + dx, y + dy
        if nx < 0 or nx >= len(moveTime) or ny < 0 or ny >= len(moveTime[0]):
          continue  # Skip out of bounds points
        ndis = max(dis, moveTime[nx][ny]) + 1
        if distance[nx][ny] is None or distance[nx][ny] > ndis:
          distance[nx][ny] = ndis
          queue.put((ndis, (nx, ny)))

    return distance[-1][-1]


# @lc code=end


def test_example_1():
  moveTime = [[0, 4], [4, 4]]
  expected = 6
  assert Solution().minTimeToReach(moveTime) == expected


def test_example_2():
  moveTime = [[0, 0, 0], [0, 0, 0]]
  expected = 3
  assert Solution().minTimeToReach(moveTime) == expected


def test_example_3():
  moveTime = [[0, 1], [1, 2]]
  expected = 3
  assert Solution().minTimeToReach(moveTime) == expected
