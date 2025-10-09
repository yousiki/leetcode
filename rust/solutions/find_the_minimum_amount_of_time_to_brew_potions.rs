/*
 * @lc app=leetcode id=3494 lang=rust
 *
 * [3494] Find the Minimum Amount of Time to Brew Potions
 *
 * https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/description/
 *
 * algorithms
 * Medium (36.11%)
 * Likes:    215
 * Dislikes: 163
 * Total Accepted:    29K
 * Total Submissions: 56.9K
 * Testcase Example:  '[1,5,2,4]\n[5,1,4,2]'
 *
 * You are given two integer arrays, skill and mana, of length n and m,
 * respectively.
 *
 * In a laboratory, n wizards must brew m potions in order. Each potion has a
 * mana capacity mana[j] and must pass through all the wizards sequentially to
 * be brewed properly. The time taken by the i^th wizard on the j^th potion is
 * timeij = skill[i] * mana[j].
 *
 * Since the brewing process is delicate, a potion must be passed to the next
 * wizard immediately after the current wizard completes their work. This means
 * the timing must be synchronized so that each wizard begins working on a
 * potion exactly when it arrives. ​
 *
 * Return the minimum amount of time required for the potions to be brewed
 * properly.
 *
 *
 * Example 1:
 *
 *
 * Input: skill = [1,5,2,4], mana = [5,1,4,2]
 *
 * Output: 110
 *
 * Explanation:
 *
 *
 *
 *
 * Potion Number
 * Start time
 * Wizard 0 done by
 * Wizard 1 done by
 * Wizard 2 done by
 * Wizard 3 done by
 *
 *
 * 0
 * 0
 * 5
 * 30
 * 40
 * 60
 *
 *
 * 1
 * 52
 * 53
 * 58
 * 60
 * 64
 *
 *
 * 2
 * 54
 * 58
 * 78
 * 86
 * 102
 *
 *
 * 3
 * 86
 * 88
 * 98
 * 102
 * 110
 *
 *
 *
 *
 * As an example for why wizard 0 cannot start working on the 1^st potion
 * before time t = 52, consider the case where the wizards started preparing
 * the 1^st potion at time t = 50. At time t = 58, wizard 2 is done with the
 * 1^st potion, but wizard 3 will still be working on the 0^th potion till time
 * t = 60.
 *
 *
 * Example 2:
 *
 *
 * Input: skill = [1,1,1], mana = [1,1,1]
 *
 * Output: 5
 *
 * Explanation:
 *
 *
 * Preparation of the 0^th potion begins at time t = 0, and is completed by
 * time t = 3.
 * Preparation of the 1^st potion begins at time t = 1, and is completed by
 * time t = 4.
 * Preparation of the 2^nd potion begins at time t = 2, and is completed by
 * time t = 5.
 *
 *
 *
 * Example 3:
 *
 *
 * Input: skill = [1,2,3,4], mana = [1,2]
 *
 * Output: 21
 *
 *
 *
 * Constraints:
 *
 *
 * n == skill.length
 * m == mana.length
 * 1 <= n, m <= 5000
 * 1 <= mana[i], skill[i] <= 5000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let skill_acc = skill
            .iter()
            .rev()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        let mut available = vec![0; skill.len()];
        let mut finish = 0;
        for m in mana {
            finish = available
                .iter()
                .zip(skill_acc.iter())
                .map(|(&a, &sa)| a + sa as i64 * m as i64)
                .max()
                .unwrap();
            available = skill_acc
                .iter()
                .zip(skill.iter())
                .map(|(&sa, &s)| finish - (sa - s) as i64 * m as i64)
                .collect();
        }
        finish
    }
}
// @lc code=end

#[cfg(test)]
mod test_find_the_minimum_amount_of_time_to_brew_potions {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]), 5);
    }
}
