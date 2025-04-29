#
# @lc app=leetcode.cn id=3529 lang=python3
#
# [3529] Count Cells in Overlapping Horizontal and Vertical Substrings
#
# https://leetcode.cn/problems/count-cells-in-overlapping-horizontal-and-vertical-substrings/description/
#
# algorithms
# Medium (34.35%)
# Likes:    2
# Dislikes: 0
# Total Accepted:    724
# Total Submissions: 2.1K
# Testcase Example:  '[["a","a","c","c"],["b","b","b","c"],["a","a","b","a"],["c","a","a","c"],["a","a","b","a"]]\n' + '"abaca"'
#
# You are given an m x n matrix grid consisting of characters and a string
# pattern.
#
# A horizontal substring is a contiguous sequence of characters read from left
# to right. If the end of a row is reached before the substring is complete, it
# wraps to the first column of the next row and continues as needed. You do not
# wrap from the bottom row back to the top.
#
# A vertical substring is a contiguous sequence of characters read from top to
# bottom. If the bottom of a column is reached before the substring is
# complete, it wraps to the first row of the next column and continues as
# needed. You do not wrap from the last column back to the first.
#
# Count the number of cells in the matrix that satisfy the following
# condition:
#
#
# The cell must be part of at least one horizontal substring and at least one
# vertical substring, where both substrings are equal to the given pattern.
#
#
# Return the count of these cells.
#
#
# Example 1:
#
#
# Input: grid =
# [["a","a","c","c"],["b","b","b","c"],["a","a","b","a"],["c","a","a","c"],["a","a","b","a"]],
# pattern = "abaca"
#
# Output: 1
#
# Explanation:
#
# The pattern "abaca" appears once as a horizontal substring (colored blue) and
# once as a vertical substring (colored red), intersecting at one cell (colored
# purple).
#
#
# Example 2:
#
#
# Input: grid =
# [["c","a","a","a"],["a","a","b","a"],["b","b","a","a"],["a","a","b","a"]],
# pattern = "aba"
#
# Output: 4
#
# Explanation:
#
# The cells colored above are all part of at least one horizontal and one
# vertical substring matching the pattern "aba".
#
#
# Example 3:
#
#
# Input: grid = [["a"]], pattern = "a"
#
# Output: 1
#
#
#
# Constraints:
#
#
# m == grid.length
# n == grid[i].length
# 1 <= m, n <= 1000
# 1 <= m * n <= 10^5
# 1 <= pattern.length <= m * n
# grid and pattern consist of only lowercase English letters.
#
#
#

# @lc code=start
class Solution:
  from functools import cache

  def countCells(self, grid: list[list[str]], pattern: str) -> int:
    # Get the dimensions of the grid
    m, n = len(grid), len(grid[0])
    # Get the horizontal concatenation of the grid
    horizontal = "".join("".join(row) for row in grid)
    # Get the vertical concatenation of the grid
    vertical = "".join("".join(grid[i][j] for i in range(m)) for j in range(n))
    # Get the valid cells for horizontal and vertical patterns
    horizontal_valid = self.mark_valid_cells(horizontal, pattern)
    vertical_valid = self.mark_valid_cells(vertical, pattern)
    # Count the number of valid cells that are both horizontally and vertically valid
    answer = 0
    for i in range(m):
      for j in range(n):
        # Calculate the index in the horizontal and vertical strings
        horizontal_index = i * n + j
        vertical_index = j * m + i
        # Check if the cell is valid in both horizontal and vertical patterns
        answer += int(
          horizontal_valid[horizontal_index] and vertical_valid[vertical_index]
        )
    return answer

  @classmethod
  def mark_valid_cells(cls, string: str, pattern: str) -> list[bool]:
    """
    Mark valid cells in the string that match the pattern.

    Args:
        string (str): The string to search within
        pattern (str): The pattern to search for

    Returns:
        set: Indices of valid cells in the string that match the pattern
    """
    # Find all occurrences of the pattern in the string using KMP algorithm
    indices = cls.kmp_search(string, pattern)
    # Create a array to mark valid cells
    valid = [False] * len(string)
    # Mark the cells that are part of the pattern
    last = 0
    for i in indices:
      for j in range(max(last, i), i + len(pattern)):
        valid[j] = True
      last = i + len(pattern)
    return valid

  @cache
  @staticmethod
  def compute_lps(pattern):
    """
    Compute Longest Prefix Suffix (LPS) array for KMP algorithm.

    Args:
        pattern (str): The pattern to create LPS array for

    Returns:
        list: LPS array where lps[i] is the length of the longest proper prefix
              that is also a suffix of pattern[0...i]
    """
    n = len(pattern)
    lps = [0] * n

    # Length of the previous longest prefix & suffix
    length = 0
    i = 1

    while i < n:
      if pattern[i] == pattern[length]:
        # Found matching character, increment length
        length += 1
        lps[i] = length
        i += 1
      else:
        if length != 0:
          # Try to find a shorter prefix that is also a suffix
          length = lps[length - 1]
        else:
          # No matching prefix found
          lps[i] = 0
          i += 1

    return lps

  @classmethod
  def kmp_search(cls, text, pattern):
    """
    Search for all occurrences of pattern in text using KMP algorithm.

    Args:
        text (str): The text to search within
        pattern (str): The pattern to search for

    Returns:
        list: Indices where pattern starts in the text
    """
    if not pattern:
      return []

    m, n = len(text), len(pattern)
    if n > m:
      return []

    # Compute the LPS array
    lps = cls.compute_lps(pattern)

    result = []
    i = 0  # Index for text
    j = 0  # Index for pattern

    while i < m:
      # Current characters match
      if pattern[j] == text[i]:
        i += 1
        j += 1

      # Found complete pattern
      if j == n:
        result.append(i - j)  # Pattern found at index i-j
        j = lps[j - 1]  # Look for the next match
      # Mismatch after j matches
      elif i < m and pattern[j] != text[i]:
        if j != 0:
          j = lps[j - 1]
        else:
          i += 1

    return result


# @lc code=end


def test_example_1():
  grid = [
    ["a", "a", "c", "c"],
    ["b", "b", "b", "c"],
    ["a", "a", "b", "a"],
    ["c", "a", "a", "c"],
    ["a", "a", "b", "a"],
  ]
  pattern = "abaca"
  expected = 1
  assert Solution().countCells(grid, pattern) == expected


def test_example_2():
  grid = [
    ["c", "a", "a", "a"],
    ["a", "a", "b", "a"],
    ["b", "b", "a", "a"],
    ["a", "a", "b", "a"],
  ]
  pattern = "aba"
  expected = 4
  assert Solution().countCells(grid, pattern) == expected
