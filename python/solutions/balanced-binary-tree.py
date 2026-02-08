#
# @lc app=leetcode id=110 lang=python3
#
# [110] Balanced Binary Tree
#
# https://leetcode.com/problems/balanced-binary-tree/description/
#
# algorithms
# Easy (57.10%)
# Likes:    11922
# Dislikes: 826
# Total Accepted:    2.3M
# Total Submissions: 4M
# Testcase Example:  '[3,9,20,null,null,15,7]'
#
# Given a binary tree, determine if it is height-balanced.
#
#
# Example 1:
#
#
# Input: root = [3,9,20,null,null,15,7]
# Output: true
#
#
# Example 2:
#
#
# Input: root = [1,2,2,3,3,null,null,4,4]
# Output: false
#
#
# Example 3:
#
#
# Input: root = []
# Output: true
#
#
#
# Constraints:
#
#
# The number of nodes in the tree is in the range [0, 5000].
# -10^4 <= Node.val <= 10^4
#
#
#
from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        return self.is_balanced_with_height(root) is not None

    @classmethod
    def is_balanced_with_height(cls, root: Optional[TreeNode]) -> Optional[int]:
        if root is None:
            return 0
        left_height = cls.is_balanced_with_height(root.left)
        right_height = cls.is_balanced_with_height(root.right)
        if left_height is None or right_height is None:
            return None
        if abs(left_height - right_height) > 1:
            return None
        return max(left_height, right_height) + 1


# @lc code=end
