/*
 * @lc app=leetcode.cn id=2517 lang=rust
 *
 * [2517] Maximum Tastiness of Candy Basket
 *
 * https://leetcode.cn/problems/maximum-tastiness-of-candy-basket/description/
 *
 * algorithms
 * Medium (72.89%)
 * Likes:    192
 * Dislikes: 0
 * Total Accepted:    30.7K
 * Total Submissions: 42.2K
 * Testcase Example:  '[13,5,1,8,21,2]\n3'
 *
 * You are given an array of positive integers price where price[i] denotes the
 * price of the i^th candy and a positive integer k.
 *
 * The store sells baskets of k distinct candies. The tastiness of a candy
 * basket is the smallest absolute difference of the prices of any two candies
 * in the basket.
 *
 * Return the maximum tastiness of a candy basket.
 *
 *
 * Example 1:
 *
 *
 * Input: price = [13,5,1,8,21,2], k = 3
 * Output: 8
 * Explanation: Choose the candies with the prices [13,5,21].
 * The tastiness of the candy basket is: min(|13 - 5|, |13 - 21|, |5 - 21|) =
 * min(8, 8, 16) = 8.
 * It can be proven that 8 is the maximum tastiness that can be achieved.
 *
 *
 * Example 2:
 *
 *
 * Input: price = [1,3,1], k = 2
 * Output: 2
 * Explanation: Choose the candies with the prices [1,3].
 * The tastiness of the candy basket is: min(|1 - 3|) = min(2) = 2.
 * It can be proven that 2 is the maximum tastiness that can be achieved.
 *
 *
 * Example 3:
 *
 *
 * Input: price = [7,7,7,7], k = 2
 * Output: 0
 * Explanation: Choosing any two distinct candies from the candies we have will
 * result in a tastiness of 0.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= k <= price.length <= 10^5
 * 1 <= price[i] <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        let mut price = price;
        price.sort();
        Self::binary_search(&price, k)
    }

    fn binary_search(price: &Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = price[price.len() - 1] - price[0];
        let mut answer = 0;
        while left <= right {
            let mid = (left + right) / 2;
            if Self::valid_tastiness(price, k, mid) {
                answer = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        answer
    }

    fn valid_tastiness(price: &Vec<i32>, k: i32, tastiness: i32) -> bool {
        let mut count = 1;
        let mut last = price[0];
        for &p in price.iter().skip(1) {
            if p - last >= tastiness {
                count += 1;
                last = p;
            }
            if count >= k {
                return true;
            }
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod test_maximum_tastiness_of_candy_basket {
    use super::Solution;

    #[test]
    fn test_1() {
        let price = vec![13, 5, 1, 8, 21, 2];
        let k = 3;
        let result = Solution::maximum_tastiness(price, k);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let price = vec![1, 3, 1];
        let k = 2;
        let result = Solution::maximum_tastiness(price, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let price = vec![7, 7, 7, 7];
        let k = 2;
        let result = Solution::maximum_tastiness(price, k);
        assert_eq!(result, 0);
    }
}
