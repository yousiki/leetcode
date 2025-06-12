#
# @lc app=leetcode.cn id=3578 lang=python3
#
# [3578] Count Partitions With Max-Min Difference at Most K
#
# https://leetcode.cn/problems/count-partitions-with-max-min-difference-at-most-k/description/
#
# algorithms
# Medium (40.74%)
# Likes:    9
# Dislikes: 0
# Total Accepted:    1.1K
# Total Submissions: 2.8K
# Testcase Example:  '[9,4,1,3,7]\n4'
#
# You are given an integer array nums and an integer k. Your task is to
# partition nums into one or more non-empty contiguous segments such that in
# each segment, the difference between its maximum and minimum elements is at
# most k.
#
# Return the total number of ways to partition nums under this condition.
#
# Since the answer may be too large, return it modulo 10^9 + 7.
#
#
# Example 1:
#
#
# Input: nums = [9,4,1,3,7], k = 4
#
# Output: 6
#
# Explanation:
#
# There are 6 valid partitions where the difference between the maximum and
# minimum elements in each segment is at most k = 4:
#
#
# [[9], [4], [1], [3], [7]]
# [[9], [4], [1], [3, 7]]
# [[9], [4], [1, 3], [7]]
# [[9], [4, 1], [3], [7]]
# [[9], [4, 1], [3, 7]]
# [[9], [4, 1, 3], [7]]
#
#
#
# Example 2:
#
#
# Input: nums = [3,3,4], k = 0
#
# Output: 2
#
# Explanation:
#
# There are 2 valid partitions that satisfy the given conditions:
#
#
# [[3], [3], [4]]
# [[3, 3], [4]]
#
#
#
#
# Constraints:
#
#
# 2 <= nums.length <= 5 * 10^4
# 1 <= nums[i] <= 10^9
# 0 <= k <= 10^9
#
#
#

# @lc code=start
class Solution:
  mod: int = int(10**9 + 7)

  def countPartitions(self, nums: list[int], k: int) -> int:
    nums = [None, *nums]  # Add a dummy element to make the index start from 1
    num_partitions = [1]  # Number of valid partitions for [1:i]

    queue_inc = []
    queue_dec = []
    queue_sum = 0
    queue_l = 1

    for queue_r in range(1, len(nums)):
      while len(queue_inc) > 0 and nums[queue_inc[-1]] >= nums[queue_r]:
        queue_inc.pop()
      queue_inc.append(queue_r)

      while len(queue_dec) > 0 and nums[queue_dec[-1]] <= nums[queue_r]:
        queue_dec.pop()
      queue_dec.append(queue_r)

      queue_sum = (queue_sum + num_partitions[queue_r - 1]) % self.mod

      while queue_l < queue_r and nums[queue_dec[0]] - nums[queue_inc[0]] > k:
        if queue_inc[0] == queue_l:
          queue_inc.pop(0)
        if queue_dec[0] == queue_l:
          queue_dec.pop(0)
        queue_sum = (queue_sum - num_partitions[queue_l - 1] + self.mod) % self.mod
        queue_l += 1

      num_partitions.append(queue_sum)

    return num_partitions[-1]


# @lc code=end


def test_example_1():
  assert Solution().countPartitions([9, 4, 1, 3, 7], 4) == 6


def test_example_2():
  assert Solution().countPartitions([3, 3, 4], 0) == 2
