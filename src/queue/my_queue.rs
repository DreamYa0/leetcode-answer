use std::collections::VecDeque;

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

/// 239.滑动窗口最大值
///
/// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
///
/// 返回 滑动窗口中的最大值 。
///
/// 示例 1：
///
/// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
///
/// 输出：[3,3,5,5,6,7]
///
/// 解释：
///
/// ```
/// 滑动窗口的位置                最大值
/// ---------------               -----
/// [1  3  -1] -3  5  3  6  7       3
///  1 [3  -1  -3] 5  3  6  7       3
///  1  3 [-1  -3  5] 3  6  7       5
///  1  3  -1 [-3  5  3] 6  7       5
///  1  3  -1  -3 [5  3  6] 7       6
///  1  3  -1  -3  5 [3  6  7]      7
/// ```
/// 示例 2：
///
/// 输入：nums = [1], k = 1
///
/// 输出：[1]
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -104 <= nums[i] <= 104
///
/// 1 <= k <= nums.length
///
/// 单调队列套路
///
/// 入（元素进入队尾，同时维护队列单调性）
///
/// 出（元素离开队首）
///
/// 记录/维护答案（根据队首）
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut ans = Vec::new();
    // 双端队列
    let mut q = VecDeque::new();
    for (i, &x) in nums.iter().enumerate() {
        // 1. 入
        // 如果队尾元素小于等于当前元素,则弹出队尾元素
        while !q.is_empty() && nums[*q.back().unwrap()] <= x {
            // 维护 q 的单调性
            q.pop_back();
        }
        // 入队
        q.push_back(i);
        // 2. 出，如果队列超出窗口大小,则弹出队首元素
        if i - q[0] >= k {
            // 队首已经离开窗口了
            q.pop_front();
        }
        // 3. 记录答案
        if i >= k - 1 {
            // 由于队首到队尾单调递减，所以窗口最大值就是队首
            ans.push(nums[q[0]]);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let max_sliding_window = max_sliding_window(nums, k);
        println!("{:?}", max_sliding_window)
    }
}