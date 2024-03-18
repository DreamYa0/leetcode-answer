use std::{
    collections::{HashMap, HashSet},
    vec,
};

/**
 * 46. 全排列
中等
相关标签
相关企业
给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。


```
示例 1：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
示例 2：

输入：nums = [0,1]
输出：[[0,1],[1,0]]
示例 3：

输入：nums = [1]
输出：[[1]]


提示：

1 <= nums.length <= 6
-10 <= nums[i] <= 10
nums 中的所有整数 互不相同
```
 */
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut used = vec![false; nums.len()];
    do_permute(&nums, &mut path, &mut res, &mut used);
    res
}

fn do_permute(nums: &Vec<i32>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
    if path.len() == nums.len() {
        // 说明path已经到了全排列的长度，也就是说已经到叶子节点了
        res.push(path.to_vec());
        return;
    }
    for i in 0..nums.len() {
        if used[i] == true {
            // 说明nums中i位置的元素已经使用过了
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        do_permute(nums, path, res, used);
        // 回溯
        used[i] = false;
        path.pop();
    }
}

/**
 * 47. 全排列 II
中等
相关标签
相关企业
给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。


```
示例 1：

输入：nums = [1,1,2]
输出：
[[1,1,2],
 [1,2,1],
 [2,1,1]]
示例 2：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]


提示：

1 <= nums.length <= 8
-10 <= nums[i] <= 10
```
 */
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut used = vec![false; nums.len()];
    let mut nums = nums;
    // 组合和排列去重之前都需要先排序，子集问题不能排序
    nums.sort();
    do_permute_unique(&nums, &mut path, &mut res, &mut used);
    res
}

fn do_permute_unique(
    nums: &Vec<i32>,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    used: &mut Vec<bool>,
) {
    if path.len() == nums.len() {
        // 说明path已经到了全排列的长度，也就是说已经到叶子节点了
        res.push(path.to_vec());
        return;
    }
    for i in 0..nums.len() {
        // 去重
        if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
            // 树层去重
            continue;
        }
        if used[i] == true {
            // 说明nums中i位置的元素已经使用过了
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        do_permute_unique(nums, path, res, used);
        // 回溯
        used[i] = false;
        path.pop();
    }
}

/**
 * 面试题 08.08. 有重复字符串的排列组合

有重复字符串的排列组合。编写一种方法，计算某字符串的所有排列组合。

```
示例1:

 输入：S = "qqe"
 输出：["eqq","qeq","qqe"]
示例2:

 输入：S = "ab"
 输出：["ab", "ba"]
提示:

字符都是英文字母。
字符串长度在[1, 9]之间。
```
 */
pub fn permutation(s: String) -> Vec<String> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut used = vec![false; s.len()];
    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    do_permutation(&s, &mut path, &mut res, &mut used);
    res
}

fn do_permutation(
    s: &Vec<char>,
    path: &mut Vec<char>,
    res: &mut Vec<String>,
    used: &mut Vec<bool>,
) {
    if path.len() == s.len() {
        res.push(path.iter().collect::<String>());
        return;
    }
    for i in 0..s.len() {
        if i > 0 && s[i] == s[i - 1] && used[i - 1] == false {
            continue;
        }
        if used[i] == true {
            continue;
        }
        used[i] = true;
        path.push(s[i]);
        do_permutation(s, path, res, used);
        // 回溯
        used[i] = false;
        path.pop();
    }
}

/**
 * LCR 157. 套餐内商品的排列顺序

某店铺将用于组成套餐的商品记作字符串 goods，其中 goods[i] 表示对应商品。请返回该套餐内所含商品的 全部排列方式 。

返回结果 无顺序要求，但不能含有重复的元素。


```
示例 1:

输入：goods = "agew"
输出：["aegw","aewg","agew","agwe","aweg","awge","eagw","eawg",
"egaw","egwa","ewag","ewga","gaew","gawe","geaw","gewa","gwae",
"gwea","waeg","wage","weag","wega","wgae","wgea"]


提示：

1 <= goods.length <= 8
```
 */
pub fn goods_order(goods: String) -> Vec<String> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut used = vec![false; goods.len()];
    let mut goods = goods.chars().collect::<Vec<char>>();
    goods.sort();
    do_goods_order(&goods, &mut path, &mut res, &mut used);
    res
}

fn do_goods_order(
    goods: &Vec<char>,
    path: &mut Vec<char>,
    res: &mut Vec<String>,
    used: &mut Vec<bool>,
) {
    if path.len() == goods.len() {
        res.push(path.iter().collect::<String>());
        return;
    }
    for i in 0..goods.len() {
        if i > 0 && goods[i] == goods[i - 1] && used[i - 1] == false {
            continue;
        }
        if used[i] == true {
            continue;
        }
        used[i] = true;
        path.push(goods[i]);
        do_goods_order(goods, path, res, used);
        // 回溯
        used[i] = false;
        path.pop();
    }
}

/**
 * 267. 回文排列 II

给定一个字符串 s ，返回 其重新排列组合后可能构成的所有回文字符串，并去除重复的组合 。

你可以按 任意顺序 返回答案。如果 s 不能形成任何回文排列时，则返回一个空列表。



示例 1：

输入: s = "aabb"
输出: ["abba", "baab"]
示例 2：

输入: s = "abc"
输出: []


提示：

1 <= s.length <= 16
s 仅由小写英文字母组成
 */
pub fn generate_palindromes(s: String) -> Vec<String> {
    let mut m = HashMap::new();
    let vec = s.chars().collect::<Vec<char>>();
    if is_can_palindrome(&vec, &mut m) == false {
        return vec![];
    }
    let mut st = vec![];
    let mut c = None;
    for ch in m.keys() {
        if m.get(ch).unwrap() % 2 == 1 {
            c = Some(ch.to_owned());
        }
        for _ in 0..(m.get(ch).unwrap() / 2) {
            st.push(ch.to_owned());
        }
    }
    let mut s = HashSet::new();
    do_generate_palindromes(&mut st, &mut s, 0, c);
    s.into_iter().collect::<Vec<String>>()
}

fn is_can_palindrome(vec: &Vec<char>, m: &mut HashMap<char, i32>) -> bool {
    let mut count = 0;
    for ch in vec.iter() {
        let entry = m.entry(*ch).or_insert(0);
        *entry += 1;
        if (*entry) % 2 == 0 {
            count -= 1;
        } else {
            count += 1;
        }
    }
    count <= 1
}

fn do_generate_palindromes(
    vec: &mut Vec<char>,
    s: &mut HashSet<String>,
    start: usize,
    ch: Option<char>,
) {
    if start == vec.len() {
        let tmp1 = vec.iter().collect::<String>();
        let tmp2 = vec.iter().rev().collect::<String>();
        if let Some(ch) = ch {
            s.insert(format!("{}{}{}", tmp1, ch, tmp2));
        } else {
            s.insert(format!("{}{}", tmp1, tmp2));
        }
    } else {
        for j in start..vec.len() {
            if vec[j] != vec[start] || start == j {
                vec.swap(start, j);
                do_generate_palindromes(vec, s, start + 1, ch);
                vec.swap(j, start);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let nums = vec![1, 2, 3];
        let res = permute(nums);
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn test_permute_unique() {
        let nums = vec![1, 1, 2];
        let res = permute_unique(nums);
        assert_eq!(res, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]);
    }

    #[test]
    fn test_permutation() {
        let s = "qqe".to_string();
        let res = permutation(s);
        assert_eq!(res, vec!["eqq", "qeq", "qqe"]);
    }

    #[test]
    fn test_goods_order() {
        let goods = "agew".to_string();
        let res = goods_order(goods);
        assert_eq!(
            res,
            vec![
                "aegw", "aewg", "agew", "agwe", "aweg", "awge", "eagw", "eawg", "egaw", "egwa",
                "ewag", "ewga", "gaew", "gawe", "geaw", "gewa", "gwae", "gwea", "waeg", "wage",
                "weag", "wega", "wgae", "wgea"
            ]
        );
    }

    #[test]
    fn test_generate_palindromes() {
        let s = "aabbccddeeffgghh".to_string();
        let res = generate_palindromes(s);
        println!("{:?}", res);
    }
}
