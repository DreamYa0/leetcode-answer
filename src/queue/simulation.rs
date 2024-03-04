/// 232. 用栈实现队列
///
/// 请你仅使用两个栈实现先入先出队列。队列应当支持一般队列支持的所有操作（push、pop、peek、empty）：
///
/// 实现 MyQueue 类：
/// ```
/// void push(int x) 将元素 x 推到队列的末尾
/// int pop() 从队列的开头移除并返回元素
/// int peek() 返回队列开头的元素
/// boolean empty() 如果队列为空，返回 true ；否则，返回 false
/// ```
/// 说明：
///
/// 你 只能 使用标准的栈操作 —— 也就是只有 push to top, peek/pop from top, size, 和 is empty 操作是合法的。
///
/// 你所使用的语言也许不支持栈。你可以使用 list 或者 deque（双端队列）来模拟一个栈，只要是标准的栈操作即可。
///
/// 示例 1：
///
/// 输入：
///
/// ["MyQueue", "push", "push", "peek", "pop", "empty"]
///
/// [[], [1], [2], [], [], []]
///
/// 输出：
///
/// [null, null, null, 1, 1, false]
///
/// ```
/// 解释：
/// MyQueue myQueue = new MyQueue();
/// myQueue.push(1); // queue is: [1]
/// myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
/// myQueue.peek(); // return 1
/// myQueue.pop(); // return 1, queue is [2]
/// myQueue.empty(); // return false
/// ```
///
/// 提示：
///
/// 1 <= x <= 9
///
/// 最多调用 100 次 push、pop、peek 和 empty
///
/// 假设所有操作都是有效的 （例如，一个空的队列不会调用 pop 或者 peek 操作）
///
/// 进阶：
///
/// 你能否实现每个操作均摊时间复杂度为 O(1) 的队列？换句话说，执行 n 个操作的总时间复杂度为 O(n) ，即使其中一个操作可能花费较长时间。
///
/// 思路
///
///这是一道模拟题，不涉及到具体算法，考察的就是对栈和队列的掌握程度。
///
///使用栈来模式队列的行为，如果仅仅用一个栈，是一定不行的，所以需要两个栈一个输入栈，一个输出栈，这里要注意输入栈和输出栈的关系。
///
/// 下面动画模拟以下队列的执行过程：
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/232.%E7%94%A8%E6%A0%88%E5%AE%9E%E7%8E%B0%E9%98%9F%E5%88%97%E7%89%88%E6%9C%AC2.gif" />
///
/// ```
/// 执行语句：
/// queue.push(1);
/// queue.push(2);
/// queue.pop(); 注意此时的输出栈的操作
/// queue.push(3);
/// queue.push(4);
/// queue.pop();
/// queue.pop();注意此时的输出栈的操作
/// queue.pop();
/// queue.empty();
/// ```
///
/// 在push数据的时候，只要数据放进输入栈就好，但在pop的时候，操作就复杂一些，输出栈如果为空，就把进栈数据全部导入进来（注意是全部导入），再从出栈弹出数据，如果输出栈不为空，则直接从出栈弹出数据就可以了。
///
/// 最后如何判断队列为空呢？如果进栈和出栈都为空的话，说明模拟的队列为空了。
///
/// 在代码实现的时候，会发现pop() 和 peek()两个函数功能类似，代码实现上也是类似的，可以思考一下如何把代码抽象一下。
#[allow(dead_code)]
struct MyQueue {
    // 入栈
    pub in_stack: Vec<i32>,
    // 出栈
    pub out_stack: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        // 入栈
        self.in_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        // 出栈
        if self.out_stack.is_empty() {
            // 如果输出栈为空，就把输入栈的数据全部导入进来
            while !self.in_stack.is_empty() {
                self.out_stack.push(self.in_stack.pop().unwrap());
            }
        }
        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        let pop = self.pop();
        // 把弹出的元素再放回去
        self.out_stack.push(pop);
        pop
    }

    fn empty(&mut self) -> bool {
        // 如果进栈和出栈都为空的话，说明模拟的队列为空了
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

/// 622. 设计循环队列
///
/// 设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于 FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。它也被称为“环形缓冲器”。
///
/// 循环队列的一个好处是我们可以利用这个队列之前用过的空间。在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。但是使用循环队列，我们能使用这些空间去存储新的值。
///
/// 你的实现应该支持如下操作：
///
/// ```
/// MyCircularQueue(k): 构造器，设置队列长度为 k 。
/// Front: 从队首获取元素。如果队列为空，返回 -1 。
/// Rear: 获取队尾元素。如果队列为空，返回 -1 。
/// enQueue(value): 向循环队列插入一个元素。如果成功插入则返回真。
/// deQueue(): 从循环队列中删除一个元素。如果成功删除则返回真。
/// isEmpty(): 检查循环队列是否为空。
/// isFull(): 检查循环队列是否已满。
///
/// 示例：
///
/// MyCircularQueue circularQueue = new MyCircularQueue(3); // 设置长度为 3
/// circularQueue.enQueue(1);  // 返回 true
/// circularQueue.enQueue(2);  // 返回 true
/// circularQueue.enQueue(3);  // 返回 true
/// circularQueue.enQueue(4);  // 返回 false，队列已满
/// circularQueue.Rear();  // 返回 3
/// circularQueue.isFull();  // 返回 true
/// circularQueue.deQueue();  // 返回 true
/// circularQueue.enQueue(4);  // 返回 true
/// circularQueue.Rear();  // 返回 4
///
/// 提示：
///
/// 所有的值都在 0 至 1000 的范围内；
/// 操作数将在 1 至 1000 的范围内；
/// 请不要使用内置的队列库。
/// ```
#[allow(dead_code)]
struct MyCircularQueue {
    // 定义数组用来存放数据
    data: Vec<i32>,
    // 队尾
    rear: i32,
    // 队首
    front: i32,
    // 队列大小
    size: i32,
}

#[allow(dead_code, unused_must_use)]
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            data: vec![0; k as usize],
            rear: -1,
            front: -1,
            size: k,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            // 如果队列满了就返回false
            return false;
        }
        if self.is_empty() {
            // 如果队列是空的那么就把元素填充到0的位置
            self.front = 0;
        }
        self.rear = (self.rear + 1) % self.size;
        self.data[self.rear as usize] = value;
        return true;
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.front == self.rear {
            // front 和 near 相等说明队列中只有一个元素
            self.front = -1;
            self.rear = -1;
            return true;
        }
        self.front = (self.front + 1) % self.size;
        return true;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front as usize]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.rear as usize]
    }

    fn is_empty(&self) -> bool {
        self.front == -1
    }

    fn is_full(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        // 判断队列是否已满
        (self.rear + 1) % self.size == self.front
    }

    /// 扩容
    fn increase(&mut self) {
        // 新的容量大小
        let new_size = self.size * 2;
        // 创建新的数组
        let mut new_data = vec![0; new_size as usize];
        if self.is_empty() {
            // 如果队列是空的
            self.size = new_size;
            self.data = new_data;
            return;
        }
        let mut new_rear = -1;
        // 如果队列不为空就循环转移数据
        while !self.is_empty() {
            // 获取老的队首数据，放入新队列的队尾
            new_data[new_rear as usize + 1] = self.front();
            // 弹出老队列的队首数据
            self.de_queue();
            // 移动新队列队尾下标
            new_rear += 1;
        }
        self.front = 0;
        self.rear = new_rear;
        self.data = new_data;
        self.size = new_size;
    }

    /// 缩容
    fn reduce(&mut self) {}
}

/**
641. 设计循环双端队列

设计实现双端队列。

```
实现 MyCircularDeque 类:

MyCircularDeque(int k) ：构造函数,双端队列最大为 k 。
boolean insertFront()：将一个元素添加到双端队列头部。 如果操作成功返回 true ，否则返回 false 。
boolean insertLast() ：将一个元素添加到双端队列尾部。如果操作成功返回 true ，否则返回 false 。
boolean deleteFront() ：从双端队列头部删除一个元素。 如果操作成功返回 true ，否则返回 false 。
boolean deleteLast() ：从双端队列尾部删除一个元素。如果操作成功返回 true ，否则返回 false 。
int getFront() )：从双端队列头部获得一个元素。如果双端队列为空，返回 -1 。
int getRear() ：获得双端队列的最后一个元素。 如果双端队列为空，返回 -1 。
boolean isEmpty() ：若双端队列为空，则返回 true ，否则返回 false  。
boolean isFull() ：若双端队列满了，则返回 true ，否则返回 false 。


示例 1：

输入
["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
输出
[null, true, true, true, false, 2, true, true, true, 4]

解释
MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
circularDeque.insertLast(1);			        // 返回 true
circularDeque.insertLast(2);			        // 返回 true
circularDeque.insertFront(3);			        // 返回 true
circularDeque.insertFront(4);			        // 已经满了，返回 false
circularDeque.getRear();  				// 返回 2
circularDeque.isFull();				        // 返回 true
circularDeque.deleteLast();			        // 返回 true
circularDeque.insertFront(4);			        // 返回 true
circularDeque.getFront();				// 返回 4



提示：

1 <= k <= 1000
0 <= value <= 1000
insertFront, insertLast, deleteFront, deleteLast, getFront, getRear, isEmpty, isFull  调用次数不大于 2000 次
```
 */
#[allow(dead_code)]
struct MyCircularDeque {
    // 定义数组用来存放数据
    data: Vec<i32>,
    // 队尾
    rear: i32,
    // 队首
    front: i32,
    // 队列大小
    size: i32,
    // 队列长度
    len: i32,
}

#[allow(dead_code)]
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            data: vec![0; k as usize],
            rear: 1,
            front: 0,
            size: 0,
            len: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            // 如果队列满了就返回false
            return false;
        }
        self.front = (self.front + 1) % self.len;
        self.data[self.front as usize] = value;
        self.size += 1;
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            // 如果队列满了就返回false
            return false;
        }
        self.rear = (self.rear - 1 + self.len) % self.len;
        self.data[self.rear as usize] = value;
        self.size += 1;
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front - 1 + self.len) % self.len;
        self.size -= 1;
        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + 1) % self.len;
        self.size -= 1;
        return true;
    }

    fn get_front(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front as usize]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.rear as usize]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        // 判断队列是否已满
        self.size == self.len
    }
}

/// 933. 最近的请求次数
///
/// 写一个 RecentCounter 类来计算特定时间范围内最近的请求。
///
/// 请你实现 RecentCounter 类：
///
/// RecentCounter() 初始化计数器，请求数为 0 。
///
/// int ping(int t) 在时间 t 添加一个新请求，其中 t 表示以毫秒为单位的某个时间，并返回过去 3000 毫秒内发生的所有请求数（包括新请求）。确切地说，返回在 [t-3000, t] 内发生的请求数。
///
/// 保证 每次对 ping 的调用都使用比之前更大的 t 值。
///
/// ```
/// 示例 1：
///
/// 输入：
/// ["RecentCounter", "ping", "ping", "ping", "ping"]
/// [[], [1], [100], [3001], [3002]]
/// 输出：
/// [null, 1, 2, 3, 3]
///
/// 解释：
/// RecentCounter recentCounter = new RecentCounter();
/// recentCounter.ping(1);     // requests = [1]，范围是 [-2999,1]，返回 1
/// recentCounter.ping(100);   // requests = [1, 100]，范围是 [-2900,100]，返回 2
/// recentCounter.ping(3001);  // requests = [1, 100, 3001]，范围是 [1,3001]，返回 3
/// recentCounter.ping(3002);  // requests = [1, 100, 3001, 3002]，范围是 [2,3002]，返回 3
///
/// 提示：
///
/// 1 <= t <= 109
/// 保证每次对 ping 调用所使用的 t 值都 严格递增
/// 至多调用 ping 方法 104 次
/// ```
#[allow(dead_code)]
struct RecentCounter {
    // 记录时间戳
    timer: Vec<i32>,
}

#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        RecentCounter { timer: Vec::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        // 入队列
        self.timer.push(t);
        // [t-3000,t]的数据出队列,如果队首和t的间距大于3000就需要将队首弹出
        while t - self.timer.first().unwrap() > 3000 {
            // 移除头部数据
            self.timer.remove(0);
        }
        self.timer.len() as i32
    }
}

/**
面试题 03.06. 动物收容所

动物收容所。有家动物收容所只收容狗与猫，且严格遵守“先进先出”的原则。在收养该收容所的动物时，收养人只能收养所有动物中“最老”（由其进入收容所的时间长短而定）的动物，或者可以挑选猫或狗（同时必须收养此类动物中“最老”的）。换言之，收养人不能自由挑选想收养的对象。请创建适用于这个系统的数据结构，实现各种操作方法，比如enqueue、dequeueAny、dequeueDog和dequeueCat。允许使用Java内置的LinkedList数据结构。

enqueue方法有一个animal参数，animal[0]代表动物编号，animal[1]代表动物种类，其中 0 代表猫，1 代表狗。

dequeue*方法返回一个列表[动物编号, 动物种类]，若没有可以收养的动物，则返回[-1,-1]。

```
示例1:

 输入：
["AnimalShelf", "enqueue", "enqueue", "dequeueCat", "dequeueDog", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [], [], []]
 输出：
[null,null,null,[0,0],[-1,-1],[1,0]]
示例2:

 输入：
["AnimalShelf", "enqueue", "enqueue", "enqueue", "dequeueDog", "dequeueCat", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [[2, 1]], [], [], []]
 输出：
[null,null,null,null,[2,1],[0,0],[1,0]]
说明:

收纳所的最大容量为20000
```
 */
#[allow(dead_code)]
struct AnimalShelf {
    // 1 代表狗 数组中存储的是动物编号
    dog: Vec<i32>,
    // 其中 0 代表猫 数组中存储的是动物编号
    cat: Vec<i32>,
}

#[allow(dead_code)]
impl AnimalShelf {
    fn new() -> Self {
        Self {
            dog: Vec::new(),
            cat: Vec::new(),
        }
    }

    fn enqueue(&mut self, animal: Vec<i32>) {
        if animal[1] == 0 {
            // 猫
            self.cat.push(animal[0]);
        } else {
            // 狗
            self.dog.push(animal[0]);
        }
    }

    fn dequeue_any(&mut self) -> Vec<i32> {
        if self.dog.is_empty() && self.cat.is_empty() {
            return [-1, -1].to_vec();
        }
        if self.cat.first().or(Some(&999999999)) < self.dog.first().or(Some(&999999999)) {
            return vec![self.cat.remove(0), 0];
        } else {
            return vec![self.dog.remove(0), 1];
        }
    }

    fn dequeue_dog(&mut self) -> Vec<i32> {
        if self.dog.is_empty() {
            return [-1, -1].to_vec();
        }
        vec![self.dog.remove(0), 1]
    }

    fn dequeue_cat(&mut self) -> Vec<i32> {
        if self.cat.is_empty() {
            return [-1, -1].to_vec();
        }
        vec![self.cat.remove(0), 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_circular_queue() {
        let mut obj = MyCircularQueue::new(3);
        let ret_1 = obj.en_queue(1);
        let ret_2 = obj.en_queue(2);
        let ret_3 = obj.en_queue(3);
        let ret_4 = obj.en_queue(4);
        let ret_5 = obj.rear();
        let ret_6 = obj.is_full();
        let ret_7 = obj.de_queue();
        let ret_8 = obj.en_queue(4);
        let ret_9 = obj.rear();
        println!(
            "{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            ret_1, ret_2, ret_3, ret_4, ret_5, ret_6, ret_7, ret_8, ret_9
        );
    }
}
