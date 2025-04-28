#
# @lc app=leetcode.cn id=3392 lang=python3
#
# [3392] Count Subarrays of Length Three With a Condition
#
# https://leetcode.cn/problems/count-subarrays-of-length-three-with-a-condition/description/
#
# algorithms
# Easy (69.53%)
# Likes:    27
# Dislikes: 0
# Total Accepted:    16.7K
# Total Submissions: 24K
# Testcase Example:  '[1,2,1,4,1]'
#
# Given an integer array nums, return the number of subarrays of length 3 such
# that the sum of the first and third numbers equals exactly half of the second
# number.
#
#
# Example 1:
#
#
# Input: nums = [1,2,1,4,1]
#
# Output: 1
#
# Explanation:
#
# Only the subarray [1,4,1] contains exactly 3 elements where the sum of the
# first and third numbers equals half the middle number.
#
#
# Example 2:
#
#
# Input: nums = [1,1,1]
#
# Output: 0
#
# Explanation:
#
# [1,1,1] is the only subarray of length 3. However, its first and third
# numbers do not add to half the middle number.
#
#
#
# Constraints:
#
#
# 3 <= nums.length <= 100
# -100 <= nums[i] <= 100
#
#
#

# @lc code=start
class Solution:
  def countSubarrays(self, nums: list[int]) -> int:
    return sum(
      [
        int((nums_i + nums_k) * 2 == nums_j)
        for nums_i, nums_j, nums_k in zip(nums, nums[1:], nums[2:])
      ]
    )


# @lc code=end


def test_example_1():
  assert Solution().countSubarrays([1, 2, 1, 4, 1]) == 1


def test_example_2():
  assert Solution().countSubarrays([1, 1, 1]) == 0
