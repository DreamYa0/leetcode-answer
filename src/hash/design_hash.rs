/**
 * 强化练习 6：设计哈希集合
不使用任何内建的哈希表库设计一个哈希集合（HashSet）。

实现 MyHashSet 类：

void add(key) 向哈希集合中插入值 key 。
bool contains(key) 返回哈希集合中是否存在这个值 key 。
void remove(key) 将给定值 key 从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。

示例：

输入：
["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
[[], [1], [2], [1], [3], [2], [2], [2], [2]]
输出：
[null, null, null, true, false, null, true, null, false]

解释：
MyHashSet myHashSet = new MyHashSet();
myHashSet.add(1);      // set = [1]
myHashSet.add(2);      // set = [1, 2]
myHashSet.contains(1); // 返回 True
myHashSet.contains(3); // 返回 False ，（未找到）
myHashSet.add(2);      // set = [1, 2]
myHashSet.contains(2); // 返回 True
myHashSet.remove(2);   // set = [1]
myHashSet.contains(2); // 返回 False ，（已移除）


提示：

0 <= key <= 106
最多调用 104 次 add、remove 和 contains
 */
#[allow(dead_code)]
struct MyHashSet {
    val: Vec<u64>,
}

#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        Self {
            val: vec![0; 15626],
        }
    }

    fn add(&mut self, key: i32) {
        // 将key对应的位置置为1
        self.val[key as usize / 64] |= 1 << key as usize % 64;
    }

    fn remove(&mut self, key: i32) {
        // 将key对应的位置置为0
        self.val[key as usize / 64] &= !(1 << key as usize % 64);
    }

    fn contains(&self, key: i32) -> bool {
        // 判断key对应的位置是否为1
        self.val[key as usize / 64] & 1 << key as usize % 64 > 0
    }
}

/**
强化练习 7：设计哈希映射
不使用任何内建的哈希表库设计一个哈希映射（HashMap）。

实现 MyHashMap 类：

MyHashMap() 用空映射初始化对象
void put(int key, int value) 向 HashMap 插入一个键值对 (key, value) 。如果 key 已经存在于映射中，则更新其对应的值 value 。
int get(int key) 返回特定的 key 所映射的 value ；如果映射中不包含 key 的映射，返回 -1 。
void remove(key) 如果映射中存在 key 的映射，则移除 key 和它所对应的 value 。


示例：

输入：
["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
输出：
[null, null, null, 1, -1, null, 1, null, -1]

解释：
MyHashMap myHashMap = new MyHashMap();
myHashMap.put(1, 1); // myHashMap 现在为 [[1,1]]
myHashMap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(1);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(3);    // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
myHashMap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
myHashMap.get(2);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
myHashMap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
myHashMap.get(2);    // 返回 -1（未找到），myHashMap 现在为 [[1,1]]


提示：

0 <= key, value <= 106
最多调用 104 次 put、get 和 remove 方法
 */
#[allow(dead_code)]
struct MyHashMap {
    val: Vec<i32>,
}

#[allow(dead_code)]
impl MyHashMap {
    fn new() -> Self {
        Self {
            val: vec![-1; 1000001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.val[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.val[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.val[key as usize] = -1;
    }
}