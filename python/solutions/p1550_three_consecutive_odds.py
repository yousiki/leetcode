#
# @lc app=leetcode.cn id=1550 lang=python3
#
# [1550] Three Consecutive Odds
#
# https://leetcode.cn/problems/three-consecutive-odds/description/
#
# algorithms
# Easy (65.69%)
# Likes:    41
# Dislikes: 0
# Total Accepted:    40.2K
# Total Submissions: 60.4K
# Testcase Example:  '[2,6,4,1]'
#
# Given an integer array arr, return true if there are three consecutive odd
# numbers in the array. Otherwise, return false.
#
# Example 1:
#
#
# Input: arr = [2,6,4,1]
# Output: false
# Explanation: There are no three consecutive odds.
#
#
# Example 2:
#
#
# Input: arr = [1,2,34,3,4,5,7,23,12]
# Output: true
# Explanation: [5,7,23] are three consecutive odds.
#
#
#
# Constraints:
#
#
# 1 <= arr.length <= 1000
# 1 <= arr[i] <= 1000
#
#
#

# @lc code=start
class Solution:
  def threeConsecutiveOdds(self, arr: list[int]) -> bool:
    return any(
      map(
        lambda t: t[0] % 2 == 1 and t[1] % 2 == 1 and t[2] % 2 == 1,
        zip(arr, arr[1:], arr[2:]),
      )
    )


# @lc code=end


def test_example_1():
  assert Solution().threeConsecutiveOdds([2, 6, 4, 1]) is False


def test_example_2():
  assert Solution().threeConsecutiveOdds([1, 2, 34, 3, 4, 5, 7, 23, 12]) is True
