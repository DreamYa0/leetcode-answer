pub mod monotonic_stack;
pub mod design_stack;

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

/// 1441. 用栈操作构建数组
///
/// 给你一个数组 target 和一个整数 n。每次迭代，需要从  list = { 1 , 2 , 3 ..., n } 中依次读取一个数字。
///
/// 请使用下述操作来构建目标数组 target ：
///
/// "Push"：从 list 中读取一个新元素， 并将其推入数组中。
///
/// "Pop"：删除数组中的最后一个元素。
///
/// 如果目标数组构建完成，就停止读取更多元素。
///
/// 题目数据保证目标数组严格递增，并且只包含 1 到 n 之间的数字。
///
/// 请返回构建目标数组所用的操作序列。如果存在多个可行方案，返回任一即可。
///
/// ```
/// 示例 1：
///
/// 输入：target = [1,3], n = 3
/// 输出：["Push","Push","Pop","Push"]
/// 解释：
/// 读取 1 并自动推入数组 -> [1]
/// 读取 2 并自动推入数组，然后删除它 -> [1]
/// 读取 3 并自动推入数组 -> [1,3]
/// 示例 2：
/// 输入：target = [1,2,3], n = 3
/// 输出：["Push","Push","Push"]
/// 示例 3：
/// 输入：target = [1,2], n = 4
/// 输出：["Push","Push"]
/// 解释：只需要读取前 2 个数字就可以停止。
/// ```
///
/// 提示：
///
/// 1 <= target.length <= 100
///
/// 1 <= n <= 100
///
/// 1 <= target[i] <= n
///
/// target 严格递增
pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    // 计数器
    let mut count = 1;
    // 当前坐标
    let mut index = 0;
    // 定义数组
    let mut res = Vec::new();
    while index < target.len() && count <= n {
        // 获取当前数组元素
        let v = target[index];
        // 如果当前数组元素等于计数器
        if v == count {
            // 入栈
            res.push("Push".to_string());
            // 计数器+1
            count += 1;
            // 下标+1
            index += 1;
        } else {
            // 如果当前数组元素不等于计数器
            // 入栈
            res.push("Push".to_string());
            // 出栈
            res.push("Pop".to_string());
            // 计数器+1
            count += 1;
        }
    }
    res
}

/// 316. 去除重复字母
///
/// 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。
///
/// ```
/// 示例 1：
/// 输入：s = "bcabc"
/// 输出："abc"
/// 示例 2：
/// 输入：s = "cbacdcbc"
/// 输出："acdb"
/// ```
/// 提示：
///
/// 1 <= s.length <= 104
///
/// s 由小写英文字母组成
///
/// 具体算法如下：
///
/// 统计每个字母的出现次数，记到一个哈希表或者数组 left 中。
///
/// 遍历 sss，先把 left[s[i]] 减一。
///
/// 如果 s[i] 在 ans 中，直接 continue。为了快速判断 s[i] 是否在 ans 中，可以创建一个哈希表或者布尔数组 inAns。
///
/// 如果 s[i] 不在 ans 中，那么判断 s[i] 是否小于 ans 的最后一个字母（记作 xxx），
/// 如果 s[i]<x 且 left[x]>0，那么可以把 x 从 ans 中去掉，同时标记 inAns[x]=false。
///
/// 反复执行第 4 步，直到 ans 为空，或者 s[i]>x，或者 left[x]=0。
///
/// 把 s[i] 加到 ans 末尾，同时标记 inAns[s[i]]=true。然后继续遍历 s 的下一个字母。
///
/// 遍历完 s 后，返回 ans。
///
/// https://leetcode.cn/problems/remove-duplicate-letters/solutions/2381483/gen-zhao-wo-guo-yi-bian-shi-li-2ni-jiu-m-zd6u/
pub fn remove_duplicate_letters(s: String) -> String {
    // 定义一个栈
    let mut stack = Vec::new();
    // 存放字符是否在栈中
    let mut in_stack = [false; 26];
    // 统计字符出现的次数
    let mut cnt = [0; 26];
    s.chars().for_each(|c| {
        cnt[c as usize - 'a' as usize] += 1;
    });

    for c in s.chars() {
        // 获取字符的下标
        let index = c as usize - 'a' as usize;
        // 如果字符不在栈中
        if !in_stack[index] {
            // 如果栈不为空，且栈顶元素大于当前元素，且栈顶元素的次数大于0,就出栈
            while let Some(&top) = stack.last() {
                // 栈顶元素比当前元素大且后续中还有栈顶元素
                if top > c && cnt[top as usize - 'a' as usize] > 0 {
                    // 标记栈中已不存在栈顶元素
                    in_stack[top as usize - 'a' as usize] = false;
                    // 弹出栈顶元素
                    stack.pop();
                } else {
                    break;
                }
            }
            // 入栈
            in_stack[index] = true;
            stack.push(c);
        }
        // 次数-1
        cnt[index] -= 1;
    }
    stack.iter().collect()
}

/// 1209. 删除字符串中的所有相邻重复项 II
///
/// 给你一个字符串 s，「k 倍重复项删除操作」将会从 s 中选择 k 个相邻且相等的字母，并删除它们，使被删去的字符串的左侧和右侧连在一起。
///
/// 你需要对 s 重复进行无限次这样的删除操作，直到无法继续为止。
///
/// 在执行完所有删除操作后，返回最终得到的字符串。
///
/// 本题答案保证唯一。
///
/// ```
/// 示例 1：
/// 输入：s = "abcd", k = 2
/// 输出："abcd"
/// 解释：没有要删除的内容。
///
/// 示例 2：
/// 输入：s = "deeedbbcccbdaa", k = 3
/// 输出："aa"
/// 解释：
/// 先删除 "eee" 和 "ccc"，得到 "ddbbbdaa"
/// 再删除 "bbb"，得到 "dddaa"
/// 最后删除 "ddd"，得到 "aa"
///
/// 示例 3：
/// 输入：s = "pbbcggttciiippooaais", k = 2
/// 输出："ps"
/// ```

/// 提示：
///
/// 1 <= s.length <= 10^5
///
/// 2 <= k <= 10^4
///
/// s 中只含有小写英文字母。
pub fn remove_duplicates_ii(s: String, k: i32) -> String {
    // 定义一个栈，统计字符出现的次数
    let mut stack: Vec<(u8, i32)> = vec![];
    for ch in s.bytes() {
        match stack.last_mut() {
            // 如果栈中最后一个元素和当前元素相等，就只计数不入栈
            Some(last) if ch == last.0 => {
                // 计数+1
                last.1 += 1;
                // 当计数等于k时，出栈
                if last.1 == k {
                    // 这个过程相同的字符只入栈了一个，所以这里只需要弹出一个元素即可
                    stack.pop();
                }
            }
            _ => stack.push((ch, 1)),
        }
    }

    String::from_utf8(
        stack
            .into_iter()
            // 元组转数组
            .map(|(ch, cnt)| vec![ch; cnt as usize])
            // 扁平化嵌套结构的迭代器
            .flatten()
            .collect(),
    )
    .unwrap()
}

/**
402. 移掉 K 位数字

给你一个以字符串表示的非负整数 num 和一个整数 k ，移除这个数中的 k 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。

```
示例 1 ：

输入：num = "1432219", k = 3
输出："1219"
解释：移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219 。
示例 2 ：

输入：num = "10200", k = 1
输出："200"
解释：移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
示例 3 ：

输入：num = "10", k = 2
输出："0"
解释：从原数字移除所有的数字，剩余为空就是 0 。


提示：

1 <= k <= num.length <= 105
num 仅由若干位数字（0 - 9）组成
除了 0 本身之外，num 不含任何前导零
```
 */
pub fn remove_kdigits(num: String, k: i32) -> String {
    // 需要保留字符串长度
    let remain = num.len() as i32 - k;
    if num.len() == k as usize {
        // 如果字符串长度等于k，直接返回0
        return "0".to_string();
    }
    let mut k = k;
    // 定义一个栈
    let mut stack = Vec::new();
    for c in num.chars() {
        // 需要注意的点：弹出元素时是while循环一直弹，而不是if判断弹一次。不然比如【"1234567890" 0】用例会出错。
        while k > 0 && !stack.is_empty() && stack.last().unwrap() > &c {
            // 如果栈中最后元素比当前元素大，就出栈
            stack.pop();
            // 移除数-1
            k -= 1;
        }
        stack.push(c);
    }

    let ns = stack
        .into_iter()
        .take(remain as usize)
        // 去掉前导0
        .skip_while(|&c| c == '0')
        .collect::<String>();
    if ns.is_empty() {
        "0".to_string()
    } else {
        ns
    }
}

/// 321. 拼接最大数
///
/// 给定长度分别为 m 和 n 的两个数组，其元素由 0-9 构成，表示两个自然数各位上的数字。现在从这两个数组中选出 k (k <= m + n) 个数字拼接成一个新的数，要求从同一个数组中取出的数字保持其在原数组中的相对顺序。
///
/// 求满足该条件的最大数。结果返回一个表示该最大数的长度为 k 的数组。
///
/// 说明: 请尽可能地优化你算法的时间和空间复杂度。
/// ```
/// 示例 1:

/// 输入:
/// nums1 = [3, 4, 6, 5]
/// nums2 = [9, 1, 2, 5, 8, 3]
/// k = 5
/// 输出:
/// [9, 8, 6, 5, 3]
/// 示例 2:

/// 输入:
/// nums1 = [6, 7]
/// nums2 = [6, 0, 4]
/// k = 5
/// 输出:
/// [6, 7, 6, 0, 4]
/// 示例 3:

/// 输入:
/// nums1 = [3, 9]
/// nums2 = [8, 9]
/// k = 3
/// 输出:
/// [9, 8, 9]
/// 提示：

/// m == nums1.length
/// n == nums2.length
/// 1 <= m, n <= 500
/// 0 <= nums1[i], nums2[i] <= 9
/// 1 <= k <= m + n
/// ```
pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    let (m, n) = (nums1.len(), nums2.len());
    ((k - n as i32).max(0)..k.min(m as i32) + 1)
        // 可以看成x是要从nums1中取的个数，k-x是要从nums2中取的个数
        .map(|x| merge(select_max(&nums1, x), select_max(&nums2, k - x)))
        .max()
        .unwrap()
}

/// 合并两个数组
fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i..] > nums2[j..] {
            ans.push(nums1[i]);
            i += 1;
        } else {
            ans.push(nums2[j]);
            j += 1;
        }
    }
    ans.extend(&nums1[i..]);
    ans.extend(&nums2[j..]);
    ans
}

/// 选择最大的k个数
fn select_max(nums: &[i32], k: i32) -> Vec<i32> {
    // 要丢弃的位数
    let mut to_drop = nums.len() - k as usize;
    let mut stk = vec![i32::MAX];
    // for循环总体操作是把nums里的数压入stk，但当后一位大于前一位的时候，
    // 把前一位弹出，同时to_drop减1
    for num in nums {
        while to_drop > 0 && stk.last() < Some(num) {
            stk.pop();
            to_drop -= 1;
        }
        stk.push(*num);
    }
    //这里相当于如果最后还有需要丢弃的，直接丢弃尾部的数据
    stk[1..stk.len() - to_drop].to_owned()
}

/// 1081. 不同字符的最小子序列

/// 返回 s 字典序最小的子序列，该子序列包含 s 的所有不同字符，且只包含一次。

/// ```
/// 示例 1：

/// 输入：s = "bcabc"
/// 输出："abc"
/// 示例 2：

/// 输入：s = "cbacdcbc"
/// 输出："acdb"

/// 提示：

/// 1 <= s.length <= 1000
/// s 由小写英文字母组成
/// ```

/// 注意：该题与 316 https://leetcode.cn/problems/remove-duplicate-letters/ 相同
pub fn smallest_subsequence(s: String) -> String {
    // 定义一个栈
    let mut stack = Vec::new();
    // 存放字符是否在栈中
    let mut in_stack = [false; 26];
    // 字符出现的次数
    let mut cnt = [0; 26];
    // 统计字符出现的次数
    s.chars().for_each(|c| cnt[c as usize - 'a' as usize] += 1);
    // 遍历字符串
    for c in s.chars() {
        // 获取字符的下标
        let idx = c as usize - 'a' as usize;
        // 如果字符不在栈中
        if !in_stack[idx] {
            while let Some(&top) = stack.last() {
                let t_idx = top as usize - 'a' as usize;
                // 如果栈顶元素比当前元素大，且后续中还有栈顶元素
                if top > c && cnt[t_idx] > 0 {
                    // 栈顶元素出栈，且标记栈中不存在此元素
                    in_stack[t_idx] = false;
                    stack.pop();
                } else {
                    // 否则终止循环
                    break;
                }
            }
            // 入栈并标记栈中存在此元素
            in_stack[idx] = true;
            stack.push(c);
        }
        // 如果在栈中存在
        cnt[idx] -= 1;
    }
    stack.iter().collect()
}

/// 636. 函数的独占时间

/// 有一个 单线程 CPU 正在运行一个含有 n 道函数的程序。每道函数都有一个位于  0 和 n-1 之间的唯一标识符。

/// 函数调用 存储在一个 调用栈 上 ：当一个函数调用开始时，它的标识符将会推入栈中。而当一个函数调用结束时，它的标识符将会从栈中弹出。标识符位于栈顶的函数是 当前正在执行的函数 。每当一个函数开始或者结束时，将会记录一条日志，包括函数标识符、是开始还是结束、以及相应的时间戳。

/// 给你一个由日志组成的列表 logs ，其中 logs[i] 表示第 i 条日志消息，该消息是一个按 "{function_id}:{"start" | "end"}:{timestamp}" 进行格式化的字符串。例如，"0:start:3" 意味着标识符为 0 的函数调用在时间戳 3 的 起始开始执行 ；而 "1:end:2" 意味着标识符为 1 的函数调用在时间戳 2 的 末尾结束执行。注意，函数可以 调用多次，可能存在递归调用 。

/// 函数的 独占时间 定义是在这个函数在程序所有函数调用中执行时间的总和，调用其他函数花费的时间不算该函数的独占时间。例如，如果一个函数被调用两次，一次调用执行 2 单位时间，另一次调用执行 1 单位时间，那么该函数的 独占时间 为 2 + 1 = 3 。

/// 以数组形式返回每个函数的 独占时间 ，其中第 i 个下标对应的值表示标识符 i 的函数的独占时间。

/// ```
/// 示例 1：

/// 输入：n = 2, logs = ["0:start:0","1:start:2","1:end:5","0:end:6"]
/// 输出：[3,4]
/// 解释：
/// 函数 0 在时间戳 0 的起始开始执行，执行 2 个单位时间，于时间戳 1 的末尾结束执行。
/// 函数 1 在时间戳 2 的起始开始执行，执行 4 个单位时间，于时间戳 5 的末尾结束执行。
/// 函数 0 在时间戳 6 的开始恢复执行，执行 1 个单位时间。
/// 所以函数 0 总共执行 2 + 1 = 3 个单位时间，函数 1 总共执行 4 个单位时间。
/// 示例 2：

/// 输入：n = 1, logs = ["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
/// 输出：[8]
/// 解释：
/// 函数 0 在时间戳 0 的起始开始执行，执行 2 个单位时间，并递归调用它自身。
/// 函数 0（递归调用）在时间戳 2 的起始开始执行，执行 4 个单位时间。
/// 函数 0（初始调用）恢复执行，并立刻再次调用它自身。
/// 函数 0（第二次递归调用）在时间戳 6 的起始开始执行，执行 1 个单位时间。
/// 函数 0（初始调用）在时间戳 7 的起始恢复执行，执行 1 个单位时间。
/// 所以函数 0 总共执行 2 + 4 + 1 + 1 = 8 个单位时间。
/// 示例 3：

/// 输入：n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]
/// 输出：[7,1]
/// 解释：
/// 函数 0 在时间戳 0 的起始开始执行，执行 2 个单位时间，并递归调用它自身。
/// 函数 0（递归调用）在时间戳 2 的起始开始执行，执行 4 个单位时间。
/// 函数 0（初始调用）恢复执行，并立刻调用函数 1 。
/// 函数 1在时间戳 6 的起始开始执行，执行 1 个单位时间，于时间戳 6 的末尾结束执行。
/// 函数 0（初始调用）在时间戳 7 的起始恢复执行，执行 1 个单位时间，于时间戳 7 的末尾结束执行。
/// 所以函数 0 总共执行 2 + 4 + 1 = 7 个单位时间，函数 1 总共执行 1 个单位时间。
/// 示例 4：

/// 输入：n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:7","1:end:7","0:end:8"]
/// 输出：[8,1]
/// 示例 5：

/// 输入：n = 1, logs = ["0:start:0","0:end:0"]
/// 输出：[1]

/// 提示：

/// 1 <= n <= 100
/// 1 <= logs.length <= 500
/// 0 <= function_id < n
/// 0 <= timestamp <= 109
/// 两个开始事件不会在同一时间戳发生
/// 两个结束事件不会在同一时间戳发生
/// 每道函数都有一个对应 "start" 日志的 "end" 日志
/// ```
pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    // 记录结果
    let mut ans = vec![0; n as usize];
    // 定义一个栈
    let mut stack = Vec::new();
    // 记录当前时间，初始从0开始，这里初始化为-1
    let mut cur = -1;
    // 遍历logs
    for log in logs {
        // 拆分log
        let log_sp = log.split(":").collect::<Vec<&str>>();
        // 程序编号
        let idx = log_sp[0].parse::<i32>().unwrap();
        // 程序开始或结束时间
        let ts = log_sp[2].parse::<i32>().unwrap();
        if log_sp[1] == "start" {
            // 当 log[i] 为函数调用：此时从该函数的调用发起时间 ts 到上一次记录的当前时间，
            // 都是前一函数的执行时间，因此可以将 ts - cur 累加到栈帧中的前一函数。即若栈不为空，
            // 则将该时间累加到栈顶对应的函数上，然后将 log[i]log[i]log[i] 入栈，同时更新当前时间
            if !stack.is_empty() {
                // 栈不为空，将 ts - cur 累加到栈帧中的前一函数
                ans[*stack.last().unwrap() as usize] += ts - cur;
            }
            // 入栈
            stack.push(idx);
            // 把当前时间记录为程序开始时间
            cur = ts;
        } else {
            // 当 log[i] 为函数结束：此时栈顶元素必然是该函数的调用记录，
            // 此时 log[i] 的结束时间与上一次记录的当前时间的时长 ts - cur + 1，
            // 必然是该函数的执行时间，将其累加到当前函数中，并更新当前时间。
            // 弹出栈中最后的元素
            let pop = stack.pop().unwrap();
            ans[pop as usize] += ts - cur + 1;
            cur = ts + 1;
        }
    }
    ans
}

/// 32. 最长有效括号
///
/// 给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。
///
/// ```
/// 示例 1：
///
/// 输入：s = "(()"
/// 输出：2
/// 解释：最长有效括号子串是 "()"
///
/// 示例 2：
///
/// 输入：s = ")()())"
/// 输出：4
/// 解释：最长有效括号子串是 "()()"
///
/// 示例 3：
///
/// 输入：s = ""
/// 输出：0
///
/// 提示：
///
/// 0 <= s.length <= 3 * 104
/// s[i] 为 '(' 或 ')'
/// ```
pub fn longest_valid_parentheses(s: String) -> i32 {
    // 定义一个栈
    //放入-1可以防止当第一个char是')'的时候发生越界异常
    let mut stack = vec![-1];
    // 定义最长有效括号长度
    let mut max_len = 0;
    // 遍历栈找寻合适的左右括号
    for (idx, c) in s.chars().enumerate() {
        let idx = idx as i32;
        if c == '(' {
            // 如果找到左括号则入栈，为寻找对应右括号做铺垫
            stack.push(idx);
        } else {
            // 如果是右括号则出栈
            stack.pop();
            if stack.is_empty() {
                // 但是如果栈是空的话还是得（单身的）把右括号放进来
                stack.push(idx);
            } else {
                // 当前全部人数减去剩余无法配对的人数（单身）即max_len
                max_len = max_len.max(idx - stack.last().unwrap());
            }
        }
    }
    max_len as i32
}

/// 2696. 删除子串后的字符串最小长度
///
/// 给你一个仅由 大写 英文字符组成的字符串 s 。
///
/// 你可以对此字符串执行一些操作，在每一步操作中，你可以从 s 中删除 任一个 "AB" 或 "CD" 子字符串。
///
/// 通过执行操作，删除所有 "AB" 和 "CD" 子串，返回可获得的最终字符串的 最小 可能长度。
///
/// 注意，删除子串后，重新连接出的字符串可能会产生新的 "AB" 或 "CD" 子串。
///
/// ```
/// 示例 1：
///
/// 输入：s = "ABFCACDB"
/// 输出：2
/// 解释：你可以执行下述操作：
/// - 从 "ABFCACDB" 中删除子串 "AB"，得到 s = "FCACDB" 。
/// - 从 "FCACDB" 中删除子串 "CD"，得到 s = "FCAB" 。
/// - 从 "FCAB" 中删除子串 "AB"，得到 s = "FC" 。
/// 最终字符串的长度为 2 。
/// 可以证明 2 是可获得的最小长度。
///
/// 示例 2：
///
/// 输入：s = "ACBBD"
/// 输出：5
/// 解释：无法执行操作，字符串长度不变。
///
/// 提示：
///
/// 1 <= s.length <= 100
/// s 仅由大写英文字母组成
/// ```
pub fn min_length(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    // 定义一个栈
    let mut stack = Vec::with_capacity(s.len());
    for c in s {
        if !stack.is_empty()
            && ((*stack.last().unwrap() == 'A' && c == 'B')
                || (*stack.last().unwrap() == 'C' && c == 'D'))
        {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.len() as i32
}

/// 682. 棒球比赛
///
/// 你现在是一场采用特殊赛制棒球比赛的记录员。这场比赛由若干回合组成，过去几回合的得分可能会影响以后几回合的得分。
///
/// 比赛开始时，记录是空白的。你会得到一个记录操作的字符串列表 ops，其中 ops[i] 是你需要记录的第 i 项操作，ops 遵循下述规则：
///
/// 整数 x - 表示本回合新获得分数 x
///
/// "+" - 表示本回合新获得的得分是前两次得分的总和。题目数据保证记录此操作时前面总是存在两个有效的分数。
///
/// "D" - 表示本回合新获得的得分是前一次得分的两倍。题目数据保证记录此操作时前面总是存在一个有效的分数。
///
/// "C" - 表示前一次得分无效，将其从记录中移除。题目数据保证记录此操作时前面总是存在一个有效的分数。
///
/// 请你返回记录中所有得分的总和。
///
/// ```
/// 示例 1：
/// 输入：ops = ["5","2","C","D","+"]
/// 输出：30
/// 解释：
/// "5" - 记录加 5 ，记录现在是 [5]
/// "2" - 记录加 2 ，记录现在是 [5, 2]
/// "C" - 使前一次得分的记录无效并将其移除，记录现在是 [5].
/// "D" - 记录加 2 * 5 = 10 ，记录现在是 [5, 10].
/// "+" - 记录加 5 + 10 = 15 ，记录现在是 [5, 10, 15].
/// 所有得分的总和 5 + 10 + 15 = 30
///
/// 示例 2：
/// 输入：ops = ["5","-2","4","C","D","9","+","+"]
/// 输出：27
/// 解释：
/// "5" - 记录加 5 ，记录现在是 [5]
/// "-2" - 记录加 -2 ，记录现在是 [5, -2]
/// "4" - 记录加 4 ，记录现在是 [5, -2, 4]
/// "C" - 使前一次得分的记录无效并将其移除，记录现在是 [5, -2]
/// "D" - 记录加 2 * -2 = -4 ，记录现在是 [5, -2, -4]
/// "9" - 记录加 9 ，记录现在是 [5, -2, -4, 9]
/// "+" - 记录加 -4 + 9 = 5 ，记录现在是 [5, -2, -4, 9, 5]
/// "+" - 记录加 9 + 5 = 14 ，记录现在是 [5, -2, -4, 9, 5, 14]
/// 所有得分的总和 5 + -2 + -4 + 9 + 5 + 14 = 27
/// 示例 3：
/// 输入：ops = ["1"]
/// 输出：1
/// ```
///
/// 提示：
///
/// 1 <= ops.length <= 1000
///
/// ops[i] 为 "C"、"D"、"+"，或者一个表示整数的字符串。整数范围是 [-3 * 104, 3 * 104]
///
/// 对于 "+" 操作，题目数据保证记录此操作时前面总是存在两个有效的分数
///
/// 对于 "C" 和 "D" 操作，题目数据保证记录此操作时前面总是存在一个有效的分数
pub fn cal_points(operations: Vec<String>) -> i32 {
    // 定义栈
    let mut stack = Vec::with_capacity(operations.len());
    for v in operations {
        match v.as_str() {
            "C" => {
                stack.pop();
            }
            "D" => {
                stack.push(stack.last().unwrap() * 2);
            }
            "+" => {
                let one = stack.pop().unwrap();
                let two = stack.pop().unwrap();
                stack.push(two);
                stack.push(one);
                stack.push(one + two);
            }
            _ => stack.push(v.parse::<i32>().unwrap()),
        }
    }
    stack.iter().sum()
}

/// 1700. 无法吃午餐的学生数量
///
/// 学校的自助午餐提供圆形和方形的三明治，分别用数字 0 和 1 表示。所有学生站在一个队列里，每个学生要么喜欢圆形的要么喜欢方形的。
///
/// 餐厅里三明治的数量与学生的数量相同。所有三明治都放在一个 栈 里，每一轮：
///
/// 如果队列最前面的学生 喜欢 栈顶的三明治，那么会 拿走它 并离开队列。
///
/// 否则，这名学生会 放弃这个三明治 并回到队列的尾部。
///
/// 这个过程会一直持续到队列里所有学生都不喜欢栈顶的三明治为止。
///
/// 给你两个整数数组 students 和 sandwiches ，其中 sandwiches[i] 是栈里面第 i​​​​​​ 个三明治的类型（i = 0 是栈的顶部）， students[j] 是初始队列里第 j​​​​​​ 名学生对三明治的喜好（j = 0 是队列的最开始位置）。请你返回无法吃午餐的学生数量。
///
/// ```
/// 示例 1：
/// 输入：students = [1,1,0,0], sandwiches = [0,1,0,1]
/// 输出：0
/// 解释：
/// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [1,0,0,1]。
/// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [0,0,1,1]。
/// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [0,1,1]，三明治栈为 sandwiches = [1,0,1]。
/// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [1,1,0]。
/// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [1,0]，三明治栈为 sandwiches = [0,1]。
/// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [0,1]。
/// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [1]，三明治栈为 sandwiches = [1]。
/// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = []，三明治栈为 sandwiches = []。
/// 所以所有学生都有三明治吃。
///
/// 示例 2：
/// 输入：students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]
/// 输出：3
/// ```
/// ```
/// 提示：
/// 1 <= students.length, sandwiches.length <= 100
/// students.length == sandwiches.length
/// sandwiches[i] 要么是 0 ，要么是 1 。
/// students[i] 要么是 0 ，要么是 1 。
/// ```
pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    // 统计students 1 的个数
    let mut one = students.iter().filter(|s| **s == 1).count();
    // students 0 的个数
    let mut zero = students.len() - one;
    // 循环弹出sandwiches栈中的数据
    for v in sandwiches {
        if v == 0 && zero > 0 {
            zero -= 1;
        } else if v == 1 && one > 0 {
            one -= 1;
        } else {
            // 当 one 和 zero 都减为0之后退出循环
            break;
        }
    }
    // 如果三明治消耗完了，剩下的学生总和
    (one + zero) as i32
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
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(eval_rpn(tokens), 9);
    }

    #[test]
    fn test_build_array() {
        let target = vec![1, 3];
        let n = 3;
        assert_eq!(
            build_array(target, n),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }

    #[test]
    fn test_remove_duplicate_letters() {
        let s = "bcabc".to_string();
        assert_eq!(remove_duplicate_letters(s), "abc".to_string());
    }

    #[test]
    fn test_remove_duplicates_ii() {
        let s = "dtpdtaaaaaaaaappppppppppppppppppppaaaaaaaaaaxxxxxxxxxxxxxxsssssssssjjjjjjjjjjjjjjjjjjjjxxxxxxxxxxxxxxxxxxxxsssssssjjjjjjjjjjjjjjjjjjjjssssxxxxxxatdwvvpctpggggggggggggggggggggajagglaaaaaaaaaaaaaaaaaaaa".to_string();
        let k = 20;
        assert_eq!(
            remove_duplicates_ii(s, k),
            "dtpdttdwvvpctpajaggl".to_string()
        );
    }

    #[test]
    fn test_remove_kdigits() {
        let num = "9".to_string();
        let k = 1;
        assert_eq!(remove_kdigits(num, k), "0".to_string());
    }

    #[test]
    fn test_smallest_subsequence() {
        let s = "cdadabcc".to_string();
        assert_eq!(smallest_subsequence(s), "adbc".to_string());
    }

    #[test]
    fn test_exclusive_time() {
        let n = 2;
        let logs = vec![
            "0:start:0".to_string(),
            "1:start:2".to_string(),
            "1:end:5".to_string(),
            "0:end:6".to_string(),
        ];
        assert_eq!(exclusive_time(n, logs), vec![3, 4]);
    }

    #[test]
    fn test_longest_valid_parentheses() {
        let s = "()".to_string();
        assert_eq!(longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_min_length() {
        let s = "ABFCACDB".to_string();
        assert_eq!(min_length(s), 2);
    }

    #[test]
    fn test_cal_points() {
        let operations = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        assert_eq!(cal_points(operations), 27);
    }
}
