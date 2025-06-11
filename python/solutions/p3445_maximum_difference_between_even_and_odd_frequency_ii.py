#
# @lc app=leetcode.cn id=3445 lang=python3
#
# [3445] Maximum Difference Between Even and Odd Frequency II
#
# https://leetcode.cn/problems/maximum-difference-between-even-and-odd-frequency-ii/description/
#
# algorithms
# Hard (33.79%)
# Likes:    21
# Dislikes: 0
# Total Accepted:    3.3K
# Total Submissions: 6.6K
# Testcase Example:  '"12233"\n4'
#
# You are given a string s and an integer k. Your task is to find the maximum
# difference between the frequency of two characters, freq[a] - freq[b], in a
# substring subs of s, such that:
#
#
# subs has a size of at least k.
# Character a has an odd frequency in subs.
# Character b has an even frequency in subs.
#
#
# Return the maximum difference.
#
# Note that subs can contain more than 2 distinct characters.
#
#
# Example 1:
#
#
# Input: s = "12233", k = 4
#
# Output: -1
#
# Explanation:
#
# For the substring "12233", the frequency of '1' is 1 and the frequency of '3'
# is 2. The difference is 1 - 2 = -1.
#
#
# Example 2:
#
#
# Input: s = "1122211", k = 3
#
# Output: 1
#
# Explanation:
#
# For the substring "11222", the frequency of '2' is 3 and the frequency of '1'
# is 2. The difference is 3 - 2 = 1.
#
#
# Example 3:
#
#
# Input: s = "110", k = 3
#
# Output: -1
#
#
#
# Constraints:
#
#
# 3 <= s.length <= 3 * 10^4
# s consists only of digits '0' to '4'.
# The input is generated that at least one substring has a character with an
# even frequency and a character with an odd frequency.
# 1 <= k <= s.length
#
#
#

# @lc code=start
class Solution:
  def maxDifference(self, s: str, k: int) -> int:
    # Convert s from str to list[int]
    chars = [ord(c) - ord("0") for c in s]
    # Iterate over all possible characters a and b
    answer = float("-inf")
    for a in range(5):
      for b in range(5):
        # If a and b are the same, skip
        if a == b:
          continue
        # Find the maximum freq[a] - freq[b]
        # and freq[a] is odd and freq[b] is even
        # and the length of the substring is at least k
        freq_a_l, freq_a_r, freq_b_l, freq_b_r = 0, 0, 0, 0
        min_freq_diff = {(0, 0): (0, 0)}
        answer_ab = float("-inf")
        for r, c in enumerate(chars):
          # Update freq_*_r
          freq_a_r += int(c == a)
          freq_b_r += int(c == b)
          # Update freq_*_l
          if r >= k:
            freq_a_l += int(chars[r - k] == a)
            freq_b_l += int(chars[r - k] == b)
            # Update min_freq_diff
            if (freq_a_l % 2, freq_b_l % 2) not in min_freq_diff:
              min_freq_diff[(freq_a_l % 2, freq_b_l % 2)] = (freq_a_l, freq_b_l)
            else:
              min_freq_diff[freq_a_l % 2, freq_b_l % 2] = min(
                min_freq_diff[freq_a_l % 2, freq_b_l % 2],
                (freq_a_l, freq_b_l),
                key=lambda x: x[0] - x[1],
              )
          # Update the local answer
          if r >= k - 1:
            if (freq_a_r % 2 ^ 1, freq_b_r % 2) in min_freq_diff:
              best_freq_a_l, best_freq_b_l = min_freq_diff[
                (freq_a_r % 2 ^ 1, freq_b_r % 2)
              ]
              if best_freq_a_l != freq_a_r and best_freq_b_l != freq_b_r:
                answer_ab = max(
                  answer_ab, freq_a_r - best_freq_a_l - (freq_b_r - best_freq_b_l)
                )
        # Update the global answer
        answer = max(answer, answer_ab)
    return int(answer)


# @lc code=end


def test_example_1():
  # Test case based on Example 1 from problem description
  assert Solution().maxDifference("12233", 4) == -1


def test_example_2():
  # Test case based on Example 2 from problem description
  assert Solution().maxDifference("1122211", 3) == 1


def test_example_3():
  # Test case based on Example 3 from problem description
  assert Solution().maxDifference("110", 3) == -1
