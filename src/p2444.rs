/*
 * @lc app=leetcode.cn id=2444 lang=rust
 *
 * [2444] Count Subarrays With Fixed Bounds
 *
 * https://leetcode.cn/problems/count-subarrays-with-fixed-bounds/description/
 *
 * algorithms
 * Hard (48.07%)
 * Likes:    128
 * Dislikes: 0
 * Total Accepted:    12.9K
 * Total Submissions: 23.6K
 * Testcase Example:  '[1,3,5,2,7,5]\n1\n5'
 *
 * You are given an integer array nums and two integers minK and maxK.
 *
 * A fixed-bound subarray of nums is a subarray that satisfies the following
 * conditions:
 *
 *
 * The minimum value in the subarray is equal to minK.
 * The maximum value in the subarray is equal to maxK.
 *
 *
 * Return the number of fixed-bound subarrays.
 *
 * A subarray is a contiguous part of an array.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
 * Output: 2
 * Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,1,1,1], minK = 1, maxK = 1
 * Output: 10
 * Explanation: Every subarray of nums is a fixed-bound subarray. There are 10
 * possible subarrays.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * 1 <= nums[i], minK, maxK <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut last_min_k: i32 = -1;
        let mut last_max_k: i32 = -1;
        let mut left_bound_min: i32 = 0;
        for right in 0..nums.len() {
            // Update with nums[right]
            if nums[right] < min_k || nums[right] > max_k {
                // Reset the boundaries
                left_bound_min = right as i32 + 1;
                last_min_k = -1;
                last_max_k = -1;
            } else {
                if nums[right] == min_k {
                    last_min_k = right as i32;
                }
                if nums[right] == max_k {
                    last_max_k = right as i32;
                }
            }
            // Calculate valid subarrays
            if last_min_k != -1 && last_max_k != -1 {
                let left_bound_max = last_min_k.min(last_max_k);
                ans += (left_bound_max - left_bound_min + 1) as i64;
            }
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
}

fn main() {
    let nums = vec![1, 3, 5, 2, 7, 5];
    let min_k = 1;
    let max_k = 5;
    let result = Solution::count_subarrays(nums, min_k, max_k);
    println!("{}", result); // Output: 2

    let nums = vec![1, 1, 1, 1];
    let min_k = 1;
    let max_k = 1;
    let result = Solution::count_subarrays(nums, min_k, max_k);
    println!("{}", result); // Output: 10
}
