#
# @lc app=leetcode.cn id=2962 lang=python3
#
# [2962] Count Subarrays Where Max Element Appears at Least K Times
#
# https://leetcode.cn/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/
#
# algorithms
# Medium (56.60%)
# Likes:    67
# Dislikes: 0
# Total Accepted:    22.6K
# Total Submissions: 38.3K
# Testcase Example:  '[1,3,2,3,3]\n2'
#
# You are given an integer array nums and a positive integer k.
#
# Return the number of subarrays where the maximum element of nums appears at
# least k times in that subarray.
#
# A subarray is a contiguous sequence of elements within an array.
#
#
# Example 1:
#
#
# Input: nums = [1,3,2,3,3], k = 2
# Output: 6
# Explanation: The subarrays that contain the element 3 at least 2 times are:
# [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
#
#
# Example 2:
#
#
# Input: nums = [1,4,2,1], k = 3
# Output: 0
# Explanation: No subarray contains the element 4 at least 3 times.
#
#
#
# Constraints:
#
#
# 1 <= nums.length <= 10^5
# 1 <= nums[i] <= 10^6
# 1 <= k <= 10^5
#
#
#

# @lc code=start
class Solution:
  def countSubarrays(self, nums: list[int], k: int) -> int:
    n, max_num = len(nums), max(nums)
    indices = [i for i in range(len(nums)) if nums[i] == max_num]
    answer, min_left = 0, 0
    for i, index_i in enumerate(indices):
      # Consider left bounary in [min_left..max_left]
      max_left = indices[i]
      # Find the possible right boundary
      if i + k - 1 < len(indices):
        min_right = indices[i + k - 1]
      else:  # Impossible to find a right boundary
        continue
      max_right = n - 1
      # Accumulate the number of valid subarrays
      # [min_left..max_left] * [min_right..max_right]
      answer += (max_left - min_left + 1) * (max_right - min_right + 1)
      # Update the left boundary for the next iteration
      min_left = max_left + 1
    return answer


# @lc code=end


def test_example_1():
  assert Solution().countSubarrays([1, 3, 2, 3, 3], 2) == 6


def test_example_2():
  assert Solution().countSubarrays([1, 4, 2, 1], 3) == 0
