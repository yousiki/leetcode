/*
 * @lc app=leetcode.cn id=3588 lang=rust
 *
 * [3588] Find Maximum Area of a Triangle
 *
 * https://leetcode.cn/problems/find-maximum-area-of-a-triangle/description/
 *
 * algorithms
 * Medium (45.65%)
 * Likes:    2
 * Dislikes: 0
 * Total Accepted:    1.3K
 * Total Submissions: 2.8K
 * Testcase Example:  '[[1,1],[1,2],[3,2],[3,3]]'
 *
 * You are given a 2D array coords of size n x 2, representing the coordinates
 * of n points in an infinite Cartesian plane.
 *
 * Find twice the maximum area of a triangle with its corners at any three
 * elements from coords, such that at least one side of this triangle is
 * parallel to the x-axis or y-axis. Formally, if the maximum area of such a
 * triangle is A, return 2 * A.
 *
 * If no such triangle exists, return -1.
 *
 * Note that a triangle cannot have zero area.
 *
 *
 * Example 1:
 *
 *
 * Input: coords = [[1,1],[1,2],[3,2],[3,3]]
 *
 * Output: 2
 *
 * Explanation:
 *
 *
 *
 * The triangle shown in the image has a base 1 and height 2. Hence its area is
 * 1/2 * base * height = 1.
 *
 *
 * Example 2:
 *
 *
 * Input: coords = [[1,1],[2,2],[3,3]]
 *
 * Output: -1
 *
 * Explanation:
 *
 * The only possible triangle has corners (1, 1), (2, 2), and (3, 3). None of
 * its sides are parallel to the x-axis or the y-axis.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n == coords.length <= 10^5
 * 1 <= coords[i][0], coords[i][1] <= 10^6
 * All coords[i] are unique.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        match Self::max_area_with_same_x(
            &coords
                .iter()
                .map(|v| (v[0], v[1]))
                .collect::<Vec<(i32, i32)>>(),
        )
        .max(Self::max_area_with_same_x(
            &coords
                .iter()
                .map(|v| (v[1], v[0]))
                .collect::<Vec<(i32, i32)>>(),
        )) {
            0 => -1,
            area => area,
        }
    }

    fn max_area_with_same_x(coords: &Vec<(i32, i32)>) -> i64 {
        use std::collections::HashMap;
        let mut max_y_by_x: HashMap<i32, i32> = HashMap::new();
        let mut min_y_by_x: HashMap<i32, i32> = HashMap::new();
        for &(x, y) in coords {
            max_y_by_x
                .entry(x)
                .and_modify(|v| *v = (*v).max(y))
                .or_insert(y);
            min_y_by_x
                .entry(x)
                .and_modify(|v| *v = (*v).min(y))
                .or_insert(y);
        }
        let xs = max_y_by_x.keys();
        let min_x = *xs.clone().min().unwrap();
        let max_x = *xs.clone().max().unwrap();
        xs.map(|x| {
            if let (Some(max_y), Some(min_y)) = (max_y_by_x.get(x), min_y_by_x.get(x)) {
                (max_y - min_y) as i64 * (x - min_x).max(max_x - x) as i64
            } else {
                0
            }
        })
        .max()
        .unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod test_find_maximum_area_of_a_triangle {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_area(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            -1
        );
    }
}
