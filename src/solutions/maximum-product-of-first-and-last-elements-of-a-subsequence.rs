// Category: algorithms
// Level: Medium
// Percent: 43.3739%

// You are given an integer array nums and an integer m.
//
// Return the maximum product of the first and last elements of any subsequence of nums of size m.
//
//
// Example 1:
//
//
// Input: nums = [-1,-9,2,3,-2,-3,1], m = 1
//
// Output: 81
//
// Explanation:
//
// The subsequence [-9] has the largest product of the first and last elements: -9 * -9 = 81. Therefore, the answer is 81.
//
//
// Example 2:
//
//
// Input: nums = [1,3,-5,5,6,-4], m = 3
//
// Output: 20
//
// Explanation:
//
// The subsequence [-5, 6, -4] has the largest product of the first and last elements.
//
//
// Example 3:
//
//
// Input: nums = [2,-1,2,-6,5,2,-5,7], m = 2
//
// Output: 35
//
// Explanation:
//
// The subsequence [5, 7] has the largest product of the first and last elements.
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 10⁵
// 	-10⁵ <= nums[i] <= 10⁵
// 	1 <= m <= nums.length
//

#[allow(unused)]
struct Solution;

#[allow(unused)]
// @lc code=start
impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let m = (m - 1) as usize;
        let mut answer = i64::MIN;
        let mut pre_max = i64::MIN;
        let mut pre_min = i64::MAX;

        for (i, &num) in nums.iter().enumerate() {
            if i >= m {
                pre_max = pre_max.max(nums[i - m] as i64);
                pre_min = pre_min.min(nums[i - m] as i64);
                answer = answer.max(pre_max * num as i64);
                answer = answer.max(pre_min * num as i64);
            }
        }

        answer
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1),
            81
        );
        assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
        assert_eq!(
            Solution::maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2),
            35
        );
    }
}
