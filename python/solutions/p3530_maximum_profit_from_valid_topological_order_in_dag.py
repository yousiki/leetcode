#
# @lc app=leetcode.cn id=3530 lang=python3
#
# [3530] Maximum Profit from Valid Topological Order in DAG
#
# https://leetcode.cn/problems/maximum-profit-from-valid-topological-order-in-dag/description/
#
# algorithms
# Hard (35.15%)
# Likes:    2
# Dislikes: 0
# Total Accepted:    625
# Total Submissions: 1.8K
# Testcase Example:  '2\n[[0,1]]\n[2,3]'
#
# You are given a Directed Acyclic Graph (DAG) with n nodes labeled from 0 to n
# - 1, represented by a 2D array edges, where edges[i] = [ui, vi] indicates a
# directed edge from node ui to vi. Each node has an associated score given in
# an array score, where score[i] represents the score of node i.
#
# You must process the nodes in a valid topological order. Each node is
# assigned a 1-based position in the processing order.
#
# The profit is calculated by summing up the product of each node's score and
# its position in the ordering.
#
# Return the maximum possible profit achievable with an optimal topological
# order.
#
# A topological order of a DAG is a linear ordering of its nodes such that for
# every directed edge u → v, node u comes before v in the ordering.
#
#
# Example 1:
#
#
# Input: n = 2, edges = [[0,1]], score = [2,3]
#
# Output: 8
#
# Explanation:
#
#
#
# Node 1 depends on node 0, so a valid order is [0,
# 1].
#
#
#
#
# Node
# Processing Order
# Score
# Multiplier
# Profit Calculation
#
#
#
#
# 0
# 1st
# 2
# 1
# 2 × 1 = 2
#
#
# 1
# 2nd
# 3
# 2
# 3 × 2 = 6
#
#
#
#
# The maximum total profit achievable over all valid topological orders is 2 +
# 6 = 8.
#
#
# Example 2:
#
#
# Input: n = 3, edges = [[0,1],[0,2]], score = [1,6,3]
#
# Output: 25
#
# Explanation:
#
#
#
# Nodes 1 and 2 depend on node 0, so the most optimal valid order is [0, 2,
# 1].
#
#
#
#
# Node
# Processing Order
# Score
# Multiplier
# Profit Calculation
#
#
#
#
# 0
# 1st
# 1
# 1
# 1 × 1 = 1
#
#
# 2
# 2nd
# 3
# 2
# 3 × 2 = 6
#
#
# 1
# 3rd
# 6
# 3
# 6 × 3 = 18
#
#
#
#
# The maximum total profit achievable over all valid topological orders is 1 +
# 6 + 18 = 25.
#
#
#
# Constraints:
#
#
# 1 <= n == score.length <= 22
# 1 <= score[i] <= 10^5
# 0 <= edges.length <= n * (n - 1) / 2
# edges[i] == [ui, vi] denotes a directed edge from ui to vi.
# 0 <= ui, vi < n
# ui != vi
# The input graph is guaranteed to be a DAG.
# There are no duplicate edges.
#
#
#

# @lc code=start
class Solution:
  max_profit: int | float

  def maxProfit(self, n: int, edges: list[list[int]], score: list[int]) -> int:
    adjacent_nodes = [[] for _ in range(n)]
    indegree = [0] * n
    for u, v in edges:
      adjacent_nodes[v].append(u)
      indegree[u] += 1

    self.max_profit = float("-inf")

    def search(used: int, profit: int):
      """
      Search for the maximum profit with the given used nodes.

      Args:
        used: The bitmask representing the used nodes.
        profit: The current profit.

      Note:
        The global `indegree` maintains the indegree of nodes corresponding to `used`.
      """
      # If all nodes are used, return 0.
      if used == (1 << n) - 1:
        self.max_profit = max(self.max_profit, profit)
        return
      # Iterate through unused nodes.
      unused_scores = [score[i] for i in range(n) if (used >> i) & 1 == 0]
      unused_scores.sort()
      unused_profit_bound = sum(
        (i + 1) * unused_score for i, unused_score in enumerate(unused_scores)
      )
      if profit + unused_profit_bound <= self.max_profit:
        return  # Prune the search space.
      num_unused = len(unused_scores)
      # Iterate through zero-indegree nodes.
      zero_indegree_nodes = [
        i for i in range(n) if indegree[i] == 0 and (used >> i) & 1 == 0
      ]
      zero_indegree_nodes.sort(key=lambda x: -score[x])
      for i in zero_indegree_nodes:
        # Mark the node as used.
        new_used = used | (1 << i)
        # Update the indegree of adjacent nodes.
        for j in adjacent_nodes[i]:
          indegree[j] -= 1
        # Calculate the profit and search recursively.
        search(new_used, profit + score[i] * num_unused)
        # Backtrack: unmark the node and restore the indegree of adjacent nodes.
        for j in adjacent_nodes[i]:
          indegree[j] += 1

    search(0, 0)

    return self.max_profit


# @lc code=end


def test_example_1():
  n = 2
  edges = [[0, 1]]
  score = [2, 3]
  expected = 8
  assert Solution().maxProfit(n, edges, score) == expected


def test_example_2():
  n = 3
  edges = [[0, 1], [0, 2]]
  score = [1, 6, 3]
  expected = 25
  assert Solution().maxProfit(n, edges, score) == expected


def test_example_3():
  n = 4
  edges = [[1, 2]]
  score = [60098, 57669, 86595, 58482]
  expected = 701307
  assert Solution().maxProfit(n, edges, score) == expected


def test_example_4():
  n = 22
  edges = []
  score = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
  ]
  expected = 3795
  assert Solution().maxProfit(n, edges, score) == expected


def test_example_5():
  n = 3
  edges = [[0, 2]]
  score = [60084, 34608, 25733]
  expected = 231975
  assert Solution().maxProfit(n, edges, score) == expected
