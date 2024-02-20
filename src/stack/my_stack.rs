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

/// 20. 有效的括号
///
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
///
/// 有效字符串需满足：
///
/// 左括号必须用相同类型的右括号闭合。
///
/// 左括号必须以正确的顺序闭合。
///
/// 每个右括号都有一个对应的相同类型的左括号。
///
/// ```
/// 示例 1：
///
/// 输入：s = "()"
/// 输出：true
///
/// 示例 2：
///
/// 输入：s = "()[]{}"
/// 输出：true
/// 示例 3：
///
/// 输入：s = "(]"
/// 输出：false
///
/// 提示：
///
/// 1 <= s.length <= 104
///
/// s 仅由括号 '()[]{}' 组成
/// ```
///
/// 由于栈结构的特殊性，非常适合做对称匹配类的题目。
///
/// 首先要弄清楚，字符串里的括号不匹配有几种情况。
///
/// 一些同学，在面试中看到这种题目上来就开始写代码，然后就越写越乱。
///
/// 建议在写代码之前要分析好有哪几种不匹配的情况，如果不在动手之前分析好，写出的代码也会有很多问题。
///
/// 先来分析一下 这里有三种不匹配的情况，
///
/// 第一种情况，字符串里左方向的括号多余了 ，所以不匹配。 括号匹配1
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/2020080915505387.png" alt="括号匹配1" style="zoom:50%;" />
///
/// 第二种情况，括号没有多余，但是 括号的类型没有匹配上。 括号匹配2
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20200809155107397.png" alt="括号匹配1" style="zoom:50%;" />
///
/// 第三种情况，字符串里右方向的括号多余了，所以不匹配。 括号匹配3
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20200809155115779.png" alt="括号匹配1" style="zoom:50%;" />
///
/// 我们的代码只要覆盖了这三种不匹配的情况，就不会出问题，可以看出 动手之前分析好题目的重要性。
///
/// 动画如下：
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/20.%E6%9C%89%E6%95%88%E6%8B%AC%E5%8F%B7.gif" alt="括号匹配1" style="zoom:50%;" />
///
/// 20.有效括号
///
/// 第一种情况：已经遍历完了字符串，但是栈不为空，说明有相应的左括号没有右括号来匹配，所以return false
///
/// 第二种情况：遍历字符串匹配的过程中，发现栈里没有要匹配的字符。所以return false
///
/// 第三种情况：遍历字符串匹配的过程中，栈已经为空了，没有匹配的字符了，说明右括号没有找到对应的左括号return false
///
/// 那么什么时候说明左括号和右括号全都匹配了呢，就是字符串遍历完之后，栈是空的，就说明全都匹配了。
///
/// 分析完之后，代码其实就比较好写了
///
/// 但还有一些技巧，在匹配左括号的时候，右括号先入栈，就只需要比较当前元素和栈顶相不相等就可以了，比左括号先入栈代码实现要简单的多了！
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        // 如果字符串长度是奇数，肯定不匹配
        return false;
    }
    // 定义一个栈
    let mut stack = Vec::new();
    // 将字符串转换成字符数组
    let mut chars = s.chars().collect::<Vec<char>>();
    // 遍历字符数组
    while let Some(c) = chars.pop() {
        // 匹配左括号的时候，右括号先入栈
        match c {
            ')' => stack.push('('),
            ']' => stack.push('['),
            '}' => stack.push('{'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }
    // 如果栈为空，说明全部匹配
    stack.is_empty()
}

/// 1047. 删除字符串中的所有相邻重复项
///
/// 给出由小写字母组成的字符串 S，重复项删除操作会选择两个相邻且相同的字母，并删除它们。
///
/// 在 S 上反复执行重复项删除操作，直到无法继续删除。
///
/// 在完成所有重复项删除操作后返回最终的字符串。答案保证唯一。
///
/// ```
/// 示例：
/// 输入："abbaca"
/// 输出："ca"
/// 解释：
/// 例如，在 "abbaca" 中，我们可以删除 "bb" 由于两字母相邻且相同，这是此时唯一可以执行删除操作的重复项。
/// 之后我们得到字符串 "aaca"，其中又只有 "aa" 可以执行重复项删除操作，所以最后的字符串为 "ca"。
/// 提示：
/// 1 <= S.length <= 20000
/// S 仅由小写英文字母组成。
/// ```
///
/// 思路
///
/// 本题要删除相邻相同元素，相对于20. 有效的括号 (opens new window)来说其实也是匹配问题，
/// 20. 有效的括号 是匹配左右括号，本题是匹配相邻元素，最后都是做消除的操作。
///
/// 本题也是用栈来解决的经典题目。
///
/// 那么栈里应该放的是什么元素呢？
///
/// 我们在删除相邻重复项的时候，其实就是要知道当前遍历的这个元素，我们在前一位是不是遍历过一样数值的元素，那么如何记录前面遍历过的元素呢？
///
/// 所以就是用栈来存放，那么栈的目的，就是存放遍历过的元素，当遍历当前的这个元素的时候，去栈里看一下我们是不是遍历过相同数值的相邻元素。
///
/// 然后再去做对应的消除操作。 如动画所示：
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/1047.%E5%88%A0%E9%99%A4%E5%AD%97%E7%AC%A6%E4%B8%B2%E4%B8%AD%E7%9A%84%E6%89%80%E6%9C%89%E7%9B%B8%E9%82%BB%E9%87%8D%E5%A4%8D%E9%A1%B9.gif" alt="括号匹配1" style="zoom:50%;" />
///
/// 1047.删除字符串中的所有相邻重复项
///
/// 从栈中弹出剩余元素，此时是字符串ac，因为从栈里弹出的元素是倒序的，所以再对字符串进行反转一下，就得到了最终的结果。
pub fn remove_duplicates(s: String) -> String {
    // 定义一个栈,用来存放遍历过的元素
    let mut stack = Vec::new();
    for c in s.chars() {
        // 遍历当前元素和栈底元素是否相等
        if let Some(&top) = stack.last() {
            if top == c {
                // 如果相等，就弹出栈底元素
                stack.pop();
            } else {
                // 如果不相等，就入栈
                stack.push(c);
            }
        } else {
            // 如果栈为空，就入栈
            stack.push(c);
        }
    }
    // 从栈中弹出剩余元素
    stack.iter().collect()
}

/// 150. 逆波兰表达式求值
///
/// 给你一个字符串数组 tokens ，表示一个根据 逆波兰表示法 表示的算术表达式。
///
/// 请你计算该表达式。返回一个表示表达式值的整数。
///
/// 注意：
///
/// 有效的算符为 '+'、'-'、'*' 和 '/' 。
/// 每个操作数（运算对象）都可以是一个整数或者另一个表达式。
/// 两个整数之间的除法总是 向零截断 。
/// 表达式中不含除零运算。
/// 输入是一个根据逆波兰表示法表示的算术表达式。
/// 答案及所有中间计算结果可以用 32 位 整数表示。
///
/// ```
/// 示例 1：
///
/// 输入：tokens = ["2","1","+","3","*"]
/// 输出：9
/// 解释：该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9
/// 示例 2：
///
/// 输入：tokens = ["4","13","5","/","+"]
/// 输出：6
/// 解释：该算式转化为常见的中缀算术表达式为：(4 + (13 / 5)) = 6
/// 示例 3：
///
/// 输入：tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
/// 输出：22
/// 解释：该算式转化为常见的中缀算术表达式为：
///   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
/// = ((10 * (6 / (12 * -11))) + 17) + 5
/// = ((10 * (6 / -132)) + 17) + 5
/// = ((10 * 0) + 17) + 5
/// = (0 + 17) + 5
/// = 17 + 5
/// = 22
///
/// ```
///
/// 提示：
///
/// 1 <= tokens.length <= 104
/// tokens[i] 是一个算符（"+"、"-"、"*" 或 "/"），或是在范围 [-200, 200] 内的一个整数
///
/// 逆波兰表达式：
///
/// 逆波兰表达式是一种后缀表达式，所谓后缀就是指算符写在后面。
///
/// 平常使用的算式则是一种中缀表达式，如 ( 1 + 2 ) * ( 3 + 4 ) 。
///
/// 该算式的逆波兰表达式写法为 ( ( 1 2 + ) ( 3 4 + ) * ) 。
///
/// 逆波兰表达式主要有以下两个优点：
///
/// 去掉括号后表达式无歧义，上式即便写成 1 2 + 3 4 + * 也可以依据次序计算出正确结果。
///
/// 适合用栈操作运算：遇到数字则入栈；遇到算符则取出栈顶两个数字进行计算，并将结果压入栈中。
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/150.%E9%80%86%E6%B3%A2%E5%85%B0%E8%A1%A8%E8%BE%BE%E5%BC%8F%E6%B1%82%E5%80%BC.gif" />
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    // 定义一个栈
    let mut stack = Vec::new();
    for v in tokens {
        match v.as_str() {
            "+" => {
                // 弹出栈顶两个元素，相加后再入栈
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b + a);
            }
            "-" => {
                // 弹出栈顶两个元素，相减后再入栈
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            "*" => {
                // 弹出栈顶两个元素，相乘后再入栈
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b * a);
            }
            "/" => {
                // 弹出栈顶两个元素，相除后再入栈
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            _ => {
                // 如果是数字，就入栈
                stack.push(v.parse::<i32>().unwrap());
            }
        }
    }
    // 栈顶元素就是最终结果
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s = "()".to_string();
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn test_remove_duplicates() {
        let s = "abbaca".to_string();
        assert_eq!(remove_duplicates(s), "ca".to_string());
    }

    #[test]
    fn test_eval_rpn() {
        let tokens = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
        assert_eq!(eval_rpn(tokens), 9);
    }
}
