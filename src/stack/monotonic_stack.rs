use std::collections::HashMap;

/**
739. 每日温度

给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i 天，
下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。


```
示例 1:

输入: temperatures = [73,74,75,71,69,72,76,73]
输出: [1,1,4,2,1,1,0,0]
示例 2:

输入: temperatures = [30,40,50,60]
输出: [1,1,1,0]
示例 3:

输入: temperatures = [30,60,90]
输出: [1,1,0]


提示：

1 <= temperatures.length <= 105
30 <= temperatures[i] <= 100
```

写法一：从左向右遍历

总结：及时去掉无用数据，保持栈中数据有序
 */
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let len = temperatures.len();
    // 定义一个栈来存储 temperatures 下标
    let mut stack = Vec::with_capacity(len);
    // 定义一个数组来存放结果
    let mut ans = vec![0; len];
    // 遍历temperatures
    for (idx, t) in temperatures.iter().enumerate() {
        // 如果栈不为空，且栈顶元素小于当前元素则出栈，否则入栈
        while !stack.is_empty() && *t > temperatures[*stack.last().unwrap() as usize] {
            let min_idx = stack.pop().unwrap();
            // 当前索引 - 栈顶元素索引就能得到他们的间隔
            ans[min_idx as usize] = idx as i32 - min_idx;
        }
        stack.push(idx as i32);
    }
    ans
}

/**
739. 每日温度

给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i 天，
下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。


```
示例 1:

输入: temperatures = [73,74,75,71,69,72,76,73]
输出: [1,1,4,2,1,1,0,0]
示例 2:

输入: temperatures = [30,40,50,60]
输出: [1,1,1,0]
示例 3:

输入: temperatures = [30,60,90]
输出: [1,1,0]


提示：

1 <= temperatures.length <= 105
30 <= temperatures[i] <= 100
```

写法二：从右向左遍历

总结：及时去掉无用数据，保持栈中数据有序
 */
pub fn daily_temperatures_ii(temperatures: Vec<i32>) -> Vec<i32> {
    let len = temperatures.len();
    // 定义一个栈来存储 temperatures 下标
    let mut stack = Vec::with_capacity(len);
    // 定义一个数组来存放结果
    let mut ans = vec![0; len];
    // 遍历temperatures
    for (i, t) in temperatures.iter().enumerate().rev() {
        // 如果栈不为空，且栈顶元素小于等于当前元素则出栈
        while !stack.is_empty() && *t >= temperatures[*stack.last().unwrap() as usize] {
            stack.pop();
        }
        if !stack.is_empty() {
            // 当前索引 - 栈顶元素索引就能得到他们的间隔
            ans[i] = *stack.last().unwrap() as i32 - i as i32;
        }
        stack.push(i as i32);
    }
    ans
}

/**
496. 下一个更大元素 I

nums1 中数字 x 的 下一个更大元素 是指 x 在 nums2 中对应位置 右侧 的 第一个 比 x 大的元素。

给你两个 没有重复元素 的数组 nums1 和 nums2 ，下标从 0 开始计数，其中nums1 是 nums2 的子集。

对于每个 0 <= i < nums1.length ，找出满足 nums1[i] == nums2[j] 的下标 j ，并且在 nums2 确定 nums2[j] 的 下一个更大元素 。如果不存在下一个更大元素，那么本次查询的答案是 -1 。

返回一个长度为 nums1.length 的数组 ans 作为答案，满足 ans[i] 是如上所述的 下一个更大元素 。


```
示例 1：

输入：nums1 = [4,1,2], nums2 = [1,3,4,2].
输出：[-1,3,-1]
解释：nums1 中每个值的下一个更大元素如下所述：
- 4 ，用加粗斜体标识，nums2 = [1,3,4,2]。不存在下一个更大元素，所以答案是 -1 。
- 1 ，用加粗斜体标识，nums2 = [1,3,4,2]。下一个更大元素是 3 。
- 2 ，用加粗斜体标识，nums2 = [1,3,4,2]。不存在下一个更大元素，所以答案是 -1 。
示例 2：

输入：nums1 = [2,4], nums2 = [1,2,3,4].
输出：[3,-1]
解释：nums1 中每个值的下一个更大元素如下所述：
- 2 ，用加粗斜体标识，nums2 = [1,2,3,4]。下一个更大元素是 3 。
- 4 ，用加粗斜体标识，nums2 = [1,2,3,4]。不存在下一个更大元素，所以答案是 -1 。


提示：

1 <= nums1.length <= nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 104
nums1和nums2中所有整数 互不相同
nums1 中的所有整数同样出现在 nums2 中


进阶：你可以设计一个时间复杂度为 O(nums1.length + nums2.length) 的解决方案吗？
```

思路

单调栈解决 Next Greater Number 一类问题

首先，讲解 Next Greater Number 的原始问题：给你一个数组，返回一个等长的数组，对应索引存储着下一个更大元素，如果没有更大的元素，就存 -1。
 */
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 定义结果
    let mut ans = vec![0; nums1.len()];
    // 定义栈，栈中最后存储数组nums2中元素的坐标，而不是元素值本身
    let mut stack = Vec::with_capacity(nums2.len());
    // 定义哈希表来存放元素和比自己大的next元素
    let mut hash = HashMap::new();
    // 反向遍历nums2数组
    for (idx, v) in nums2.iter().enumerate().rev() {
        while !stack.is_empty() && *v >= nums2[*stack.last().unwrap()] {
            // 栈中比 v 小的数的坐标都弹出来
            stack.pop();
        }
        hash.insert(
            v,
            if stack.is_empty() {
                -1
            } else {
                nums2[*stack.last().unwrap()]
            },
        );
        // 把 v 的坐标入栈
        stack.push(idx);
    }
    for (idx, v) in nums1.iter().enumerate() {
        ans[idx] = *hash.get(v).unwrap();
    }
    ans
}

/// 503. 下一个更大元素 II
///
/// https://leetcode.cn/problems/next-greater-element-ii/
///
/// 本题应该用个「单调递减栈」来实现
///
/// 建立「单调递减栈」，并对原数组遍历一次：
///
/// 如果栈为空，则把当前元素放入栈内；
///
/// 如果栈不为空，则需要判断当前元素和栈顶元素的大小：
///
/// 如果当前元素比栈顶元素大：说明当前元素是前面一些元素的「下一个更大元素」，则逐个弹出栈顶元素，直到当前元素比栈顶元素小为止。
///
/// 如果当前元素比栈顶元素小：说明当前元素的「下一个更大元素」与栈顶元素相同，则把当前元素入栈。
///
/// <img src="https://pic.leetcode-cn.com/1614996551-SXYMXC-503.gif" alt="image" style="zoom:50%;" />
///
/// 如何实现循环数组
///
/// 一种实现方式是，把数组复制一份到数组末尾，这样虽然不是严格的循环数组，但是对于本题已经足够了，因为本题对数组最多遍历两次。
///
/// 另一个常见的实现方式是，使用取模运算 % len 可以把下标 i 映射到数组 nums 长度的 0−N 内。
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let old_len = nums.len();
    // 解决环形数组可以把当前数组扩展一倍
    let nums = nums.repeat(2);
    // 定义单调栈,栈中存放nums数组元素的下标
    let mut stack = Vec::with_capacity(nums.len());
    // 定义结果
    let mut ans = vec![0; old_len];
    for (idx, _) in nums.iter().enumerate().rev() {
        while !stack.is_empty() && nums[idx % old_len] >= nums[*stack.last().unwrap()] {
            // 栈中比 v 小的数的坐标都弹出来
            stack.pop();
        }
        ans[idx % old_len] = if stack.is_empty() {
            -1
        } else {
            nums[*stack.last().unwrap()]
        };
        // 把当前元素下标入栈
        stack.push(idx % old_len);
    }
    ans
}

/**
1475. 商品折扣后的最终价格

给你一个数组 prices ，其中 prices[i] 是商店里第 i 件商品的价格。

商店里正在进行促销活动，如果你要买第 i 件商品，那么你可以得到与 prices[j] 相等的折扣，
其中 j 是满足 j > i 且 prices[j] <= prices[i] 的 最小下标 ，如果没有满足条件的 j ，你将没有任何折扣。

请你返回一个数组，数组中第 i 个元素是折扣后你购买商品 i 最终需要支付的价格。


```
示例 1：

输入：prices = [8,4,6,2,3]
输出：[4,2,4,2,3]
解释：
商品 0 的价格为 price[0]=8 ，你将得到 prices[1]=4 的折扣，所以最终价格为 8 - 4 = 4 。
商品 1 的价格为 price[1]=4 ，你将得到 prices[3]=2 的折扣，所以最终价格为 4 - 2 = 2 。
商品 2 的价格为 price[2]=6 ，你将得到 prices[3]=2 的折扣，所以最终价格为 6 - 2 = 4 。
商品 3 和 4 都没有折扣。
示例 2：

输入：prices = [1,2,3,4,5]
输出：[1,2,3,4,5]
解释：在这个例子中，所有商品都没有折扣。
示例 3：

输入：prices = [10,1,1,6]
输出：[9,0,1,6]


提示：

1 <= prices.length <= 500
1 <= prices[i] <= 10^3
```
 */
pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    // 定义一个单调递增栈
    let mut stack = Vec::with_capacity(prices.len());
    // 定义哈希表存储prices数组的下标和对应的折扣（如果prices数组存在重复数据如果存值会出现被覆盖）
    let mut hash = HashMap::new();
    // 反向遍历prices数组
    for (i, v) in prices.iter().enumerate().rev() {
        while !stack.is_empty() && v < stack.last().unwrap() {
            // 把比v大的值统统弹出
            stack.pop();
        }
        hash.insert(
            i,
            if stack.is_empty() {
                0
            } else {
                *stack.last().unwrap()
            },
        );
        // 把 v 入栈
        stack.push(*v);
    }
    // 定义结果
    let mut ans = Vec::with_capacity(prices.len());
    for (i,v) in prices.iter().enumerate() {
        ans.push(v - hash.get(&i).unwrap());
    }
    ans
}

/**
907. 子数组的最小值之和

给定一个整数数组 arr，找到 min(b) 的总和，其中 b 的范围为 arr 的每个（连续）子数组。

由于答案可能很大，因此 返回答案模 10^9 + 7 。

 
```
示例 1：

输入：arr = [3,1,2,4]
输出：17
解释：
子数组为 [3]，[1]，[2]，[4]，[3,1]，[1,2]，[2,4]，[3,1,2]，[1,2,4]，[3,1,2,4]。 
最小值为 3，1，2，4，1，1，2，1，1，1，和为 17。
示例 2：

输入：arr = [11,81,94,43,3]
输出：444
 

提示：

1 <= arr.length <= 3 * 104
1 <= arr[i] <= 3 * 104
```
 */
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    // 为简化代码逻辑，可以在遍历前，往 arr 末尾和栈顶分别加一个 −1，当作哨兵。
    let mut arr=arr;
    arr.push(-1);
    // 定义一个单调栈
    let mut stack = Vec::with_capacity(arr.len());
    // 哨兵
    stack.push(-1);
    // 定义结果
    let mut ans = 0;
    // 遍历arr数组
    for (r,v) in arr.iter().enumerate() {
        while !stack.is_empty()&& *v <= arr[*stack.last().unwrap() as usize] {
            // 弹出栈顶元素，此时的元素右边界
            let i = stack.pop().unwrap() as i32;
            // 计算结果 stack.last().unwrap() 是左边界 L
            // 以 arr[i] 为最小值的子数组的个数为 (i−L)⋅(R−i)，对答案的贡献为 arr[i]⋅(i−L)⋅(R−i)
            ans += arr[i as usize] * (i - stack.last().unwrap()) * (r as i32 - i);
        }
        // 入队
        stack.push(r as i32);
    }
    ans % 1000000007
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_next_greater_element() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        assert_eq!(next_greater_element(nums1, nums2), vec![-1, 3, -1]);
    }

    #[test]
    fn test_next_greater_elements() {
        let nums = vec![1, 2, 1];
        assert_eq!(next_greater_elements(nums), vec![2, -1, 2]);
    }

    #[test]
    fn test_final_prices() {
        let prices = vec![10, 1, 1, 6];
        assert_eq!(final_prices(prices), vec![9, 0, 1, 6]);
    }

    #[test]
    fn test_sum_subarray_mins() {
        let arr = vec![3, 1, 2, 4];
        assert_eq!(sum_subarray_mins(arr), 17);
    }
}
