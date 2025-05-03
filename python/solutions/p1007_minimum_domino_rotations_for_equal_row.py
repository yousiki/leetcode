#
# @lc app=leetcode.cn id=1007 lang=python3
#
# [1007] Minimum Domino Rotations For Equal Row
#
# https://leetcode.cn/problems/minimum-domino-rotations-for-equal-row/description/
#
# algorithms
# Medium (48.92%)
# Likes:    140
# Dislikes: 0
# Total Accepted:    12.5K
# Total Submissions: 24.7K
# Testcase Example:  '[2,1,2,4,2,2]\n[5,2,6,2,3,2]'
#
# In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom
# halves of the i^th domino. (A domino is a tile with two numbers from 1 to 6 -
# one on each half of the tile.)
#
# We may rotate the i^th domino, so that tops[i] and bottoms[i] swap values.
#
# Return the minimum number of rotations so that all the values in tops are the
# same, or all the values in bottoms are the same.
#
# If it cannot be done, return -1.
#
#
# Example 1:
#
#
# Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
# Output: 2
# Explanation:
# The first figure represents the dominoes as given by tops and bottoms: before
# we do any rotations.
# If we rotate the second and fourth dominoes, we can make every value in the
# top row equal to 2, as indicated by the second figure.
#
#
# Example 2:
#
#
# Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
# Output: -1
# Explanation:
# In this case, it is not possible to rotate the dominoes to make one row of
# values equal.
#
#
#
# Constraints:
#
#
# 2 <= tops.length <= 2 * 10^4
# bottoms.length == tops.length
# 1 <= tops[i], bottoms[i] <= 6
#
#
#

# @lc code=start
class Solution:
  def minDominoRotations(self, tops: list[int], bottoms: list[int]) -> int:
    def calc(a: list[int], b: list[int], tar: int) -> int:
      result = 0
      for ai, bi in zip(a, b):
        if ai == tar:
          continue
        elif bi == tar:
          result += 1
        else:
          return 2 * len(tops)
      return result

    answer = 2 * len(tops)
    for i in range(1, 7):
      answer = min(answer, calc(tops, bottoms, i))
      answer = min(answer, calc(bottoms, tops, i))
    return -1 if answer == 2 * len(tops) else answer


# @lc code=end


def test_example_1():
  tops = [2, 1, 2, 4, 2, 2]
  bottoms = [5, 2, 6, 2, 3, 2]
  expected = 2
  assert Solution().minDominoRotations(tops, bottoms) == expected


def test_example_2():
  tops = [3, 5, 1, 2, 3]
  bottoms = [3, 6, 3, 3, 4]
  expected = -1
  assert Solution().minDominoRotations(tops, bottoms) == expected
