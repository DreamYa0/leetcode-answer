use std::collections::BTreeMap;

/// 225. 用队列实现栈
///
/// 请你仅使用两个队列实现一个后入先出（LIFO）的栈，并支持普通栈的全部四种操作（push、top、pop 和 empty）。
///
/// 实现 MyStack 类：
///
/// ```
/// void push(int x) 将元素 x 压入栈顶。
/// int pop() 移除并返回栈顶元素。
/// int top() 返回栈顶元素。
/// boolean empty() 如果栈是空的，返回 true ；否则，返回 false 。
/// ```
///
/// 注意：
///
/// 你只能使用队列的基本操作 —— 也就是 push to back、peek/pop from front、size 和 is empty 这些操作。
///
/// 你所使用的语言也许不支持队列。 你可以使用 list （列表）或者 deque（双端队列）来模拟一个队列 , 只要是标准的队列操作即可。
///
/// 示例：
/// ```
/// 输入：
/// ["MyStack", "push", "push", "top", "pop", "empty"]
/// [[], [1], [2], [], [], []]
/// 输出：
/// [null, null, null, 2, 2, false]
/// ```
///
/// ```
/// 解释：
/// MyStack myStack = new MyStack();
/// myStack.push(1);
/// myStack.push(2);
/// myStack.top(); // 返回 2
/// myStack.pop(); // 返回 2
/// myStack.empty(); // 返回 False
/// ```
///
/// 提示：
///
/// 1 <= x <= 9
///
/// 最多调用100 次 push、pop、top 和 empty
///
/// 每次调用 pop 和 top 都保证栈不为空
///
/// 进阶：你能否仅用一个队列来实现栈。
///
/// 思路
///
/// 一个队列在模拟栈弹出元素的时候只要将队列头部的元素（除了最后一个元素外） 重新添加到队列尾部，此时再去弹出元素就是栈的顺序了。
#[allow(dead_code)]
struct MyStack {
    pub queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyStack {
    fn new() -> Self {
        MyStack { queue: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x)
    }

    fn pop(&mut self) -> i32 {
        // 只需要弹出数组长度-1个元素,在重新放回数组中
        let len = self.queue.len() - 1;
        for _ in 0..len {
            // 弹出数组第一个元素,并放入数组最后
            let remove = self.queue.remove(0);
            self.queue.push(remove);
        }
        // 弹出数组第一个元素
        self.queue.remove(0)
    }

    fn top(&mut self) -> i32 {
        let pop = self.pop();
        self.queue.push(pop);
        pop
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// 面试题 03.01. 三合一
///
/// 三合一。描述如何只用一个数组来实现三个栈。
///
/// 你应该实现push(stackNum, value)、pop(stackNum)、isEmpty(stackNum)、peek(stackNum)方法。stackNum表示栈下标，value表示压入的值。
///
/// 构造函数会传入一个stackSize参数，代表每个栈的大小。
///
/// ```
/// 示例1:
///
///  输入：
/// ["TripleInOne", "push", "push", "pop", "pop", "pop", "isEmpty"]
/// [[1], [0, 1], [0, 2], [0], [0], [0], [0]]
///  输出：
/// [null, null, null, 1, -1, -1, true]
/// 说明：当栈为空时`pop, peek`返回-1，当栈满时`push`不压入元素。
/// 示例2:

///  输入：
/// ["TripleInOne", "push", "push", "push", "pop", "pop", "pop", "peek"]
/// [[2], [0, 1], [0, 2], [0, 3], [0], [0], [0], [0]]
///  输出：
/// [null, null, null, null, 2, 1, -1, -1]
/// ```
///
/// 提示：
///
/// 0 <= stackNum <= 2
#[allow(dead_code)]
struct TripleInOne {
    d: Vec<i32>,
    i: [usize; 3],
}

#[allow(dead_code)]
impl TripleInOne {
    fn new(stack_size: i32) -> Self {
        Self {
            d: vec![0; 3 * stack_size as usize],
            i: [0, 1, 2],
        }
    }

    fn push(&mut self, stack_num: i32, value: i32) {
        let n = stack_num as usize;
        if self.i[n] < self.d.len() {
            self.d[self.i[n]] = value;
            self.i[n] += 3;
        }
    }

    fn pop(&mut self, stack_num: i32) -> i32 {
        match stack_num as usize {
            n if self.i[n] >= 3 => {
                self.i[n] -= 3;
                self.d[self.i[n]]
            }
            _ => -1,
        }
    }

    fn peek(&self, stack_num: i32) -> i32 {
        if self.i[stack_num as usize] >= 3 {
            self.d[self.i[stack_num as usize] - 3]
        } else {
            -1
        }
    }

    fn is_empty(&self, stack_num: i32) -> bool {
        self.i[stack_num as usize] < 3
    }
}

/// 155. 栈的最小值
///
/// 请设计一个栈，除了常规栈支持的pop与push函数以外，还支持min函数，该函数返回栈元素中的最小值。执行push、pop和min操作的时间复杂度必须为O(1)。
///
/// ```
/// 示例：
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin();   --> 返回 -3.
/// minStack.pop();
/// minStack.top();      --> 返回 0.
/// minStack.getMin();   --> 返回 -2.
/// ```
#[allow(dead_code)]
struct MinStack {
    // 数据栈
    pub vec: Vec<i32>,
    // 存放各个阶段的最小值
    pub min: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            vec: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.vec.push(x);
        let len = self.min.len();
        if len > 0 {
            // 如果数组中有元素，数组中最后的元素和当前值取最小值
            self.min.push(x.min(self.min[len - 1]));
        } else {
            self.min.push(x);
        }
    }

    fn pop(&mut self) {
        self.vec.pop();
        self.min.pop();
    }

    fn top(&mut self) -> i32 {
        let len = self.vec.len();
        if len > 0 {
            // 取栈顶元素
            self.vec[len - 1]
        } else {
            -1
        }
    }

    fn get_min(&mut self) -> i32 {
        let len = self.min.len();
        if len > 0 {
            self.min[len - 1]
        } else {
            -1
        }
    }
}

/// 716. 最大栈
///
/// 设计一个最大栈数据结构，既支持栈操作，又支持查找栈中最大元素。
///
/// ```
/// 实现 MaxStack 类：
///
/// MaxStack() 初始化栈对象
/// void push(int x) 将元素 x 压入栈中。
/// int pop() 移除栈顶元素并返回这个元素。
/// int top() 返回栈顶元素，无需移除。
/// int peekMax() 检索并返回栈中最大元素，无需移除。
/// int popMax() 检索并返回栈中最大元素，并将其移除。如果有多个最大元素，只要移除 最靠近栈顶 的那个。
///
/// 示例：
///
/// 输入
/// ["MaxStack", "push", "push", "push", "top", "popMax", "top", "peekMax", "pop", "top"]
/// [[], [5], [1], [5], [], [], [], [], [], []]
/// 输出
/// [null, null, null, null, 5, 5, 1, 5, 1, 5]
///
/// 解释
/// MaxStack stk = new MaxStack();
/// stk.push(5);   // [5] - 5 既是栈顶元素，也是最大元素
/// stk.push(1);   // [5, 1] - 栈顶元素是 1，最大元素是 5
/// stk.push(5);   // [5, 1, 5] - 5 既是栈顶元素，也是最大元素
/// stk.top();     // 返回 5，[5, 1, 5] - 栈没有改变
/// stk.popMax();  // 返回 5，[5, 1] - 栈发生改变，栈顶元素不再是最大元素
/// stk.top();     // 返回 1，[5, 1] - 栈没有改变
/// stk.peekMax(); // 返回 5，[5, 1] - 栈没有改变
/// stk.pop();     // 返回 1，[5] - 此操作后，5 既是栈顶元素，也是最大元素
/// stk.top();     // 返回 5，[5] - 栈没有改变
///
/// ```
///
/// 提示：
///
/// -107 <= x <= 107
///
/// 最多调用 104 次 push、pop、top、peekMax 和 popMax
///
/// 调用 pop、top、peekMax 或 popMax 时，栈中 至少存在一个元素
///
/// 进阶：
///
/// 试着设计解决方案：调用 top 方法的时间复杂度为 O(1) ，调用其他方法的时间复杂度为 O(logn) 。
#[allow(dead_code)]
struct MaxStack {
    stack: BTreeMap<usize, i32>,
    max: BTreeMap<(i32, usize), usize>,
}

#[allow(dead_code)]
impl MaxStack {
    fn new() -> Self {
        MaxStack {
            stack: BTreeMap::new(),
            max: BTreeMap::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let last = self.stack.iter().next_back();
        let new_index = if let Some((&i, _)) = last { i + 1 } else { 0 };
        self.stack.insert(new_index, x);
        self.max.insert((x, new_index), new_index);
    }

    fn pop(&mut self) -> i32 {
        let (&i, &v) = self.stack.iter().next_back().unwrap();
        self.stack.remove(&i);
        self.max.remove(&(v, i));
        v
    }

    fn top(&self) -> i32 {
        let (_, &v) = self.stack.iter().next_back().unwrap();
        v
    }

    fn peek_max(&self) -> i32 {
        let (&(v, _), _) = self.max.iter().next_back().unwrap();
        v
    }

    fn pop_max(&mut self) -> i32 {
        let (&(v, _), &i) = self.max.iter().next_back().unwrap();
        self.stack.remove(&i);
        self.max.remove(&(v, i));
        v
    }
}

/// LCR 125. 图书整理 II
///
/// 读者来到图书馆排队借还书，图书管理员使用两个书车来完成整理借还书的任务。书车中的书从下往上叠加存放，图书管理员每次只能拿取书车顶部的书。排队的读者会有两种操作：
///
/// push(bookID)：把借阅的书籍还到图书馆。
///
/// pop()：从图书馆中借出书籍。
///
/// 为了保持图书的顺序，图书管理员每次取出供读者借阅的书籍是 最早 归还到图书馆的书籍。你需要返回 每次读者借出书的值 。
///
/// 如果没有归还的书可以取出，返回 -1 。
///
/// ```
/// 示例 1：
///
/// 输入：
/// ["BookQueue", "push", "push", "pop"]
/// [[], [1], [2], []]
/// 输出：[null,null,null,1]
/// 解释：
/// MyQueue myQueue = new MyQueue();
/// myQueue.push(1); // queue is: [1]
/// myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
/// myQueue.pop(); // return 1, queue is [2]
///
/// 提示：
///
/// 1 <= bookID <= 10000
/// 最多会对 push、pop 进行 10000 次调用
/// ```
#[allow(dead_code)]
struct CQueue {
    // 入栈
    en: Vec<i32>,
    // 出栈
    out: Vec<i32>,
}

#[allow(dead_code)]
impl CQueue {
    fn new() -> Self {
        Self {
            en: Vec::new(),
            out: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.en.push(value)
    }

    fn delete_head(&mut self) -> i32 {
        if !self.out.is_empty() {
            // 如果出栈不为空的时候直接从out弹出数据
            return self.out.pop().unwrap();
        }
        if self.en.is_empty() {
            return -1;
        }
        if !self.en.is_empty() {
            // 如果出栈为空，en的栈底元素是需要删除的，所以需要把en中 len - 1 的数据压入out
            let mut len = self.en.len() - 1;
            while len > 0 {
                self.out.push(self.en.pop().unwrap());
                len -= 1;
            }
            // 当en中len-1的数据都放入到out之后，en中最后一个元素就是需要被删除的
            return self.en.pop().unwrap();
        }
        -1
    }
}
