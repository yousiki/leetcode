/*
 * @lc app=leetcode.cn id=2181 lang=rust
 *
 * [2181] 合并零之间的节点
 *
 * https://leetcode.cn/problems/merge-nodes-in-between-zeros/description/
 *
 * algorithms
 * Medium (84.10%)
 * Likes:    52
 * Dislikes: 0
 * Total Accepted:    30.4K
 * Total Submissions: 35K
 * Testcase Example:  '[0,3,1,0,4,5,2,0]'
 *
 * 给你一个链表的头节点 head ，该链表包含由 0 分隔开的一连串整数。链表的 开端 和 末尾 的节点都满足 Node.val == 0 。
 *
 * 对于每两个相邻的 0 ，请你将它们之间的所有节点合并成一个节点，其值是所有已合并节点的值之和。然后将所有 0 移除，修改后的链表不应该含有任何 0
 * 。
 *
 * 返回修改后链表的头节点 head 。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 * 输入：head = [0,3,1,0,4,5,2,0]
 * 输出：[4,11]
 * 解释：
 * 上图表示输入的链表。修改后的链表包含：
 * - 标记为绿色的节点之和：3 + 1 = 4
 * - 标记为红色的节点之和：4 + 5 + 2 = 11
 *
 *
 * 示例 2：
 *
 *
 *
 * 输入：head = [0,1,0,3,0,2,2,0]
 * 输出：[1,3,4]
 * 解释：
 * 上图表示输入的链表。修改后的链表包含：
 * - 标记为绿色的节点之和：1 = 1
 * - 标记为红色的节点之和：3 = 3
 * - 标记为黄色的节点之和：2 + 2 = 4
 *
 *
 *
 *
 * 提示：
 *
 *
 * 列表中的节点数目在范围 [3, 2 * 10^5] 内
 * 0 <= Node.val <= 1000
 * 不 存在连续两个 Node.val == 0 的节点
 * 链表的 开端 和 末尾 节点都满足 Node.val == 0
 *
 *
 */

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut sum = 0;
        let mut sums = Vec::new();
        while let Some(mut node) = head {
            if node.val == 0 {
                if sum != 0 {
                    sums.push(sum);
                    sum = 0;
                }
            } else {
                sum += node.val;
            }
            head = node.next.take();
        }
        let mut head = None;
        for sum in sums.into_iter().rev() {
            let mut node = ListNode::new(sum);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
// @lc code=end

fn main() {
    println!("Examples not available");
}
