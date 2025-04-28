#
# @lc app=leetcode.cn id=3534 lang=python3
#
# [3534] Path Existence Queries in a Graph II
#
# https://leetcode.cn/problems/path-existence-queries-in-a-graph-ii/description/
#
# algorithms
# Hard (30.43%)
# Likes:    3
# Dislikes: 0
# Total Accepted:    430
# Total Submissions: 1.4K
# Testcase Example:  '5\n[1,8,3,4,2]\n3\n[[0,3],[2,4]]'
#
# You are given an integer n representing the number of nodes in a graph,
# labeled from 0 to n - 1.
#
# You are also given an integer array nums of length n and an integer maxDiff.
#
# An undirected edge exists between nodes i and j if the absolute difference
# between nums[i] and nums[j] is at most maxDiff (i.e., |nums[i] - nums[j]| <=
# maxDiff).
#
# You are also given a 2D integer array queries. For each queries[i] = [ui,
# vi], find the minimum distance between nodes ui and vi. If no path exists
# between the two nodes, return -1 for that query.
#
# Return an array answer, where answer[i] is the result of the i^th query.
#
# Note: The edges between the nodes are unweighted.
#
#
# Example 1:
#
#
# Input: n = 5, nums = [1,8,3,4,2], maxDiff = 3, queries = [[0,3],[2,4]]
#
# Output: [1,1]
#
# Explanation:
#
# The resulting graph is:
#
#
#
#
#
#
# Query
# Shortest Path
# Minimum Distance
#
#
# [0, 3]
# 0 → 3
# 1
#
#
# [2, 4]
# 2 → 4
# 1
#
#
#
#
# Thus, the output is [1, 1].
#
#
# Example 2:
#
#
# Input: n = 5, nums = [5,3,1,9,10], maxDiff = 2, queries =
# [[0,1],[0,2],[2,3],[4,3]]
#
# Output: [1,2,-1,1]
#
# Explanation:
#
# The resulting graph is:
#
#
#
#
#
#
#
# Query
# Shortest Path
# Minimum Distance
#
#
# [0, 1]
# 0 → 1
# 1
#
#
# [0, 2]
# 0 → 1 → 2
# 2
#
#
# [2, 3]
# None
# -1
#
#
# [4, 3]
# 3 → 4
# 1
#
#
#
#
# Thus, the output is [1, 2, -1, 1].
#
# Example 3:
#
#
# Input: n = 3, nums = [3,6,1], maxDiff = 1, queries = [[0,0],[0,1],[1,2]]
#
# Output: [0,-1,-1]
#
# Explanation:
#
# There are no edges between any two nodes because:
#
#
# Nodes 0 and 1: |nums[0] - nums[1]| = |3 - 6| = 3 > 1
# Nodes 0 and 2: |nums[0] - nums[2]| = |3 - 1| = 2 > 1
# Nodes 1 and 2: |nums[1] - nums[2]| = |6 - 1| = 5 > 1
#
#
# Thus, no node can reach any other node, and the output is [0, -1, -1].
#
#
#
# Constraints:
#
#
# 1 <= n == nums.length <= 10^5
# 0 <= nums[i] <= 10^5
# 0 <= maxDiff <= 10^5
# 1 <= queries.length <= 10^5
# queries[i] == [ui, vi]
# 0 <= ui, vi < n
#
#
#

# @lc code=start
class Solution:
  def pathExistenceQueries(
    self, n: int, nums: list[int], maxDiff: int, queries: list[list[int]]
  ) -> list[int]:
    from bisect import bisect_left
    from functools import cache

    # Map nums to sorted unique list
    sorted_nums = list(sorted(set(nums)))

    def index(x: int):
      return bisect_left(sorted_nums, x)

    # right_most[i] is the rightmost index j such that i can reach j
    right_most: list[int] = [i for i in range(len(sorted_nums))]
    right_ptr: int = 0
    for i, num in enumerate(sorted_nums):
      # Find the rightmost index j such that nums[j] - num <= maxDiff
      while (
        right_ptr < len(sorted_nums) - 1 and sorted_nums[right_ptr + 1] - num <= maxDiff
      ):
        right_ptr += 1
      right_most[i] = right_ptr

    bit_length = len(sorted_nums).bit_length() + 1

    # right_most_st[j][i] is the rightmost index k such that i can reach in 2^j steps
    right_most_st: list[list[int]] = [right_most]
    for j in range(1, bit_length):
      right_most_st.append([right_most_st[j - 1][k] for k in right_most_st[j - 1]])

    # For each query, find the minimum distance between the two nodes
    def answer_query(query: list[int]) -> int:
      u, v = query
      if nums[u] == nums[v]:
        return int(u != v)
      u = index(nums[u])
      v = index(nums[v])
      u, v = min(u, v), max(u, v)
      if right_most_st[-1][u] < v:
        return -1
      return _answer_query(u, v)

    @cache
    def _answer_query(u: int, v: int) -> int:
      answer = 0
      for j in range(bit_length - 1, -1, -1):
        if right_most_st[j][u] < v or j == 0:
          u = right_most_st[j][u]
          answer += 1 << j
      if u < v:
        answer += 1
      return answer

    return list(map(answer_query, queries))


# @lc code=end


def test_example_1():
  assert Solution().pathExistenceQueries(5, [1, 8, 3, 4, 2], 3, [[0, 3], [2, 4]]) == [
    1,
    1,
  ]


def test_example_2():
  assert Solution().pathExistenceQueries(
    5, [5, 3, 1, 9, 10], 2, [[0, 1], [0, 2], [2, 3], [4, 3]]
  ) == [
    1,
    2,
    -1,
    1,
  ]


def test_example_3():
  assert Solution().pathExistenceQueries(3, [3, 6, 1], 1, [[0, 0], [0, 1], [1, 2]]) == [
    0,
    -1,
    -1,
  ]


def test_example_4():
  assert Solution().pathExistenceQueries(2, [15, 15], 18, [[0, 0], [1, 1], [1, 0]]) == [
    0,
    0,
    1,
  ]


def test_example_5():
  n = 100000
  nums = [i for i in range(n)]
  maxDiff = 1
  queries = [[0, n - 1]]
  assert Solution().pathExistenceQueries(n, nums, maxDiff, queries) == [n - 1]
