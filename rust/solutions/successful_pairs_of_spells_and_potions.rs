/*
 * @lc app=leetcode id=2300 lang=rust
 *
 * [2300] Successful Pairs of Spells and Potions
 *
 * https://leetcode.com/problems/successful-pairs-of-spells-and-potions/description/
 *
 * algorithms
 * Medium (46.05%)
 * Likes:    2909
 * Dislikes: 93
 * Total Accepted:    280.2K
 * Total Submissions: 593.8K
 * Testcase Example:  '[5,1,3]\n[1,2,3,4,5]\n7'
 *
 * You are given two positive integer arrays spells and potions, of length n
 * and m respectively, where spells[i] represents the strength of the i^th
 * spell and potions[j] represents the strength of the j^th potion.
 *
 * You are also given an integer success. A spell and potion pair is considered
 * successful if the product of their strengths is at least success.
 *
 * Return an integer array pairs of length n where pairs[i] is the number of
 * potions that will form a successful pair with the i^th spell.
 *
 *
 * Example 1:
 *
 *
 * Input: spells = [5,1,3], potions = [1,2,3,4,5], success = 7
 * Output: [4,0,3]
 * Explanation:
 * - 0^th spell: 5 * [1,2,3,4,5] = [5,10,15,20,25]. 4 pairs are successful.
 * - 1^st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
 * - 2^nd spell: 3 * [1,2,3,4,5] = [3,6,9,12,15]. 3 pairs are successful.
 * Thus, [4,0,3] is returned.
 *
 *
 * Example 2:
 *
 *
 * Input: spells = [3,1,2], potions = [8,5,8], success = 16
 * Output: [2,0,2]
 * Explanation:
 * - 0^th spell: 3 * [8,5,8] = [24,15,24]. 2 pairs are successful.
 * - 1^st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
 * - 2^nd spell: 2 * [8,5,8] = [16,10,16]. 2 pairs are successful.
 * Thus, [2,0,2] is returned.
 *
 *
 *
 * Constraints:
 *
 *
 * n == spells.length
 * m == potions.length
 * 1 <= n, m <= 10^5
 * 1 <= spells[i], potions[i] <= 10^5
 * 1 <= success <= 10^10
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort();
        spells
            .iter()
            .map(|&spell| {
                let first_potion_idx = Self::binary_search_potions(&potions, success, spell);
                (potions.len() - first_potion_idx) as i32
            })
            .collect()
    }

    fn binary_search_potions(potions: &Vec<i32>, success: i64, spell: i32) -> usize {
        let mut left = 0 as isize;
        let mut right = potions.len() as isize - 1;
        let mut answer = potions.len() as isize;
        while left <= right {
            let mid = (left + right) / 2;
            if potions[mid as usize] as i64 * spell as i64 >= success {
                answer = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        answer as usize
    }
}
// @lc code=end

#[cfg(test)]
mod test_successful_pairs_of_spells_and_potions {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
