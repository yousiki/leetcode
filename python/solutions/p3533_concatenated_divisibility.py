#
# @lc app=leetcode.cn id=3533 lang=python3
#
# [3533] Concatenated Divisibility
#
# https://leetcode.cn/problems/concatenated-divisibility/description/
#
# algorithms
# Hard (31.65%)
# Likes:    2
# Dislikes: 0
# Total Accepted:    690
# Total Submissions: 2.2K
# Testcase Example:  '[3,12,45]\n5'
#
# You are given an array of positive integers nums and a positive integer k.
#
# A permutation of nums is said to form a divisible concatenation if, when you
# concatenate the decimal representations of the numbers in the order specified
# by the permutation, the resulting number is divisible by k.
#
# Return the lexicographically smallest permutation (when considered as a list
# of integers) that forms a divisible concatenation. If no such permutation
# exists, return an empty list.
#
#
# Example 1:
#
#
# Input: nums = [3,12,45], k = 5
#
# Output: [3,12,45]
#
# Explanation:
#
#
#
#
# Permutation
# Concatenated Value
# Divisible by 5
#
#
#
#
# [3, 12, 45]
# 31245
# Yes
#
#
# [3, 45, 12]
# 34512
# No
#
#
# [12, 3, 45]
# 12345
# Yes
#
#
# [12, 45, 3]
# 12453
# No
#
#
# [45, 3, 12]
# 45312
# No
#
#
# [45, 12, 3]
# 45123
# No
#
#
#
#
# The lexicographically smallest permutation that forms a divisible
# concatenation is [3,12,45].
#
#
# Example 2:
#
#
# Input: nums = [10,5], k = 10
#
# Output: [5,10]
#
# Explanation:
#
#
#
#
# Permutation
# Concatenated Value
# Divisible by 10
#
#
#
#
# [5, 10]
# 510
# Yes
#
#
# [10, 5]
# 105
# No
#
#
#
#
# The lexicographically smallest permutation that forms a divisible
# concatenation is [5,10].
#
#
# Example 3:
#
#
# Input: nums = [1,2,3], k = 5
#
# Output: []
#
# Explanation:
#
# Since no permutation of nums forms a valid divisible concatenation, return an
# empty list.
#
#
#
# Constraints:
#
#
# 1 <= nums.length <= 13
# 1 <= nums[i] <= 10^5
# 1 <= k <= 100
#
#
#

# @lc code=start
class Solution:
  def concatenatedDivisibility(self, nums: list[int], k: int) -> list[int]:
    from functools import cache

    nums.sort()
    powers = [10 ** len(str(num)) for num in nums]
    answer = []

    @cache
    def search(unused: int, concat_mod_k: int) -> bool:
      if unused == 0:
        return concat_mod_k == 0
      for i in range(len(nums)):
        if (unused >> i) & 1 == 1:
          if search(unused ^ (1 << i), (concat_mod_k * powers[i] + nums[i]) % k):
            answer.append(nums[i])
            return True
      return False

    search(2 ** len(nums) - 1, 0)
    answer.reverse()
    return answer


# @lc code=end


def test_example_1():
  assert Solution().concatenatedDivisibility([3, 12, 45], 5) == [3, 12, 45]


def test_example_2():
  assert Solution().concatenatedDivisibility([10, 5], 10) == [5, 10]
