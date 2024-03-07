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
        assert_eq!(
            res,
            vec![
                vec![1, 1, 2],
                vec![1, 2, 1],
                vec![2, 1, 1],
            ]
        );
    }
}
