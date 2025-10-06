/*
 * @lc app=leetcode.com id=11 lang=rust
 *
 * [11] Container With Most Water
 *
 * https://leetcode.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (61.56%)
 * Likes:    5652
 * Dislikes: 0
 * Total Accepted:    1.9M
 * Total Submissions: 3M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 *
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut height_index = height
            .iter()
            .enumerate()
            .map(|(i, &h)| (h, i as i32))
            .collect::<Vec<(i32, i32)>>();
        height_index.sort();
        height_index.reverse();
        let mut answer = 0;
        let (mut left, mut right) = (10001, -1);
        for (h, i) in height_index {
            left = left.min(i);
            right = right.max(i);
            answer = answer.max(h * (right - left).max(0));
        }
        answer
    }
}
// @lc code=end

#[cfg(test)]
mod test_container_with_most_water {
    use super::Solution;

    #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn test2() {
        let height = vec![1, 1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}
