#
# @lc app=leetcode.cn id=845 lang=python3
#
# [845] Longest Mountain in Array
#
# https://leetcode.cn/problems/longest-mountain-in-array/description/
#
# algorithms
# Medium (42.92%)
# Likes:    301
# Dislikes: 0
# Total Accepted:    59.2K
# Total Submissions: 138K
# Testcase Example:  '[2,1,4,7,3,2,5]'
#
# You may recall that an array arr is a mountain array if and only if:
#
#
# arr.length >= 3
# There exists some index i (0-indexed) with 0 < i < arr.length - 1 such
# that:
#
# arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
# arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
#
#
#
#
# Given an integer array arr, return the length of the longest subarray, which
# is a mountain. Return 0 if there is no mountain subarray.
#
#
# Example 1:
#
#
# Input: arr = [2,1,4,7,3,2,5]
# Output: 5
# Explanation: The largest mountain is [1,4,7,3,2] which has length 5.
#
#
# Example 2:
#
#
# Input: arr = [2,2,2]
# Output: 0
# Explanation: There is no mountain.
#
#
#
# Constraints:
#
#
# 1 <= arr.length <= 10^4
# 0 <= arr[i] <= 10^4
#
#
#
# Follow up:
#
#
# Can you solve it using only one pass?
# Can you solve it in O(1) space?
#
#
#

# @lc code=start
class Solution:
  def longestMountain(self, arr: list[int]) -> int:
    left_most = [0]
    for i in range(1, len(arr)):
      if arr[i] > arr[i - 1]:
        left_most.append(left_most[-1] + 1)
      else:
        left_most.append(0)
    right_most = [0]
    for i in range(len(arr) - 2, -1, -1):
      if arr[i] > arr[i + 1]:
        right_most.append(right_most[-1] + 1)
      else:
        right_most.append(0)
    right_most.reverse()
    longest = list(
      map(
        lambda p: p[0] + p[1] + 1,
        filter(lambda p: p[0] > 0 and p[1] > 0, zip(left_most, right_most)),
      )
    )
    return max(longest) if len(longest) > 0 else 0


# @lc code=end


def test_example_1():
  assert Solution().longestMountain([2, 1, 4, 7, 3, 2, 5]) == 5


def test_example_2():
  assert Solution().longestMountain([2, 2, 2]) == 0
