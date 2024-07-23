# 滑动窗口

## 固定长度滑动窗口

### 代码模版

题目一般会给你一个k区间，然后求k区间内的某种结果

```rust
    // 一般先处理 0到k 的第一个窗口结果
    for i in 0..k as usize {
        
    }
    
    // 滑动窗口处理后续 k到nums.len() 元素
    for i in k as usize..nums.len() {
        
    }
```

## 动态滑动窗口

### 最小滑窗模板

给定数组 nums，定义滑窗的左右边界 i, j，求满足某个条件的滑窗的最小长度。

```python
while j < len(nums):
    判断[i, j]是否满足条件
    while 满足条件：
        不断更新结果(注意在while内更新！)
        i += 1 （最大程度的压缩i，使得滑窗尽可能的小）
    j += 1
```

### 最大滑窗模板

给定数组 nums，定义滑窗的左右边界 i, j，求满足某个条件的滑窗的最大长度。

```python
while j < len(nums):
    判断[i, j]是否满足条件
    while 不满足条件：
        i += 1 （最保守的压缩i，一旦满足条件了就退出压缩i的过程，使得滑窗尽可能的大）
    不断更新结果（注意在while外更新！）
    j += 1
```

最小滑窗和最大滑窗，关键的区别在于，最大滑窗是在迭代右移右边界的过程中更新结果，而最小滑窗是在迭代右移左边界的过程中更新结果。因此虽然都是滑窗，但是两者的模板和对应的贪心思路并不一样，而真正理解后就可以在lc.76，lc.904，lc.3, lc.1004写出非常无脑的代码。

# 分组循环

适用场景：按照题目要求，数组会被分割成若干组，且每一组的判断/处理逻辑是一样的。

## 核心思想：

- 外层循环负责遍历组之前的准备工作（记录开始位置），和遍历组之后的统计工作（更新答案最大值）。

- 内层循环负责遍历组，找出这一组最远在哪结束。

这个写法的好处是，各个逻辑块分工明确，也不需要特判最后一组（易错点）。以我的经验，这个写法是所有写法中最不容易出 bug 的，推荐大家记住。

## 复杂度分析

- 时间复杂度：O(n)，其中 n 为 nums 的长度。时间复杂度乍一看是 O(n^2)，但注意变量 i 只会增加，不会重置也不会减少。所以二重循环总共循环 O(n) 次，所以时间复杂度是 O(n)。

- 空间复杂度：O(1)。仅用到若干额外变量。

## 分组循环模版

```python
n = len(nums)
i = 0
while i < n:
    start = i
    while i < n and ...:
        i += 1
    # 从 start 到 i-1 是一组
    # 下一组从 i 开始，无需 i += 1
```

# 双指针

## 移除元素

题目一般会要求从数组或字符串中移除某些元素

### 代码模版

```rust
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut slow = 0;
    // 遍历数组，循环不变量为 slow - fast 之间的元素都不是val
    for fast in 0..nums.len() {
        // 当不满足条件的数据都会往数组左边进行移动
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    // 返回temp位置的索引就是数组的长度
    slow as i32
}
```

### 字符串反转

题目一般会要求对数组或字符串进行反转

### 代码模版

```rust
/// 345. 反转字符串中的元音字母
pub fn reverse_vowels(s: String) -> String {
    // 定义元音字母
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = s.len() - 1;
    while left < right {
        if !vowels.contains(&s[left]) {
            // 左边不满足条件右移左指针
            left += 1;
        } else if !vowels.contains(&s[right]) {
            // 右边不满足条件左移右指针
            right -= 1;
        } else {
            // 满足条件后对左右指针位置的元素进行位置交换
            let temp = s[left];
            s[left] = s[right];
            s[right] = temp;
            left += 1;
            right -= 1;
        }
    }
    s.iter().collect()
}
```

## 区间反转

对某些区间的数组或者字符串进行反转：如对字符串中的单词进行反转

### 代码模版

```rust
/// 557. 反转字符串中的单词 III
pub fn reverse_words_iii(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let len = s.len();
    // 定义起始位置
    let mut start = 0;
    for i in 0..=len {
        if i == len || s[i].is_ascii_whitespace() {
            // 如果遇到空格或者字符串末尾，就反转 start -- i - 1
            reverse(&mut s, start, i - 1);
            start = i + 1;
        }
    }
    s.iter().collect::<String>()
}

/// 对范围内的数组内容进行反转
fn reverse(s: &mut Vec<char>, mut begin: usize, mut end: usize) {
    // 反转begin到end之间的字符
    while begin < end {
        let temp = s[begin];
        s[begin] = s[end];
        s[end] = temp;
        begin += 1;
        end -= 1;
    }
}
```

## 多元素之和

多元素之和一般分为两数之和、三数之和、四数之和





# 前缀和

### 代码模版

```rust
		// 定义前缀和数组
    let mut prefix = vec![0; arr.len() + 1];
    // 计算前缀和
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] + arr[i];
    }
```

# 二分算法

二分的初始范围，还可以进一步缩小至
$$
[min(citations[0],n)+1,min(citations[n−1],n)]
$$
但为了缩小二分初始范围，多访问了两次数组，相比直接在 [1,n] 上二分，并没有明显的优势。

所以在正文中，我没有采纳上述优化。

为什么可以缩小到这个范围，留作思考题。欢迎在评论区发表你的思路。

## 代码模板

### 左闭右闭区间写法

```rust
pub fn h_index(citations: Vec<i32>) -> i32 {
        // 在区间 [left, right] 内询问
        let n = citations.len();
        let mut left = 1;
        let mut right = n;
        while left <= right { // 区间不为空
            // 循环不变量：
            // left-1 的回答一定为「是」
            // right+1 的回答一定为「否」
            let mid = (left + right) / 2;
            // 引用次数最多的 mid 篇论文，引用次数均 >= mid
            if citations[n - mid] >= mid as i32 {
                left = mid + 1; // 询问范围缩小到 [mid+1, right]
            } else {
                right = mid - 1; // 询问范围缩小到 [left, mid-1]
            }
        }
        // 循环结束后 right 等于 left-1，回答一定为「是」
        // 根据循环不变量，right 现在是最大的回答为「是」的数
        right as i32
    }
```

### 左闭右开区间写法

```rust
pub fn h_index(citations: Vec<i32>) -> i32 {
        // 在区间 [left, right) 内询问
        let n = citations.len();
        let mut left = 1;
        let mut right = n + 1;
        while left < right { // 区间不为空
            // 循环不变量：
            // left-1 的回答一定为「是」
            // right 的回答一定为「否」
            let mid = (left + right) / 2;
            // 引用次数最多的 mid 篇论文，引用次数均 >= mid
            if citations[n - mid] >= mid as i32 {
                left = mid + 1; // 询问范围缩小到 [mid+1, right)
            } else {
                right = mid; // 询问范围缩小到 [left, mid)
            }
        }
        // 根据循环不变量，left-1 现在是最大的回答为「是」的数
        left as i32 - 1
    }
```

### 左开右闭区间写法

```rust
pub fn h_index(citations: Vec<i32>) -> i32 {
        // 在区间 (left, right] 内询问
        let n = citations.len();
        let mut left = 0;
        let mut right = n;
        while left < right { // 区间不为空
            // 循环不变量：
            // left 的回答一定为「是」
            // right+1 的回答一定为「否」
            let mid = (left + right + 1) / 2; // 保证 mid 在二分区间内
            // 引用次数最多的 mid 篇论文，引用次数均 >= mid
            if citations[n - mid] >= mid as i32 {
                left = mid; // 询问范围缩小到 (mid, right]
            } else {
                right = mid - 1; // 询问范围缩小到 (left, mid-1]
            }
        }
        // 根据循环不变量，left 现在是最大的回答为「是」的数
        left as i32
    }
```

### 开区间写法

```rust
pub fn h_index(citations: Vec<i32>) -> i32 {
        // 在区间 (left, right) 内询问
        let n = citations.len();
        let mut left = 0;
        let mut right = n + 1;
        while left + 1 < right { // 区间不为空
            // 循环不变量：
            // left 的回答一定为「是」
            // right 的回答一定为「否」
            let mid = (left + right) / 2;
            // 引用次数最多的 mid 篇论文，引用次数均 >= mid
            if citations[n - mid] >= mid as i32 {
                left = mid; // 询问范围缩小到 (mid, right)
            } else {
                right = mid; // 询问范围缩小到 (left, mid)
            }
        }
        // 根据循环不变量，left 现在是最大的回答为「是」的数
        left as i32
    }
```

### 复杂度分析

- 时间复杂度：O(log⁡n)，其中 n 为 citations 的长度。每次循环，都会将二分范围减少一半，所以循环会执行 O(log⁡n)次，所以时间复杂度为 O(log⁡n)。
- 空间复杂度：O(1)。仅用到若干额外变量。

# 单调栈

## 解决问题类型

单调栈解决 Next Greater Number 一类问题

栈（stack）是很简单的一种数据结构，先进后出的逻辑顺序，符合某些问题的特点，比如说函数调用栈。

单调栈实际上就是栈，只是利用了一些巧妙的逻辑，使得每次新元素入栈后，栈内的元素都保持有序（单调递增或单调递减）。

听起来有点像堆（heap）？不是的，单调栈用途不太广泛，只处理一种典型的问题，叫做 Next Greater Element。本文用讲解单调队列的算法模版解决这类问题，并且探讨处理「循环数组」的策略。

首先，讲解 Next Greater Number 的原始问题：给你一个数组，返回一个等长的数组，对应索引存储着下一个更大元素，如果没有更大的元素，就存 -1。不好用语言解释清楚，直接上一个例子：

给你一个数组 [2,1,2,4,3]，你返回数组 [4,2,4,-1,-1]。

解释：第一个 2 后面比 2 大的数是 4; 1 后面比 1 大的数是 2；第二个 2 后面比 2 大的数是 4; 4 后面没有比 4 大的数，填 -1；3 后面没有比 3 大的数，填 -1。

这道题的暴力解法很好想到，就是对每个元素后面都进行扫描，找到第一个更大的元素就行了。但是暴力解法的时间复杂度是 O(n^2)。

这个问题可以这样抽象思考：把数组的元素想象成并列站立的人，元素大小想象成人的身高。这些人面对你站成一列，如何求元素「2」的 Next Greater Number 呢？很简单，如果能够看到元素「2」，那么他后面可见的第一个人就是「2」的 Next Greater Number，因为比「2」小的元素身高不够，都被「2」挡住了，第一个露出来的就是答案。

![](https://dreamyao.oss-cn-chengdu.aliyuncs.com/2024-03-03-140730.png)

### 代码模板

```rust
	pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
      // 定义一个栈来存储 temperatures 下标
      let mut stack = Vec::with_capacity(temperatures.len());
      // 遍历temperatures
      for (idx, t) in temperatures.iter().enumerate() {
          // 如果栈不为空，且栈顶元素小于当前元素则出栈，否则入栈
          while !stack.is_empty() && *t > temperatures[*stack.last().unwrap() as usize] {
            	// 比当前元素小的元素的下标都出栈
              let min_idx = stack.pop().unwrap();
              // 当前索引 - 栈顶元素索引就能得到他们的间隔
              ans[min_idx as usize] = idx as i32 - min_idx;
          }
        	// 当前元素下标入栈
          stack.push(idx as i32);
      }
      ans
}
```

# 哈希表

## 原数组哈希法

### 实例题：

[41. 缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive/)

给你一个未排序的整数数组 `nums` ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 `O(n)` 并且只使用常数级别额外空间的解决方案。 

**示例 1：**

```
输入：nums = [1,2,0]
输出：3
解释：范围 [1,2] 中的数字都在数组中。
```

**示例 2：**

```
输入：nums = [3,4,-1,1]
输出：2
解释：1 在数组中，但 2 没有。
```

**示例 3：**

```
输入：nums = [7,8,9,11,12]
输出：1
解释：最小的正数 1 没有出现。
```

 

**提示：**

- `1 <= nums.length <= 105`
- `-231 <= nums[i] <= 231 - 1`

最早知道这个思路是在《剑指 Offe》这本书上看到的，感兴趣的朋友不妨做一下这道问题：剑指 Offer 03. 数组中重复的数字。下面简要叙述：

- 由于题目要求我们「只能使用常数级别的空间」，而要找的数一定在 [1, N + 1] 左闭右闭（这里 N 是数组的长度）这个区间里。因此，我们可以就把原始的数组当做哈希表来使用。事实上，哈希表其实本身也是一个数组；

- 我们要找的数就在 [1, N + 1] 里，最后 N + 1 这个元素我们不用找。因为在前面的 N 个元素都找不到的情况下，我们才返回 N + 1；
- 那么，我们可以采取这样的思路：就把 1 这个数放到下标为 0 的位置， 2 这个数放到下标为 1 的位置，按照这种思路整理一遍数组。然后我们再遍历一次数组，第 1 个遇到的它的值不等于下标的那个数，就是我们要找的缺失的第一个正数。
- 这个思想就相当于我们自己编写哈希函数，这个哈希函数的规则特别简单，那就是数值为 i 的数映射到下标为 i - 1 的位置。

我们来看一下这个算法是如何应用在示例 2 上的。

![](https://dreamyao.oss-cn-chengdu.aliyuncs.com/2024-03-12-085224.png)

### 代码模板

```rust
		for i in 0..nums.len() {
        // 我们可以采取这样的思路：就把 1 这个数放到下标为 0 的位置， 2 这个数放到下标为 1 的位置，按照这种思路整理一遍数组。
        // 然后我们再遍历一次数组，第 1 个遇到的它的值不等于下标的那个数，就是我们要找的缺失的第一个正数。
        // 通过这种方式在原数组中对数据位置进行交换
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[(nums[i] - 1) as usize] != nums[i]
        {
            let a = nums[i] as usize - 1;
            nums.swap(a, i)
        }
    }
```

## 题单

[41. 缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive)

[442. 数组中重复的数据](https://leetcode-cn.com/problems/find-all-duplicates-in-an-array/)

[448. 找到所有数组中消失的数字](https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array/)

[LCR 120. 寻找文件副本](https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof)

# 二叉树

递归三部曲