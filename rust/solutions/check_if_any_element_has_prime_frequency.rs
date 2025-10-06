/*
 * @lc app=leetcode.com id=3591 lang=rust
 *
 * [3591] Check if Any Element Has Prime Frequency
 *
 * https://leetcode.com/problems/check-if-any-element-has-prime-frequency/description/
 *
 * algorithms
 * Easy (60.89%)
 * Likes:    2
 * Dislikes: 0
 * Total Accepted:    4K
 * Total Submissions: 6.6K
 * Testcase Example:  '[1,2,3,4,5,4]'
 *
 * You are given an integer array nums.
 *
 * Return true if the frequency of any element of the array is prime,
 * otherwise, return false.
 *
 * The frequency of an element x is the number of times it occurs in the
 * array.
 *
 * A prime number is a natural number greater than 1 with only two factors, 1
 * and itself.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,4,5,4]
 *
 * Output: true
 *
 * Explanation:
 *
 * 4 has a frequency of two, which is a prime number.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3,4,5]
 *
 * Output: false
 *
 * Explanation:
 *
 * All elements have a frequency of one.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [2,2,2,4,4]
 *
 * Output: true
 *
 * Explanation:
 *
 * Both 2 and 4 have a prime frequency.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut freq = std::collections::HashMap::new();
        for x in nums {
            freq.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }
        for &x in freq.values() {
            if Self::is_prime(x) {
                return true;
            }
        }
        return false;
    }

    /// Check whether a number is prime
    fn is_prime(x: i32) -> bool {
        if x <= 1 {
            return false;
        }
        for i in 2..=(x as f64).sqrt().floor() as i32 {
            if x % i == 0 {
                return false;
            }
        }
        return true;
    }
}
// @lc code=end

#[cfg(test)]
mod test_check_if_any_element_has_prime_frequency {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]), true);
    }
}
