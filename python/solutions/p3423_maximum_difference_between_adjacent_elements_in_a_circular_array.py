#
# @lc app=leetcode.cn id=3423 lang=python3
#
# [3423] Maximum Difference Between Adjacent Elements in a Circular Array
#
# https://leetcode.cn/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/
#
# algorithms
# Easy (86.33%)
# Likes:    5
# Dislikes: 0
# Total Accepted:    7.2K
# Total Submissions: 8.3K
# Testcase Example:  '[1,2,4]'
#
# Given a circular array nums, find the maximum absolute difference between
# adjacent elements.
#
# Note: In a circular array, the first and last elements are adjacent.
#
#
# Example 1:
#
#
# Input: nums = [1,2,4]
#
# Output: 3
#
# Explanation:
#
# Because nums is circular, nums[0] and nums[2] are adjacent. They have the
# maximum absolute difference of |4 - 1| = 3.
#
#
# Example 2:
#
#
# Input: nums = [-5,-10,-5]
#
# Output: 5
#
# Explanation:
#
# The adjacent elements nums[0] and nums[1] have the maximum absolute
# difference of |-5 - (-10)| = 5.
#
#
#
# Constraints:
#
#
# 2 <= nums.length <= 100
# -100 <= nums[i] <= 100
#
#
#

# @lc code=start
class Solution:
  def maxAdjacentDistance(self, nums: list[int]) -> int:
    return max(map(lambda x: abs(x[0] - x[1]), zip(nums, nums[1:] + nums[:1])))


# @lc code=end


def test_example_1():
  assert Solution().maxAdjacentDistance(nums=[1, 2, 4]) == 3


def test_example_2():
  assert Solution().maxAdjacentDistance(nums=[-5, -10, -5]) == 5
