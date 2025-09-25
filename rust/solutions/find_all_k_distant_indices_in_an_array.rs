/*
 * @lc app=leetcode.cn id=2200 lang=rust
 *
 * [2200] Find All K-Distant Indices in an Array
 *
 * https://leetcode.cn/problems/find-all-k-distant-indices-in-an-array/description/
 *
 * algorithms
 * Easy (64.55%)
 * Likes:    47
 * Dislikes: 0
 * Total Accepted:    29.6K
 * Total Submissions: 45.9K
 * Testcase Example:  '[3,4,9,1,3,9,5]\n9\n1'
 *
 * You are given a 0-indexed integer array nums and two integers key and k. A
 * k-distant index is an index i of nums for which there exists at least one
 * index j such that |i - j| <= k and nums[j] == key.
 *
 * Return a list of all k-distant indices sorted in increasing order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [3,4,9,1,3,9,5], key = 9, k = 1
 * Output: [1,2,3,4,5,6]
 * Explanation: Here, nums[2] == key and nums[5] == key.
 * - For index 0, |0 - 2| > k and |0 - 5| > k, so there is no j where |0 - j|
 * <= k and nums[j] == key. Thus, 0 is not a k-distant index.
 * - For index 1, |1 - 2| <= k and nums[2] == key, so 1 is a k-distant index.
 * - For index 2, |2 - 2| <= k and nums[2] == key, so 2 is a k-distant index.
 * - For index 3, |3 - 2| <= k and nums[2] == key, so 3 is a k-distant index.
 * - For index 4, |4 - 5| <= k and nums[5] == key, so 4 is a k-distant index.
 * - For index 5, |5 - 5| <= k and nums[5] == key, so 5 is a k-distant index.
 * - For index 6, |6 - 5| <= k and nums[5] == key, so 6 is a k-distant index.
 * Thus, we return [1,2,3,4,5,6] which is sorted in increasing order.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,2,2,2,2], key = 2, k = 2
 * Output: [0,1,2,3,4]
 * Explanation: For all indices i in nums, there exists some index j such that
 * |i - j| <= k and nums[j] == key, so every index is a k-distant index.
 * Hence, we return [0,1,2,3,4].
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 1000
 * 1 <= nums[i] <= 1000
 * key is an integer from the array nums.
 * 1 <= k <= nums.length
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut answer = vec![];
        for (i, &x) in nums.iter().enumerate() {
            if x == key {
                for j in (i as i32 - k).max(0) as usize..=(i + k as usize).min(nums.len() - 1) {
                    if let Some(&back) = answer.last() {
                        if back >= j as i32 {
                            continue;
                        }
                    }
                    answer.push(j as i32);
                }
            }
        }
        answer
    }
}
// @lc code=end

#[cfg(test)]
mod test_find_all_k_distant_indices_in_an_array {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
    }
}
