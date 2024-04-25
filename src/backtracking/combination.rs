/**
77. 组合
给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。


```
示例 1：

输入：n = 4, k = 2
输出：
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]
示例 2：

输入：n = 1, k = 1
输出：[[1]]


提示：

1 <= n <= 20
1 <= k <= n
```
 */
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    backtracking(n, k, 1, &mut path, &mut res);
    res
}

fn backtracking(n: i32, k: i32, start: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if path.len() == k as usize {
        // 当path的长度等于k时说明已经到了收集结果的时候了
        res.push(path.to_vec());
        return;
    }
    for i in start..=n - (k - path.len() as i32) + 1 {
        // 处理节点
        path.push(i);
        // 递归
        backtracking(n, k, i + 1, path, res);
        // 回溯，撤销处理的节点
        path.pop();
    }
}

/**
216. 组合总和 III
找出所有相加之和为 n 的 k 个数的组合，且满足下列条件：

只使用数字1到9
每个数字 最多使用一次
返回 所有可能的有效组合的列表 。该列表不能包含相同的组合两次，组合可以以任何顺序返回。

```
示例 1:

输入: k = 3, n = 7
输出: [[1,2,4]]
解释:
1 + 2 + 4 = 7
没有其他符合的组合了。
示例 2:

输入: k = 3, n = 9
输出: [[1,2,6], [1,3,5], [2,3,4]]
解释:
1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
没有其他符合的组合了。
示例 3:

输入: k = 4, n = 1
输出: []
解释: 不存在有效的组合。
在[1,9]范围内使用4个不同的数字，我们可以得到的最小和是1+2+3+4 = 10，因为10 > 1，没有有效的组合。


提示:

2 <= k <= 9
1 <= n <= 60
```
 */
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    backtracking3(n, k, 1, 0, &mut path, &mut res);
    res
}

fn backtracking3(
    n: i32,
    k: i32,
    start: i32,
    sum: i32,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if path.len() == k as usize {
        if sum == n {
            res.push(path.to_vec());
        }
        return;
    }
    for i in start..=9 - (k - path.len() as i32) + 1 {
        // 剪枝操作
        if sum + i > n {
            break;
        }
        path.push(i);
        // 回溯隐藏在sum + i中
        backtracking3(n, k, i + 1, sum + i, path, res);
        path.pop();
    }
}

/**
 * 377. 组合总和 Ⅳ
中等
相关标签
相关企业
给你一个由 不同 整数组成的数组 nums ，和一个目标整数 target 。请你从 nums 中找出并返回总和为 target 的元素组合的个数。

题目数据保证答案符合 32 位整数范围。



示例 1：

输入：nums = [1,2,3], target = 4
输出：7
解释：
所有可能的组合为：
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)
请注意，顺序不同的序列被视作不同的组合。
示例 2：

输入：nums = [9], target = 3
输出：0


提示：

1 <= nums.length <= 200
1 <= nums[i] <= 1000
nums 中的所有元素 互不相同
1 <= target <= 1000


进阶：如果给定的数组中含有负数会发生什么？问题会产生何种变化？如果允许负数出现，需要向题目中添加哪些限制条件？
 */
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    fn dfs(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
        if i == 0 {
            // 爬完了
            return 1;
        }
        if memo[i] != -1 {
            // 之前计算过
            return memo[i];
        }
        let mut res = 0;
        for &x in nums {
            // 枚举所有可以爬的台阶数
            let x = x as usize;
            if x <= i {
                res += dfs(i - x, nums, memo);
            }
        }
        memo[i] = res; // 记忆化
        res
    }
    let t = target as usize;
    let mut memo = vec![-1; t + 1]; // -1 表示没有计算过
    dfs(t, &nums, &mut memo)
}

/**
 * 17. 电话号码的字母组合
中等

给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。

给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。


<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/original_images/17_telephone_keypad.png" style="width: 200px;">

```
示例 1：

输入：digits = "23"
输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
示例 2：

输入：digits = ""
输出：[]
示例 3：

输入：digits = "2"
输出：["a","b","c"]


提示：

0 <= digits.length <= 4
digits[i] 是范围 ['2', '9'] 的一个数字。
```
 */
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    // 定义哈希表来映射数字和字母
    let letter_mapping = vec![
        vec![' '],
        vec![' '],
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];
    let digits = digits.chars().collect::<Vec<char>>();
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_letter_combinations(&digits, 0, &letter_mapping, &mut path, &mut res);
    res.iter().map(|x| x.iter().collect::<String>()).collect()
}

fn do_letter_combinations(
    digits: &Vec<char>,
    index: usize,
    letter_mapping: &Vec<Vec<char>>,
    path: &mut Vec<char>,
    res: &mut Vec<Vec<char>>,
) {
    if index == digits.len() {
        res.push(path.clone());
        return;
    }
    // 遍历到了输入的那个数字
    let digit = digits[index] as usize - '0' as usize;
    // 从哈希表中取对应的字母
    let letters = &letter_mapping[digit];
    for i in 0..letters.len() {
        // 放入结果
        path.push(letters[i]);
        // 递归
        do_letter_combinations(digits, index + 1, letter_mapping, path, res);
        // 回溯
        path.pop();
    }
}

/**
 * 39. 组合总和
给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，
找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。

candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。

对于给定的输入，保证和为 target 的不同组合数少于 150 个。


```
示例 1：

输入：candidates = [2,3,6,7], target = 7
输出：[[2,2,3],[7]]
解释：
2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
7 也是一个候选， 7 = 7 。
仅有这两种组合。
示例 2：

输入: candidates = [2,3,5], target = 8
输出: [[2,2,2,2],[2,3,3],[3,5]]
示例 3：

输入: candidates = [2], target = 1
输出: []


提示：

1 <= candidates.length <= 30
2 <= candidates[i] <= 40
candidates 的所有元素 互不相同
1 <= target <= 40
```
 */
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_combination_sum(&candidates, target, 0, 0, &mut path, &mut res);
    res
}

fn do_combination_sum(
    candidates: &Vec<i32>,
    target: i32,
    sum: i32,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    // 终止条件
    if sum > target {
        return;
    }
    // 终止条件
    if sum == target {
        res.push(path.clone());
        return;
    }
    for i in start..candidates.len() {
        // 剪枝操作，对总集合排序之后，如果下一层的sum（就是本层的 sum + candidates[i]）已经大于target，就可以结束本轮for循环的遍历。
        if sum + candidates[i] <= target {
            // 处理节点
            path.push(candidates[i]);
            // 关键点:不用i+1了，表示可以重复读取当前的数 ，sum的回溯操作隐藏在sum + candidates[i]中
            do_combination_sum(candidates, target, sum + candidates[i], i, path, res);
            // 回溯
            path.pop();
        }
    }
}

/**
 * 40. 组合总和 II
中等
相关标签
相关企业
给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

candidates 中的每个数字在每个组合中只能使用 一次 。

注意：解集不能包含重复的组合。


```
示例 1:

输入: candidates = [10,1,2,7,6,1,5], target = 8,
输出:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]
示例 2:

输入: candidates = [2,5,2,1,2], target = 5,
输出:
[
[1,2,2],
[5]
]


提示:

1 <= candidates.length <= 100
1 <= candidates[i] <= 50
1 <= target <= 30
```
 */
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut candidates = candidates;
    candidates.sort();
    do_combination_sum2(&candidates, target, 0, 0, &mut path, &mut res);
    res
}

fn do_combination_sum2(
    candidates: &Vec<i32>,
    target: i32,
    sum: i32,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    // 终止条件
    if sum > target {
        return;
    }
    // 终止条件
    if sum == target {
        res.push(path.clone());
        return;
    }
    for i in start..candidates.len() {
        // 剪枝操作，对总集合排序之后，如果下一层的sum（就是本层的 sum + candidates[i]）已经大于target，就可以结束本轮for循环的遍历。
        if sum + candidates[i] <= target {
            // 要对同一树层使用过的元素进行跳过
            if i > 0 && candidates[i] == candidates[i - 1] && i > start {
                continue;
            }
            // 处理节点
            path.push(candidates[i]);
            // 关键点:需要i+1了，表示不可以重复读取当前的数 ，sum的回溯操作隐藏在sum + candidates[i]中
            do_combination_sum2(candidates, target, sum + candidates[i], i + 1, path, res);
            // 回溯
            path.pop();
        }
    }
}

/**
 * 22. 括号生成

数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。


```
示例 1：

输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
示例 2：

输入：n = 1
输出：["()"]


提示：

1 <= n <= 8
```
 */
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n <= 0 {
        return vec![];
    }
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_generate_parenthesis(n as usize, n as usize, &mut path, &mut res);
    res
}

fn do_generate_parenthesis(left: usize, right: usize, path: &mut Vec<char>, res: &mut Vec<String>) {
    // 因为每一次尝试，都使用新的字符串变量，所以无需回溯
    if left == 0 && right == 0 {
        // 左括号数和右括号数都为0了，就可以收集结果了
        res.push(path.clone().iter().collect::<String>());
        return;
    }
    // 剪枝（如图，左括号可以使用的个数严格大于右括号可以使用的个数，才剪枝，注意这个细节）
    if left > right {
        return;
    }
    if left > 0 {
        path.push('(');
        do_generate_parenthesis(left - 1, right, path, res);
        // 回溯
        path.pop();
    }
    if right > 0 {
        path.push(')');
        do_generate_parenthesis(left, right - 1, path, res);
        // 回溯
        path.pop();
    }
}

/**
 * 494. 目标和
中等
相关标签
相关企业
给你一个非负整数数组 nums 和一个整数 target 。

向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：

例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。


```
示例 1：

输入：nums = [1,1,1,1,1], target = 3
输出：5
解释：一共有 5 种方法让最终目标和为 3 。
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3
示例 2：

输入：nums = [1], target = 1
输出：1


提示：

1 <= nums.length <= 20
0 <= nums[i] <= 1000
0 <= sum(nums[i]) <= 1000
-1000 <= target <= 1000
```
 */
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = Vec::new();
    do_find_target_sum_ways(&nums, target, 0, 0, &mut res);
    res.iter().count() as i32
}

fn do_find_target_sum_ways(
    nums: &Vec<i32>,
    target: i32,
    start: usize,
    sum: i32,
    res: &mut Vec<i32>,
) {
    if start == nums.len() {
        if target == sum {
            // 遍历到数组末尾后开始收集结果
            res.push(1);
        }
        return;
    }
    // 回溯隐藏在 sum + nums[start] 和 sum - nums[start] 中
    do_find_target_sum_ways(nums, target, start + 1, sum + nums[start], res);
    do_find_target_sum_ways(nums, target, start + 1, sum - nums[start], res);
}

/**
 * 2006. 差的绝对值为 K 的数对数目
简单
相关标签
相关企业
提示
给你一个整数数组 nums 和一个整数 k ，请你返回数对 (i, j) 的数目，满足 i < j 且 |nums[i] - nums[j]| == k 。

|x| 的值定义为：

如果 x >= 0 ，那么值为 x 。
如果 x < 0 ，那么值为 -x 。


示例 1：

输入：nums = [1,2,2,1], k = 1
输出：4
解释：差的绝对值为 1 的数对为：
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
示例 2：

输入：nums = [1,3], k = 3
输出：0
解释：没有任何数对差的绝对值为 3 。
示例 3：

输入：nums = [3,2,1,5,4], k = 2
输出：3
解释：差的绝对值为 2 的数对为：
- [3,2,1,5,4]
- [3,2,1,5,4]
- [3,2,1,5,4]


提示：

1 <= nums.length <= 200
1 <= nums[i] <= 100
1 <= k <= 99
 */
pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = Vec::new();
    let mut path = Vec::new();
    do_count_k_difference(&nums, k, 0, &mut path, &mut res);
    res.iter().count() as i32
}

fn do_count_k_difference(
    nums: &Vec<i32>,
    k: i32,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if path.len() == 2 {
        if (path[0] - path[1]).abs() == k {
            res.push(path.clone());
        }
        return;
    }
    for i in start..nums.len() {
        path.push(nums[i]);
        do_count_k_difference(nums, k, i + 1, path, res);
        path.pop();
    }
}

/**
 * 强化练习 5 ：好数对的数目
给你一个整数数组 nums 。

如果一组数字 (i,j) 满足 nums[i] == nums[j] 且 i < j ，就可以认为这是一组 好数对 。

返回好数对的数目。



示例 1：

输入：nums = [1,2,3,1,1,3]
输出：4
解释：有 4 组好数对，分别是 (0,3), (0,4), (3,4), (2,5) ，下标从 0 开始
示例 2：

输入：nums = [1,1,1,1]
输出：6
解释：数组中的每组数字都是好数对
示例 3：

输入：nums = [1,2,3]
输出：0


提示：

1 <= nums.length <= 100
1 <= nums[i] <= 100
 */
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut res = Vec::new();
    let mut path = Vec::new();
    do_num_identical_pairs(&nums, 0, &mut path, &mut res);
    res.iter().count() as i32
}

fn do_num_identical_pairs(
    nums: &Vec<i32>,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if path.len() == 2 {
        if path[0] == path[1] {
            res.push(path.clone());
        }
        return;
    }
    for i in start..nums.len() {
        path.push(nums[i]);
        do_num_identical_pairs(nums, i + 1, path, res);
        path.pop();
    }
}

/**
 * 强化练习 6 ：大餐计数
大餐 是指 恰好包含两道不同餐品 的一餐，其美味程度之和等于 2 的幂。

你可以搭配 任意 两道餐品做一顿大餐。

给你一个整数数组 deliciousness ，其中 deliciousness[i] 是第 i​​​​​​​​​​​​​​ 道餐品的美味程度，返回你可以用数组中的餐品做出的不同 大餐 的数量。结果需要对 109 + 7 取余。

注意，只要餐品下标不同，就可以认为是不同的餐品，即便它们的美味程度相同。



示例 1：

输入：deliciousness = [1,3,5,7,9]
输出：4
解释：大餐的美味程度组合为 (1,3) 、(1,7) 、(3,5) 和 (7,9) 。
它们各自的美味程度之和分别为 4 、8 、8 和 16 ，都是 2 的幂。
示例 2：

输入：deliciousness = [1,1,1,3,3,3,7]
输出：15
解释：大餐的美味程度组合为 3 种 (1,1) ，9 种 (1,3) ，和 3 种 (1,7) 。


提示：

1 <= deliciousness.length <= 105
0 <= deliciousness[i] <= 220
 */
pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mut res = Vec::new();
    let mut path = Vec::new();
    do_count_pairs(&deliciousness, 0, &mut path, &mut res);
    res.iter().count() as i32
}

fn do_count_pairs(
    deliciousness: &Vec<i32>,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if path.len() == 2 {
        let sum = path[0] + path[1];
        // n & (n - 1)来检查一个数n是否是2的幂
        if sum > 0 && sum & (sum - 1) == 0 {
            res.push(path.to_vec());
        }
        return;
    }
    for i in start..deliciousness.len() {
        path.push(deliciousness[i]);
        do_count_pairs(deliciousness, i + 1, path, res);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let n = 4;
        let k = 2;
        let res = combine(n, k);
        assert_eq!(
            res,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn test_combination_sum3() {
        let k = 3;
        let n = 7;
        let res = combination_sum3(k, n);
        assert_eq!(res, vec![vec![1, 2, 4]]);
    }

    #[test]
    fn test_letter_combinations() {
        let digits = "23".to_string();
        let res = letter_combinations(digits);
        assert_eq!(
            res,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let res = combination_sum(candidates, target);
        assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let res = combination_sum2(candidates, target);
        assert_eq!(
            res,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_generate_parenthesis() {
        let n = 3;
        let res = generate_parenthesis(n);
        assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn test_find_target_sum_ways() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 3;
        let res = find_target_sum_ways(nums, target);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_count_k_difference() {
        let nums = vec![1, 2, 2, 1];
        let k = 1;
        let res = count_k_difference(nums, k);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_count_pairs() {
        let deliciousness = vec![1, 3, 5, 7, 9];
        let res = count_pairs(deliciousness);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_combination_sum4() {
        let nums = vec![1, 2, 3];
        let target = 4;
        let res = combination_sum4(nums, target);
        assert_eq!(res, 7);
    }
}
