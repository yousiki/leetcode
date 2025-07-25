// Category: algorithms
// Level: Medium
// Percent: 32.6274%

// You are given an array nums of distinct integers.
//
// In one operation, you can swap any two adjacent elements in the array.
//
// An arrangement of the array is considered valid if the parity of adjacent elements alternates, meaning every pair of neighboring elements consists of one even and one odd number.
//
// Return the minimum number of adjacent swaps required to transform nums into any valid arrangement.
//
// If it is impossible to rearrange nums such that no two adjacent elements have the same parity, return -1.
//
//
// Example 1:
//
//
// Input: nums = [2,4,6,5,7]
//
// Output: 3
//
// Explanation:
//
// Swapping 5 and 6, the array becomes [2,4,5,6,7]
//
// Swapping 5 and 4, the array becomes [2,5,4,6,7]
//
// Swapping 6 and 7, the array becomes [2,5,4,7,6]. The array is now a valid arrangement. Thus, the answer is 3.
//
//
// Example 2:
//
//
// Input: nums = [2,4,5,7]
//
// Output: 1
//
// Explanation:
//
// By swapping 4 and 5, the array becomes [2,5,4,7], which is a valid arrangement. Thus, the answer is 1.
//
//
// Example 3:
//
//
// Input: nums = [1,2,3]
//
// Output: 0
//
// Explanation:
//
// The array is already a valid arrangement. Thus, no operations are needed.
//
//
// Example 4:
//
//
// Input: nums = [4,5,6,8]
//
// Output: -1
//
// Explanation:
//
// No valid arrangement is possible. Thus, the answer is -1.
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 10⁵
// 	1 <= nums[i] <= 10⁹
// 	All elements in nums are distinct.
//

#[allow(unused)]
struct Solution;

#[allow(unused)]
// @lc code=start
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let num_odd = nums.iter().filter(|&x| x % 2 == 1).count() as i32;
        let num_even = nums.len() as i32 - num_odd;
        if (num_even - num_odd).abs() > 1 {
            -1
        } else if num_even < num_odd {
            Self::min_swaps_with_start(&nums, 1)
        } else if num_even > num_odd {
            Self::min_swaps_with_start(&nums, 0)
        } else {
            Self::min_swaps_with_start(&nums, 0).min(Self::min_swaps_with_start(&nums, 1))
        }
    }

    fn min_swaps_with_start(nums: &Vec<i32>, start_mod_2: i32) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(i, x)| *x % 2 == start_mod_2)
            .map(|(i, x)| i as i32)
            .enumerate()
            .map(|(j, i)| (i - j as i32 * 2).abs())
            .sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::min_swaps(vec![2, 4, 6, 5, 7]), 3);
        assert_eq!(Solution::min_swaps(vec![2, 4, 5, 7]), 1);
        assert_eq!(Solution::min_swaps(vec![1, 2, 3]), 0);
        assert_eq!(Solution::min_swaps(vec![4, 5, 6, 8]), -1);
    }
}
