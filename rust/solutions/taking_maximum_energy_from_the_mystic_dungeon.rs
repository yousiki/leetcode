/*
 * @lc app=leetcode id=3147 lang=rust
 *
 * [3147] Taking Maximum Energy From the Mystic Dungeon
 *
 * https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/description/
 *
 * algorithms
 * Medium (41.72%)
 * Likes:    323
 * Dislikes: 27
 * Total Accepted:    65.7K
 * Total Submissions: 125.5K
 * Testcase Example:  '[5,2,-10,-5,1]\n3'
 *
 * In a mystic dungeon, n magicians are standing in a line. Each magician has
 * an attribute that gives you energy. Some magicians can give you negative
 * energy, which means taking energy from you.
 *
 * You have been cursed in such a way that after absorbing energy from magician
 * i, you will be instantly transported to magician (i + k). This process will
 * be repeated until you reach the magician where (i + k) does not exist.
 *
 * In other words, you will choose a starting point and then teleport with k
 * jumps until you reach the end of the magicians' sequence, absorbing all the
 * energy during the journey.
 *
 * You are given an array energy and an integer k. Return the maximum possible
 * energy you can gain.
 *
 * Note that when you are reach a magician, you must take energy from them,
 * whether it is negative or positive energy.
 *
 *
 * Example 1:
 *
 *
 * Input:  energy = [5,2,-10,-5,1], k = 3
 *
 * Output: 3
 *
 * Explanation: We can gain a total energy of 3 by starting from magician 1
 * absorbing 2 + 1 = 3.
 *
 *
 * Example 2:
 *
 *
 * Input: energy = [-2,-3,-1], k = 2
 *
 * Output: -1
 *
 * Explanation: We can gain a total energy of -1 by starting from magician
 * 2.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= energy.length <= 10^5
 * -1000 <= energy[i] <= 1000
 * 1 <= k <= energy.length - 1
 *
 *
 *
 * ​​​​​​
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut suffix = vec![0; k as usize];
        let mut answer = std::i32::MIN;
        for (i, e) in energy.iter().rev().enumerate() {
            let i = i % k as usize;
            suffix[i] = suffix[i] + e;
            answer = answer.max(suffix[i]);
        }
        answer
    }
}
// @lc code=end

#[cfg(test)]
mod test_taking_maximum_energy_from_the_mystic_dungeon {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
    }
}
