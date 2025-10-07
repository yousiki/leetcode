/*
 * @lc app=leetcode id=1488 lang=rust
 *
 * [1488] Avoid Flood in The City
 *
 * https://leetcode.com/problems/avoid-flood-in-the-city/description/
 *
 * algorithms
 * Medium (27.63%)
 * Likes:    1610
 * Dislikes: 323
 * Total Accepted:    45.3K
 * Total Submissions: 159.4K
 * Testcase Example:  '[1,2,3,4]'
 *
 * Your country has an infinite number of lakes. Initially, all the lakes are
 * empty, but when it rains over the nth lake, the nth lake becomes full of
 * water. If it rains over a lake that is full of water, there will be a flood.
 * Your goal is to avoid floods in any lake.
 *
 * Given an integer array rains where:
 *
 *
 * rains[i] > 0 means there will be rains over the rains[i] lake.
 * rains[i] == 0 means there are no rains this day and you can choose one lake
 * this day and dry it.
 *
 *
 * Return an array ans where:
 *
 *
 * ans.length == rains.length
 * ans[i] == -1 if rains[i] > 0.
 * ans[i] is the lake you choose to dry in the ith day if rains[i] == 0.
 *
 *
 * If there are multiple valid answers return any of them. If it is impossible
 * to avoid flood return an empty array.
 *
 * Notice that if you chose to dry a full lake, it becomes empty, but if you
 * chose to dry an empty lake, nothing changes.
 *
 *
 * Example 1:
 *
 *
 * Input: rains = [1,2,3,4]
 * Output: [-1,-1,-1,-1]
 * Explanation: After the first day full lakes are [1]
 * After the second day full lakes are [1,2]
 * After the third day full lakes are [1,2,3]
 * After the fourth day full lakes are [1,2,3,4]
 * There's no day to dry any lake and there is no flood in any lake.
 *
 *
 * Example 2:
 *
 *
 * Input: rains = [1,2,0,0,2,1]
 * Output: [-1,-1,2,1,-1,-1]
 * Explanation: After the first day full lakes are [1]
 * After the second day full lakes are [1,2]
 * After the third day, we dry lake 2. Full lakes are [1]
 * After the fourth day, we dry lake 1. There is no full lakes.
 * After the fifth day, full lakes are [2].
 * After the sixth day, full lakes are [1,2].
 * It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another
 * acceptable scenario.
 *
 *
 * Example 3:
 *
 *
 * Input: rains = [1,2,0,1,2]
 * Output: []
 * Explanation: After the second day, full lakes are  [1,2]. We have to dry one
 * lake in the third day.
 * After that, it will rain over lakes [1,2]. It's easy to prove that no matter
 * which lake you choose to dry in the 3rd day, the other one will flood.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= rains.length <= 10^5
 * 0 <= rains[i] <= 10^9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut next_rain = vec![usize::MAX; rains.len()];
        let mut last_appearance =
            std::collections::HashMap::<i32, usize>::with_capacity(rains.len());
        rains
            .iter()
            .enumerate()
            .rev()
            .filter(|&(_, &lake)| lake > 0)
            .for_each(|(day, &lake)| {
                if let Some(last_day) = last_appearance.insert(lake, day) {
                    next_rain[day] = last_day;
                }
            });
        let mut ans = vec![-1; rains.len()];
        let mut waiting =
            std::collections::BinaryHeap::<std::cmp::Reverse<usize>>::with_capacity(rains.len());
        for (day, &lake) in rains.iter().enumerate() {
            if lake == 0 {
                if let Some(std::cmp::Reverse(rain_day)) = waiting.pop() {
                    if rain_day < day {
                        return vec![];
                    }
                    ans[day] = rains[rain_day];
                } else {
                    ans[day] = 1;
                }
            } else {
                if next_rain[day] != usize::MAX {
                    waiting.push(std::cmp::Reverse(next_rain[day]));
                }
            }
        }
        if !waiting.is_empty() { vec![] } else { ans }
    }
}
// @lc code=end

#[cfg(test)]
mod test_avoid_flood_in_the_city {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 3, 4]),
            vec![-1, -1, -1, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
            vec![-1, -1, 2, 1, -1, -1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::avoid_flood(vec![0, 1, 1]), vec![]);
    }
}
