/*
 * @lc app=leetcode.com id=1518 lang=rust
 *
 * [1518] Water Bottles
 *
 * https://leetcode.com/problems/water-bottles/description/
 *
 * algorithms
 * Easy (69.89%)
 * Likes:    190
 * Dislikes: 0
 * Total Accepted:    76.1K
 * Total Submissions: 107.4K
 * Testcase Example:  '9\n3'
 *
 * There are numBottles water bottles that are initially full of water. You can
 * exchange numExchange empty water bottles from the market with one full water
 * bottle.
 *
 * The operation of drinking a full water bottle turns it into an empty
 * bottle.
 *
 * Given the two integers numBottles and numExchange, return the maximum number
 * of water bottles you can drink.
 *
 *
 * Example 1:
 *
 *
 * Input: numBottles = 9, numExchange = 3
 * Output: 13
 * Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
 * Number of water bottles you can drink: 9 + 3 + 1 = 13.
 *
 *
 * Example 2:
 *
 *
 * Input: numBottles = 15, numExchange = 4
 * Output: 19
 * Explanation: You can exchange 4 empty bottles to get 1 full water bottle.
 * Number of water bottles you can drink: 15 + 3 + 1 = 19.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= numBottles <= 100
 * 2 <= numExchange <= 100
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drunk = num_bottles;
        let mut num_bottles = num_bottles;
        while num_bottles >= num_exchange {
            let new_exchanges = num_bottles / num_exchange;
            total_drunk += new_exchanges;
            let remaining_bottles = num_bottles % num_exchange;
            num_bottles = new_exchanges + remaining_bottles;
        }
        total_drunk
    }
}
// @lc code=end

#[cfg(test)]
mod test_water_bottles {
    use super::Solution;

    #[test]
    fn test1() {
        let num_bottles = 9;
        let num_exchange = 3;
        let result = Solution::num_water_bottles(num_bottles, num_exchange);
        assert_eq!(result, 13);
    }

    #[test]
    fn test2() {
        let num_bottles = 15;
        let num_exchange = 4;
        let result = Solution::num_water_bottles(num_bottles, num_exchange);
        assert_eq!(result, 19);
    }
}
