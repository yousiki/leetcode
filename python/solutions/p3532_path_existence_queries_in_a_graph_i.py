#
# @lc app=leetcode.cn id=3532 lang=python3
#
# [3532] Path Existence Queries in a Graph I
#
# https://leetcode.cn/problems/path-existence-queries-in-a-graph-i/description/
#
# algorithms
# Medium (48.52%)
# Likes:    2
# Dislikes: 0
# Total Accepted:    1.4K
# Total Submissions: 3K
# Testcase Example:  '2\n[1,3]\n1\n[[0,0],[0,1]]'
#
# You are given an integer n representing the number of nodes in a graph,
# labeled from 0 to n - 1.
#
# You are also given an integer array nums of length n sorted in non-decreasing
# order, and an integer maxDiff.
#
# An undirected edge exists between nodes i and j if the absolute difference
# between nums[i] and nums[j] is at most maxDiff (i.e., |nums[i] - nums[j]| <=
# maxDiff).
#
# You are also given a 2D integer array queries. For each queries[i] = [ui,
# vi], determine whether there exists a path between nodes ui and vi.
#
# Return a boolean array answer, where answer[i] is true if there exists a path
# between ui and vi in the i^th query and false otherwise.
#
#
# Example 1:
#
#
# Input: n = 2, nums = [1,3], maxDiff = 1, queries = [[0,0],[0,1]]
#
# Output: [true,false]
#
# Explanation:
#
#
# Query [0,0]: Node 0 has a trivial path to itself.
# Query [0,1]: There is no edge between Node 0 and Node 1 because |nums[0] -
# nums[1]| = |1 - 3| = 2, which is greater than maxDiff.
# Thus, the final answer after processing all the queries is [true, false].
#
#
#
# Example 2:
#
#
# Input: n = 4, nums = [2,5,6,8], maxDiff = 2, queries =
# [[0,1],[0,2],[1,3],[2,3]]
#
# Output: [false,false,true,true]
#
# Explanation:
#
# The resulting graph is:
#
#
#
#
# Query [0,1]: There is no edge between Node 0 and Node 1 because |nums[0] -
# nums[1]| = |2 - 5| = 3, which is greater than maxDiff.
# Query [0,2]: There is no edge between Node 0 and Node 2 because |nums[0] -
# nums[2]| = |2 - 6| = 4, which is greater than maxDiff.
# Query [1,3]: There is a path between Node 1 and Node 3 through Node 2 since
# |nums[1] - nums[2]| = |5 - 6| = 1 and |nums[2] - nums[3]| = |6 - 8| = 2, both
# of which are within maxDiff.
# Query [2,3]: There is an edge between Node 2 and Node 3 because |nums[2] -
# nums[3]| = |6 - 8| = 2, which is equal to maxDiff.
# Thus, the final answer after processing all the queries is [false, false,
# true, true].
#
#
#
#
# Constraints:
#
#
# 1 <= n == nums.length <= 10^5
# 0 <= nums[i] <= 10^5
# nums is sorted in non-decreasing order.
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
  ) -> list[bool]:
    right_most = [i for i in range(n)]
    left_ptr = 0
    for i, x in enumerate(nums):
      if i > 0 and x - nums[i - 1] > maxDiff:
        for j in range(left_ptr, i):
          right_most[j] = i - 1
        left_ptr = i
    for j in range(left_ptr, n):
      right_most[j] = n
    return list(map(lambda query: right_most[min(query)] >= max(query), queries))


# @lc code=end


def test_example_1():
  assert Solution().pathExistenceQueries(2, [1, 3], 1, [[0, 0], [0, 1]]) == [
    True,
    False,
  ]


def test_example_2():
  assert Solution().pathExistenceQueries(
    4, [2, 5, 6, 8], 2, [[0, 1], [0, 2], [1, 3], [2, 3]]
  ) == [False, False, True, True]
