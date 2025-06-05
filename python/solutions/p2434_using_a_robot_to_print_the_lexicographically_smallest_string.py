#
# @lc app=leetcode.cn id=2434 lang=python3
#
# [2434] Using a Robot to Print the Lexicographically Smallest String
#
# https://leetcode.cn/problems/using-a-robot-to-print-the-lexicographically-smallest-string/description/
#
# algorithms
# Medium (45.77%)
# Likes:    71
# Dislikes: 0
# Total Accepted:    9.1K
# Total Submissions: 20K
# Testcase Example:  '"zza"'
#
# You are given a string s and a robot that currently holds an empty string t.
# Apply one of the following operations until s and t are both empty:
#
#
# Remove the first character of a string s and give it to the robot. The robot
# will append this character to the string t.
# Remove the last character of a string t and give it to the robot. The robot
# will write this character on paper.
#
#
# Return the lexicographically smallest string that can be written on the
# paper.
#
#
# Example 1:
#
#
# Input: s = "zza"
# Output: "azz"
# Explanation: Let p denote the written string.
# Initially p="", s="zza", t="".
# Perform first operation three times p="", s="", t="zza".
# Perform second operation three times p="azz", s="", t="".
#
#
# Example 2:
#
#
# Input: s = "bac"
# Output: "abc"
# Explanation: Let p denote the written string.
# Perform first operation twice p="", s="c", t="ba".
# Perform second operation twice p="ab", s="c", t="".
# Perform first operation p="ab", s="", t="c".
# Perform second operation p="abc", s="", t="".
#
#
# Example 3:
#
#
# Input: s = "bdda"
# Output: "addb"
# Explanation: Let p denote the written string.
# Initially p="", s="bdda", t="".
# Perform first operation four times p="", s="", t="bdda".
# Perform second operation four times p="addb", s="", t="".
#
#
#
# Constraints:
#
#
# 1 <= s.length <= 10^5
# s consists of only English lowercase letters.
#
#
#

# @lc code=start
class Solution:
  def robotWithString(self, s: str) -> str:
    # Split string into chars with their indices
    chars = [(c, i) for i, c in enumerate(s)]
    # Sort chars by two keys: value and index
    chars.sort()
    # Initialize the arrays for simulation
    s_index, t, result = 0, [], []
    # Simulate operations to construct the result string
    for char, index in chars:
      # Push the chars from s to t until index is reached
      while s_index <= index:
        t.append(s[s_index])
        s_index += 1
      # Pop the chars from t to result until the end of t is larger than the current char
      while len(t) > 0 and t[-1] <= char:
        result.append(t.pop())
    # Pop the remaining chars from t to result
    result.extend(reversed(t))
    # Join the result array into a string
    return "".join(result)


# @lc code=end


def test_example_1():
  assert Solution().robotWithString("zza") == "azz"


def test_example_2():
  assert Solution().robotWithString("bac") == "abc"


def test_example_3():
  assert Solution().robotWithString("bdda") == "addb"
