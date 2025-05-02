#
# @lc app=leetcode.cn id=838 lang=python3
#
# [838] Push Dominoes
#
# https://leetcode.cn/problems/push-dominoes/description/
#
# algorithms
# Medium (55.95%)
# Likes:    340
# Dislikes: 0
# Total Accepted:    43.5K
# Total Submissions: 77.1K
# Testcase Example:  '"RR.L"'
#
# There are n dominoes in a line, and we place each domino vertically upright.
# In the beginning, we simultaneously push some of the dominoes either to the
# left or to the right.
#
# After each second, each domino that is falling to the left pushes the
# adjacent domino on the left. Similarly, the dominoes falling to the right
# push their adjacent dominoes standing on the right.
#
# When a vertical domino has dominoes falling on it from both sides, it stays
# still due to the balance of the forces.
#
# For the purposes of this question, we will consider that a falling domino
# expends no additional force to a falling or already fallen domino.
#
# You are given a string dominoes representing the initial state where:
#
#
# dominoes[i] = 'L', if the i^th domino has been pushed to the left,
# dominoes[i] = 'R', if the i^th domino has been pushed to the right, and
# dominoes[i] = '.', if the i^th domino has not been pushed.
#
#
# Return a string representing the final state.
#
#
# Example 1:
#
#
# Input: dominoes = "RR.L"
# Output: "RR.L"
# Explanation: The first domino expends no additional force on the second
# domino.
#
#
# Example 2:
#
#
# Input: dominoes = ".L.R...LR..L.."
# Output: "LL.RR.LLRRLL.."
#
#
#
# Constraints:
#
#
# n == dominoes.length
# 1 <= n <= 10^5
# dominoes[i] is either 'L', 'R', or '.'.
#
#
#

# @lc code=start
class Solution:
  def pushDominoes(self, dominoes: str) -> str:
    first_left = []
    last_left = None
    for i in range(len(dominoes)):
      if dominoes[i] == "R":
        last_left = i
      elif dominoes[i] == "L":
        last_left = None
      first_left.append(last_left)
    first_right = []
    last_right = None
    for i in range(len(dominoes) - 1, -1, -1):
      if dominoes[i] == "L":
        last_right = i
      elif dominoes[i] == "R":
        last_right = None
      first_right.append(last_right)
    first_right.reverse()
    ans = []
    for i in range(len(dominoes)):
      if dominoes[i] != ".":
        ans.append(dominoes[i])
      elif first_left[i] is not None and first_right[i] is None:
        ans.append("R")
      elif first_right[i] is not None and first_left[i] is None:
        ans.append("L")
      elif first_left[i] is None and first_right[i] is None:
        ans.append(".")
      else:
        left_distance = abs(i - first_left[i])
        right_distance = abs(i - first_right[i])
        if left_distance < right_distance:
          ans.append("R")
        elif left_distance > right_distance:
          ans.append("L")
        else:
          ans.append(".")
    return "".join(ans)


# @lc code=end


def test_example_1():
  # Example 1
  # Input: dominoes = "RR.L"
  # Output: "RR.L"
  # Explanation: The first domino expends no additional force on the second domino.
  assert Solution().pushDominoes("RR.L") == "RR.L"


def test_example_2():
  # Example 2
  # Input: dominoes = ".L.R...LR..L.."
  # Output: "LL.RR.LLRRLL.."
  assert Solution().pushDominoes(".L.R...LR..L..") == "LL.RR.LLRRLL.."


if __name__ == "__main__":
  test_example_1()
  test_example_2()
  print("All tests passed.")
