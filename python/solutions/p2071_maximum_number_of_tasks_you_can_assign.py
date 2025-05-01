#
# @lc app=leetcode.cn id=2071 lang=python3
#
# [2071] Maximum Number of Tasks You Can Assign
#
# https://leetcode.cn/problems/maximum-number-of-tasks-you-can-assign/description/
#
# algorithms
# Hard (34.73%)
# Likes:    124
# Dislikes: 0
# Total Accepted:    10.1K
# Total Submissions: 27.5K
# Testcase Example:  '[3,2,1]\n[0,3,3]\n1\n1'
#
# You have n tasks and m workers. Each task has a strength requirement stored
# in a 0-indexed integer array tasks, with the i^th task requiring tasks[i]
# strength to complete. The strength of each worker is stored in a 0-indexed
# integer array workers, with the j^th worker having workers[j] strength. Each
# worker can only be assigned to a single task and must have a strength greater
# than or equal to the task's strength requirement (i.e., workers[j] >=
# tasks[i]).
#
# Additionally, you have pills magical pills that will increase a worker's
# strength by strength. You can decide which workers receive the magical pills,
# however, you may only give each worker at most one magical pill.
#
# Given the 0-indexed integer arrays tasks and workers and the integers pills
# and strength, return the maximum number of tasks that can be completed.
#
#
# Example 1:
#
#
# Input: tasks = [3,2,1], workers = [0,3,3], pills = 1, strength = 1
# Output: 3
# Explanation:
# We can assign the magical pill and tasks as follows:
# - Give the magical pill to worker 0.
# - Assign worker 0 to task 2 (0 + 1 >= 1)
# - Assign worker 1 to task 1 (3 >= 2)
# - Assign worker 2 to task 0 (3 >= 3)
#
#
# Example 2:
#
#
# Input: tasks = [5,4], workers = [0,0,0], pills = 1, strength = 5
# Output: 1
# Explanation:
# We can assign the magical pill and tasks as follows:
# - Give the magical pill to worker 0.
# - Assign worker 0 to task 0 (0 + 5 >= 5)
#
#
# Example 3:
#
#
# Input: tasks = [10,15,30], workers = [0,10,10,10,10], pills = 3, strength =
# 10
# Output: 2
# Explanation:
# We can assign the magical pills and tasks as follows:
# - Give the magical pill to worker 0 and worker 1.
# - Assign worker 0 to task 0 (0 + 10 >= 10)
# - Assign worker 1 to task 1 (10 + 10 >= 15)
# The last pill is not given because it will not make any worker strong enough
# for the last task.
#
#
#
# Constraints:
#
#
# n == tasks.length
# m == workers.length
# 1 <= n, m <= 5 * 10^4
# 0 <= pills <= m
# 0 <= tasks[i], workers[j], strength <= 10^9
#
#
#

# @lc code=start
class Solution:
  def maxTaskAssign(
    self, tasks: list[int], workers: list[int], pills: int, strength: int
  ) -> int:
    from sortedcontainers import SortedList

    tasks.sort()
    workers.sort(reverse=True)

    def is_valid(n_tasks: int) -> bool:
      """Check if n_tasks can be assigned to workers."""
      _tasks = tasks[:n_tasks]
      _workers = SortedList(workers[:n_tasks])
      _pills = pills
      for task in reversed(_tasks):
        # Try to assign a worker without pill
        # Search for the first worker >= task (worker > task - 1)
        idx = _workers.bisect_right(task - 1)
        # Assign this task to worker idx if possible
        if idx < len(_workers):
          _workers.pop(idx)
          continue
        # Try to assign a worker with pill
        if _pills == 0:
          return False
        # Search for the first worker >= task - strength
        idx = _workers.bisect_right(task - strength - 1)
        if idx < len(_workers):
          _workers.pop(idx)
          _pills -= 1
          continue
        # Cannot assign any worker
        return False
      return True

    # Binary search for the maximum number of tasks that can be assigned
    left, right = 0, min(len(tasks), len(workers))
    while left < right:
      mid = (left + right + 1) // 2
      # Check if we can assign mid tasks
      if is_valid(mid):
        left = mid
      else:
        right = mid - 1
    return left


# @lc code=end


def test_example_1():
  tasks = [3, 2, 1]
  workers = [0, 3, 3]
  pills = 1
  strength = 1
  expected = 3
  assert Solution().maxTaskAssign(tasks, workers, pills, strength) == expected


def test_example_2():
  tasks = [5, 4]
  workers = [0, 0, 0]
  pills = 1
  strength = 5
  expected = 1
  assert Solution().maxTaskAssign(tasks, workers, pills, strength) == expected


def test_example_3():
  tasks = [10, 15, 30]
  workers = [0, 10, 10, 10, 10]
  pills = 3
  strength = 10
  expected = 2
  assert Solution().maxTaskAssign(tasks, workers, pills, strength) == expected


def test_example_4():
  tasks = [5, 9, 8, 5, 9]
  workers = [1, 6, 4, 2, 6]
  pills = 1
  strength = 5
  expected = 3
  assert Solution().maxTaskAssign(tasks, workers, pills, strength) == expected
