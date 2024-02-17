/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] 用栈实现队列
 *
 * https://leetcode.cn/problems/implement-queue-using-stacks/description/
 *
 * algorithms
 * Easy (67.84%)
 * Likes:    1053
 * Dislikes: 0
 * Total Accepted:    414.3K
 * Total Submissions: 610.8K
 * Testcase Example:  '["MyQueue","push","push","peek","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * 请你仅使用两个栈实现先入先出队列。队列应当支持一般队列支持的所有操作（push、pop、peek、empty）：
 *
 * 实现 MyQueue 类：
 *
 *
 * void push(int x) 将元素 x 推到队列的末尾
 * int pop() 从队列的开头移除并返回元素
 * int peek() 返回队列开头的元素
 * boolean empty() 如果队列为空，返回 true ；否则，返回 false
 *
 *
 * 说明：
 *
 *
 * 你 只能 使用标准的栈操作 —— 也就是只有 push to top, peek/pop from top, size, 和 is empty
 * 操作是合法的。
 * 你所使用的语言也许不支持栈。你可以使用 list 或者 deque（双端队列）来模拟一个栈，只要是标准的栈操作即可。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：
 * ["MyQueue", "push", "push", "peek", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * 输出：
 * [null, null, null, 1, 1, false]
 *
 * 解释：
 * MyQueue myQueue = new MyQueue();
 * myQueue.push(1); // queue is: [1]
 * myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
 * myQueue.peek(); // return 1
 * myQueue.pop(); // return 1, queue is [2]
 * myQueue.empty(); // return false
 *
 *
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= x <= 9
 * 最多调用 100 次 push、pop、peek 和 empty
 * 假设所有操作都是有效的 （例如，一个空的队列不会调用 pop 或者 peek 操作）
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你能否实现每个操作均摊时间复杂度为 O(1) 的队列？换句话说，执行 n 个操作的总时间复杂度为 O(n) ，即使其中一个操作可能花费较长时间。
 *
 *
 */

// @lc code=start
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn tidy_up(&mut self) {
        if self.stack2.is_empty() {
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap());
            }
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.tidy_up();
        self.stack2.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.tidy_up();
        *self.stack2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.peek(), 1);
    assert_eq!(obj.pop(), 1);
    assert_eq!(obj.empty(), false);
}
