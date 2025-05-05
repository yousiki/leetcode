#
# @lc app=leetcode.cn id=1128 lang=python3
#
# [1128] Number of Equivalent Domino Pairs
#
# https://leetcode.cn/problems/number-of-equivalent-domino-pairs/description/
#
# algorithms
# Easy (54.66%)
# Likes:    188
# Dislikes: 0
# Total Accepted:    57.4K
# Total Submissions: 100.4K
# Testcase Example:  '[[1,2],[2,1],[3,4],[5,6]]'
#
# Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] =
# [c, d] if and only if either (a == c and b == d), or (a == d and b == c) -
# that is, one domino can be rotated to be equal to another domino.
#
# Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and
# dominoes[i] is equivalent to dominoes[j].
#
#
# Example 1:
#
#
# Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
# Output: 1
#
#
# Example 2:
#
#
# Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
# Output: 3
#
#
#
# Constraints:
#
#
# 1 <= dominoes.length <= 4 * 10^4
# dominoes[i].length == 2
# 1 <= dominoes[i][j] <= 9
#
#
#

# @lc code=start
class Solution:
  def numEquivDominoPairs(self, dominoes: list[list[int]]) -> int:
    from collections import Counter

    counter = Counter(map(tuple, map(sorted, dominoes)))
    return sum(v * (v - 1) // 2 for v in counter.values())


# @lc code=end


def test_example_1():
  assert Solution().numEquivDominoPairs([[1, 2], [2, 1], [3, 4], [5, 6]]) == 1


def test_example_2():
  assert Solution().numEquivDominoPairs([[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]) == 3
