/*
 * @lc app=leetcode.cn id=1860 lang=rust
 *
 * [1860] 增长的内存泄露
 *
 * https://leetcode.cn/problems/incremental-memory-leak/description/
 *
 * algorithms
 * Medium (76.29%)
 * Likes:    13
 * Dislikes: 0
 * Total Accepted:    7.4K
 * Total Submissions: 9.7K
 * Testcase Example:  '2\n2'
 *
 * 给你两个整数 memory1 和 memory2 分别表示两个内存条剩余可用内存的位数。现在有一个程序每秒递增的速度消耗着内存。
 *
 * 在第 i 秒（秒数从 1 开始），有 i 位内存被分配到 剩余内存较多 的内存条（如果两者一样多，则分配到第一个内存条）。如果两者剩余内存都不足 i
 * 位，那么程序将 意外退出 。
 *
 * 请你返回一个数组，包含 [crashTime, memory1crash, memory2crash] ，其中
 * crashTime是程序意外退出的时间（单位为秒）， memory1crash 和 memory2crash
 * 分别是两个内存条最后剩余内存的位数。
 *
 *
 *
 * 示例 1：
 *
 * 输入：memory1 = 2, memory2 = 2
 * 输出：[3,1,0]
 * 解释：内存分配如下：
 * - 第 1 秒，内存条 1 被占用 1 位内存。内存条 1 现在有 1 位剩余可用内存。
 * - 第 2 秒，内存条 2 被占用 2 位内存。内存条 2 现在有 0 位剩余可用内存。
 * - 第 3 秒，程序意外退出，两个内存条分别有 1 位和 0 位剩余可用内存。
 *
 *
 * 示例 2：
 *
 * 输入：memory1 = 8, memory2 = 11
 * 输出：[6,0,4]
 * 解释：内存分配如下：
 * - 第 1 秒，内存条 2 被占用 1 位内存，内存条 2 现在有 10 位剩余可用内存。
 * - 第 2 秒，内存条 2 被占用 2 位内存，内存条 2 现在有 8 位剩余可用内存。
 * - 第 3 秒，内存条 1 被占用 3 位内存，内存条 1 现在有 5 位剩余可用内存。
 * - 第 4 秒，内存条 2 被占用 4 位内存，内存条 2 现在有 4 位剩余可用内存。
 * - 第 5 秒，内存条 1 被占用 5 位内存，内存条 1 现在有 0 位剩余可用内存。
 * - 第 6 秒，程序意外退出，两个内存条分别有 0 位和 4 位剩余可用内存。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= memory1, memory2 <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        if memory1 == 0 && memory2 == 0 {
            return vec![1, 0, 0];
        }
        let diff = (memory1 as i64 - memory2 as i64).abs();
        // 1 + 2 + ... + n <= diff < 1 + 2 + ... + (n+1)
        // n(n+1)/2 <= diff < (n+1)(n+2)/2
        // n^2 + n - 2diff <= 0 < n^2 + 3n + 2 - 2diff
        // n <= (-1 + sqrt(1 + 8diff)) / 2 < n + 1
        let n = ((-1.0 + (1.0 + 8.0 * diff as f64).sqrt()) / 2.0).floor() as i64;
        let n = n.max(1);
        let mut mem1 = if memory1 >= memory2 {
            memory1 as i64 - n * (n + 1) / 2
        } else {
            memory1 as i64
        };
        let mut mem2 = if memory1 >= memory2 {
            memory2 as i64
        } else {
            memory2 as i64 - n * (n + 1) / 2
        };
        let mut i = n + 1;
        while mem1 >= i || mem2 >= i {
            if mem1 >= mem2 {
                mem1 -= i;
            } else {
                mem2 -= i;
            }
            i += 1;
        }
        vec![i as i32, mem1 as i32, mem2 as i32]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::mem_leak(2, 2), vec![3, 1, 0]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::mem_leak(8, 11), vec![6, 0, 4]);
    }
}

fn main() {
    println!("{:?}", Solution::mem_leak(2, 2));
    println!("{:?}", Solution::mem_leak(8, 11));
}
