/*
 * @lc app=leetcode id=3719 lang=rust
 *
 * [3719] Longest Balanced Subarray I
 *
 * https://leetcode.com/problems/longest-balanced-subarray-i/description/
 *
 * algorithms
 * Medium (52.55%)
 * Likes:    88
 * Dislikes: 8
 * Total Accepted:    33.7K
 * Total Submissions: 61.9K
 * Testcase Example:  '[2,5,4,3]'
 *
 * You are given an integer array nums.
 *
 * A subarray is called balanced if the number of distinct even numbers in the
 * subarray is equal to the number of distinct odd numbers.
 *
 * Return the length of the longest balanced subarray.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,5,4,3]
 *
 * Output: 4
 *
 * Explanation:
 *
 *
 * The longest balanced subarray is [2, 5, 4, 3].
 * It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [5, 3].
 * Thus, the answer is 4.
 *
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,2,2,5,4]
 *
 * Output: 5
 *
 * Explanation:
 *
 *
 * The longest balanced subarray is [3, 2, 2, 5, 4].
 * It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [3, 5].
 * Thus, the answer is 5.
 *
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,2,3,2]
 *
 * Output: 3
 *
 * Explanation:
 *
 *
 * The longest balanced subarray is [2, 3, 2].
 * It has 1 distinct even number [2] and 1 distinct odd number [3]. Thus, the
 * answer is 3.
 *
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 1500
 * 1 <= nums[i] <= 10^5
 *
 *
 */

struct Solution;

// @lc code=start
struct UniqueCounter {
    n_even: u32,
    n_odd: u32,
    counters: Vec<u32>,
}

impl UniqueCounter {
    fn new(capacity: usize) -> Self {
        UniqueCounter {
            n_even: 0,
            n_odd: 0,
            counters: vec![0; capacity],
        }
    }

    fn add(&mut self, number: usize) -> bool {
        self.counters[number] += 1;
        if self.counters[number] == 1 {
            if number % 2 == 0 {
                self.n_even += 1;
            } else {
                self.n_odd += 1;
            }
        }
        self.n_even == self.n_odd
    }

    fn del(&mut self, number: usize) -> bool {
        assert!(self.counters[number] > 0);
        self.counters[number] -= 1;
        if self.counters[number] == 0 {
            if number % 2 == 0 {
                self.n_even -= 1;
            } else {
                self.n_odd -= 1;
            }
        }
        self.n_even == self.n_odd
    }
}

fn hash_numbers(nums: Vec<i32>) -> (Vec<usize>, usize) {
    let mut map = std::collections::HashMap::<i32, usize>::new();
    let mut hashed = Vec::new();
    let mut cnt_even = 0usize;
    let mut cnt_odd = 1usize;
    for x in nums {
        let y = if map.contains_key(&x) {
            *map.get(&x).unwrap()
        } else if x % 2 == 0 {
            cnt_even = cnt_even + 2;
            map.insert(x, cnt_even);
            cnt_even
        } else {
            cnt_odd = cnt_odd + 2;
            map.insert(x, cnt_odd);
            cnt_odd
        };
        hashed.push(y);
    }
    (hashed, cnt_even.max(cnt_odd) + 1)
}

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let (nums, capacity) = hash_numbers(nums);
        let mut max_balanced = 0;
        let mut counter = UniqueCounter::new(capacity);
        for left in 0..nums.len() {
            if left % 2 == 0 {
                for right in left..nums.len() {
                    if counter.add(nums[right]) {
                        max_balanced = max_balanced.max(right - left + 1);
                    }
                }
            } else {
                if counter.del(nums[left - 1]) {
                    max_balanced = max_balanced.max(nums.len() - left);
                }
                for right in (left..nums.len()).rev() {
                    if counter.del(nums[right]) {
                        max_balanced = max_balanced.max(right - left);
                    }
                }
            }
        }
        max_balanced as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test_longest_balanced_subarray_i {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
