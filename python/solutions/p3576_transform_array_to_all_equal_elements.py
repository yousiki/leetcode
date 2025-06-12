#
# @lc app=leetcode.cn id=3576 lang=python3
#
# [3576] Transform Array to All Equal Elements
#
# https://leetcode.cn/problems/transform-array-to-all-equal-elements/description/
#
# algorithms
# Medium (42.31%)
# Likes:    0
# Dislikes: 0
# Total Accepted:    2.1K
# Total Submissions: 5.1K
# Testcase Example:  '[1,-1,1,-1,1]\n3'
#
# You are given an integer array nums of size n containing only 1 and -1, and
# an integer k.
#
# You can perform the following operation at most k times:
#
#
#
# Choose an index i (0 <= i < n - 1), and multiply both nums[i] and nums[i + 1]
# by -1.
#
#
#
# Note that you can choose the same index i more than once in different
# operations.
#
# Return true if it is possible to make all elements of the array equal after
# at most k operations, and false otherwise.
#
#
# Example 1:
#
#
# Input: nums = [1,-1,1,-1,1], k = 3
#
# Output: true
#
# Explanation:
#
# We can make all elements in the array equal in 2 operations as follows:
#
#
# Choose index i = 1, and multiply both nums[1] and nums[2] by -1. Now nums =
# [1,1,-1,-1,1].
# Choose index i = 2, and multiply both nums[2] and nums[3] by -1. Now nums =
# [1,1,1,1,1].
#
#
#
# Example 2:
#
#
# Input: nums = [-1,-1,-1,1,1,1], k = 5
#
# Output: false
#
# Explanation:
#
# It is not possible to make all array elements equal in at most 5
# operations.
#
#
#
# Constraints:
#
#
# 1 <= n == nums.length <= 10^5
# nums[i] is either -1 or 1.
# 1 <= k <= n
#
#
#

# @lc code=start
class Solution:
  def canMakeEqual(self, nums: list[int], k: int) -> bool:
    return self.can_make_equal_to(nums, k, 1) or self.can_make_equal_to(nums, k, -1)

  @staticmethod
  def can_make_equal_to(nums: list[int], k: int, target: int) -> bool:
    nums = nums.copy()
    for i in range(len(nums) - 1):
      if nums[i] != target:
        k -= 1
        nums[i] *= -1
        nums[i + 1] *= -1
    return k >= 0 and nums[-1] == target


# @lc code=end


def test_example_1():
  nums = [1, -1, 1, -1, 1]
  k = 2
  assert Solution().canMakeEqual(nums, k) == True


def test_example_2():
  nums = [-1, -1, -1, 1, 1, 1]
  k = 5
  assert Solution().canMakeEqual(nums, k) == False
