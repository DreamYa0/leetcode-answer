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
}
