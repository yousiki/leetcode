/*
 * @lc app=leetcode.cn id=1441 lang=rust
 *
 * [1441] Build an Array With Stack Operations
 *
 * https://leetcode.cn/problems/build-an-array-with-stack-operations/description/
 *
 * algorithms
 * Medium (72.75%)
 * Likes:    133
 * Dislikes: 0
 * Total Accepted:    58.4K
 * Total Submissions: 80.2K
 * Testcase Example:  '[1,3]\n3'
 *
 * You are given an integer array target and an integer n.
 *
 * You have an empty stack with the two following operations:
 *
 *
 * "Push": pushes an integer to the top of the stack.
 * "Pop": removes the integer on the top of the stack.
 *
 *
 * You also have a stream of the integers in the range [1, n].
 *
 * Use the two stack operations to make the numbers in the stack (from the
 * bottom to the top) equal to target. You should follow the following
 * rules:
 *
 *
 * If the stream of the integers is not empty, pick the next integer from the
 * stream and push it to the top of the stack.
 * If the stack is not empty, pop the integer at the top of the stack.
 * If, at any moment, the elements in the stack (from the bottom to the top)
 * are equal to target, do not read new integers from the stream and do not do
 * more operations on the stack.
 *
 *
 * Return the stack operations needed to build target following the mentioned
 * rules. If there are multiple valid answers, return any of them.
 *
 *
 * Example 1:
 *
 *
 * Input: target = [1,3], n = 3
 * Output: ["Push","Push","Pop","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of
 * the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Pop the integer on the top of the stack. s = [1].
 * Read 3 from the stream and push it to the stack. s = [1,3].
 *
 *
 * Example 2:
 *
 *
 * Input: target = [1,2,3], n = 3
 * Output: ["Push","Push","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of
 * the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Read 3 from the stream and push it to the stack. s = [1,2,3].
 *
 *
 * Example 3:
 *
 *
 * Input: target = [1,2], n = 4
 * Output: ["Push","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of
 * the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Since the stack (from the bottom to the top) is equal to target, we stop the
 * stack operations.
 * The answers that read integer 3 from the stream are not accepted.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= target.length <= 100
 * 1 <= n <= 100
 * 1 <= target[i] <= n
 * target is strictly increasing.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut operations = Vec::new();
        let mut stream_val = 1;
        for target_val in target {
            while stream_val < target_val {
                operations.push("Push".to_string());
                operations.push("Pop".to_string());
                stream_val += 1;
            }
            operations.push("Push".to_string());
            stream_val += 1;
        }
        operations
    }
}
// @lc code=end

#[cfg(test)]
mod test_build_an_array_with_stack_operations {
    use super::Solution;

    #[test]
    fn test1() {
        let target = vec![1, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        let expected = vec![
            "Push".to_string(),
            "Push".to_string(),
            "Pop".to_string(),
            "Push".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let target = vec![1, 2, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        let expected = vec!["Push".to_string(), "Push".to_string(), "Push".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let target = vec![1, 2];
        let n = 4;
        let result = Solution::build_array(target, n);
        let expected = vec!["Push".to_string(), "Push".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let target = vec![2, 3, 4];
        let n = 4;
        let result = Solution::build_array(target, n);
        let expected = vec![
            "Push".to_string(),
            "Pop".to_string(),
            "Push".to_string(),
            "Push".to_string(),
            "Push".to_string(),
        ];
        assert_eq!(result, expected);
    }
}
