/*
 * @lc app=leetcode.cn id=2616 lang=rust
 *
 * [2616] Minimize the Maximum Difference of Pairs
 *
 * https://leetcode.cn/problems/minimize-the-maximum-difference-of-pairs/description/
 *
 * algorithms
 * Medium (51.82%)
 * Likes:    87
 * Dislikes: 0
 * Total Accepted:    16.3K
 * Total Submissions: 31.5K
 * Testcase Example:  '[10,1,2,7,1,3]\n2'
 *
 * You are given a 0-indexed integer array nums and an integer p. Find p pairs
 * of indices of nums such that the maximum difference amongst all the pairs is
 * minimized. Also, ensure no index appears more than once amongst the p
 * pairs.
 *
 * Note that for a pair of elements at the index i and j, the difference of
 * this pair is |nums[i] - nums[j]|, where |x| represents the absolute value of
 * x.
 *
 * Return the minimum maximum difference among all p pairs. We define the
 * maximum of an empty set to be zero.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [10,1,2,7,1,3], p = 2
 * Output: 1
 * Explanation: The first pair is formed from the indices 1 and 4, and the
 * second pair is formed from the indices 2 and 5.
 * The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) =
 * max(0, 1) = 1. Therefore, we return 1.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,2,1,2], p = 1
 * Output: 0
 * Explanation: Let the indices 1 and 3 form a pair. The difference of that
 * pair is |2 - 2| = 0, which is the minimum we can attain.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] <= 10^9
 * 0 <= p <= (nums.length)/2
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        // Sort the array in ascending order
        let mut nums = nums;
        nums.sort();
        let nums = nums;
        // Binary search for the minimum maximum difference
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];
        let mut answer = right;
        while left <= right {
            let mid = (left + right) / 2;
            if Self::can_form_pairs(&nums, p, mid) {
                answer = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return answer;
    }

    /// Check if we can form `p` pairs with a maximum difference of `limit`
    /// Assumes `nums` is sorted in ascending order
    fn can_form_pairs(nums: &Vec<i32>, p: i32, limit: i32) -> bool {
        // Special case: if p is 0, we can always form 0 pairs
        if p == 0 {
            return true;
        }
        let mut count = 0; // Count of pairs formed
        let mut i = 0; // Index to iterate through the sorted array
        while i < nums.len() - 1 {
            // If the difference between the current and next element is leq to `mid`
            if nums[i + 1] - nums[i] <= limit {
                count += 1; // We can form a pair
                if count >= p {
                    return true; // We have formed enough pairs
                }
                i += 2; // Move to the next pair
            } else {
                i += 1; // Move to the next element
            }
        }
        return false; // Not enough pairs formed
    }
}

// @lc code=end
#[cfg(test)]
mod test_minimize_the_maximum_difference_of_pairs {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimize_max(vec![0, 5, 3, 4], 0), 0)
    }
}
