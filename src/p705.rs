/*
* @lc app=leetcode.cn id=705 lang=rust
*
* [705] 设计哈希集合
*
* https://leetcode.cn/problems/design-hashset/description/
*
* algorithms
* Easy (63.47%)
* Likes:    324
* Dislikes: 0
* Total Accepted:    110.7K
* Total Submissions: 174.4K
* Testcase Example:  '["MyHashSet","add","add","contains","contains","add","contains","remove","contains"]\n' +
 '[[],[1],[2],[1],[3],[2],[2],[2],[2]]'
*
* 不使用任何内建的哈希表库设计一个哈希集合（HashSet）。
*
* 实现 MyHashSet 类：
*
*
* void add(key) 向哈希集合中插入值 key 。
* bool contains(key) 返回哈希集合中是否存在这个值 key 。
* void remove(key) 将给定值 key 从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。
*
*
*
* 示例：
*
*
* 输入：
* ["MyHashSet", "add", "add", "contains", "contains", "add", "contains",
* "remove", "contains"]
* [[], [1], [2], [1], [3], [2], [2], [2], [2]]
* 输出：
* [null, null, null, true, false, null, true, null, false]
*
* 解释：
* MyHashSet myHashSet = new MyHashSet();
* myHashSet.add(1);      // set = [1]
* myHashSet.add(2);      // set = [1, 2]
* myHashSet.contains(1); // 返回 True
* myHashSet.contains(3); // 返回 False ，（未找到）
* myHashSet.add(2);      // set = [1, 2]
* myHashSet.contains(2); // 返回 True
* myHashSet.remove(2);   // set = [1]
* myHashSet.contains(2); // 返回 False ，（已移除）
*
*
*
* 提示：
*
*
* 0 <= key <= 10^6
* 最多调用 10^4 次 add、remove 和 contains
*
*
*/

// @lc code=start
struct MyHashSet {
    arr: Vec<i32>,
    len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            len: 100007,
            arr: vec![-1; 100007],
        }
    }

    fn find(&self, key: i32) -> i32 {
        let mut index = key % self.len;
        let end = (index + self.len) % self.len;
        while self.arr[index as usize] != -1 && self.arr[index as usize] != key && index != end {
            index = (index + 1) % self.len;
        }
        index
    }

    fn add(&mut self, key: i32) {
        let index = self.find(key);
        if self.arr[index as usize] == -1 {
            self.arr[index as usize] = key;
        }
    }

    fn remove(&mut self, key: i32) {
        let index = self.find(key);
        if self.arr[index as usize] == key {
            self.arr[index as usize] = -1;
        }
    }

    fn contains(&self, key: i32) -> bool {
        let index = self.find(key);
        self.arr[index as usize] == key
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
// @lc code=end

fn main() {
    let mut obj = MyHashSet::new();
    obj.add(1);
    obj.add(2);
    let ret_3: bool = obj.contains(1);
    assert_eq!(ret_3, true);
    let ret_4: bool = obj.contains(3);
    assert_eq!(ret_4, false);
    obj.add(2);
    let ret_5: bool = obj.contains(2);
    assert_eq!(ret_5, true);
    obj.remove(2);
    let ret_6: bool = obj.contains(2);
    assert_eq!(ret_6, false);
}
