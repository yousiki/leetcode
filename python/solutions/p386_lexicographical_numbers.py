#
# @lc app=leetcode.cn id=386 lang=python3
#
# [386] Lexicographical Numbers
#
# https://leetcode.cn/problems/lexicographical-numbers/description/
#
# algorithms
# Medium (74.63%)
# Likes:    516
# Dislikes: 0
# Total Accepted:    85.3K
# Total Submissions: 114.2K
# Testcase Example:  '13'
#
# Given an integer n, return all the numbers in the range [1, n] sorted in
# lexicographical order.
#
# You must write an algorithm that runs in O(n) time and uses O(1) extra
# space.
#
#
# Example 1:
# Input: n = 13
# Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
# Example 2:
# Input: n = 2
# Output: [1,2]
#
#
# Constraints:
#
#
# 1 <= n <= 5 * 10^4
#
#
#

# @lc code=start
class Solution:
  def lexicalOrder(self, n: int) -> list[int]:
    answer, current = [], "1"
    while len(answer) < n:
      if int(current) <= n:
        answer.append(int(current))
        current = current + "0"
      else:
        current = current[:-1]
        current = str(int(current) + 1)
        while current[-1] == "0":
          current = current[:-1]
    return answer


# @lc code=end


def test_example_1():
  assert Solution().lexicalOrder(13) == [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]


def test_example_2():
  assert Solution().lexicalOrder(2) == [1, 2]


def test_example_3():
  assert Solution().lexicalOrder(34) == [
    1,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    2,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    3,
    30,
    31,
    32,
    33,
    34,
    4,
    5,
    6,
    7,
    8,
    9,
  ]
