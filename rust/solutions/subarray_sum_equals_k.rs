/*
 * @lc app=leetcode.com id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 *
 * https://leetcode.com/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (45.51%)
 * Likes:    2892
 * Dislikes: 0
 * Total Accepted:    855.8K
 * Total Submissions: 1.9M
 * Testcase Example:  '[1,1,1]\n2'
 *
 * Given an array of integers nums and an integer k, return the total number of
 * subarrays whose sum equals to k.
 *
 * A subarray is a contiguous non-empty sequence of elements within an
 * array.
 *
 *
 * Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -1000 <= nums[i] <= 1000
 * -10^7 <= k <= 10^7
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let accum = nums
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<i32>>();
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);
        for &x in &accum {
            if let Some(&v) = map.get(&(x - k)) {
                count += v;
            }
            *map.entry(x).or_insert(0) += 1;
        }
        count
    }
}
// @lc code=end

#[cfg(test)]
mod test_subarray_sum_equals_k {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, 2);
    }
}
