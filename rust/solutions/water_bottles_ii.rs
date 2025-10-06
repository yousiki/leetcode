/*
 * @lc app=leetcode.com id=3100 lang=rust
 *
 * [3100] Water Bottles II
 *
 * https://leetcode.com/problems/water-bottles-ii/description/
 *
 * algorithms
 * Medium (68.26%)
 * Likes:    16
 * Dislikes: 0
 * Total Accepted:    14.8K
 * Total Submissions: 19.6K
 * Testcase Example:  '13\n6'
 *
 * You are given two integers numBottles and numExchange.
 *
 * numBottles represents the number of full water bottles that you initially
 * have. In one operation, you can perform one of the following
 * operations:
 *
 *
 * Drink any number of full water bottles turning them into empty bottles.
 * Exchange numExchange empty bottles with one full water bottle. Then,
 * increase numExchange by one.
 *
 *
 * Note that you cannot exchange multiple batches of empty bottles for the same
 * value of numExchange. For example, if numBottles == 3 and numExchange == 1,
 * you cannot exchange 3 empty water bottles for 3 full bottles.
 *
 * Return the maximum number of water bottles you can drink.
 *
 *
 * Example 1:
 *
 *
 * Input: numBottles = 13, numExchange = 6
 * Output: 15
 * Explanation: The table above shows the number of full water bottles, empty
 * water bottles, the value of numExchange, and the number of bottles drunk.
 *
 *
 * Example 2:
 *
 *
 * Input: numBottles = 10, numExchange = 3
 * Output: 13
 * Explanation: The table above shows the number of full water bottles, empty
 * water bottles, the value of numExchange, and the number of bottles
 * drunk.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= numBottles <= 100
 * 1 <= numExchange <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drunk = 0;
        let mut full_bottles = num_bottles;
        let mut empty_bottles = 0;
        let mut num_exchange = num_exchange;
        while full_bottles > 0 {
            total_drunk += full_bottles;
            empty_bottles += full_bottles;
            full_bottles = 0;
            while empty_bottles >= num_exchange {
                empty_bottles -= num_exchange;
                full_bottles += 1;
                num_exchange += 1;
            }
        }
        total_drunk
    }
}
// @lc code=end

#[cfg(test)]
mod test_water_bottles_ii {
    use super::Solution;

    #[test]
    fn test1() {
        let num_bottles = 13;
        let num_exchange = 6;
        let result = Solution::max_bottles_drunk(num_bottles, num_exchange);
        assert_eq!(result, 15);
    }

    #[test]
    fn test2() {
        let num_bottles = 10;
        let num_exchange = 3;
        let result = Solution::max_bottles_drunk(num_bottles, num_exchange);
        assert_eq!(result, 13);
    }
}
