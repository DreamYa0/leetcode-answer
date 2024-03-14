/**
 * 强化练习 2：子数组异或查询
有一个正整数数组 arr，现给你一个对应的查询数组 queries，其中 queries[i] = [Li, Ri]。

对于每个查询 i，请你计算从 Li 到 Ri 的 XOR 值（即 arr[Li] xor arr[Li+1] xor ... xor arr[Ri]）作为本次查询的结果。

并返回一个包含给定查询 queries 所有结果的数组。



示例 1：

输入：arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
输出：[2,7,14,8]
解释：
数组中元素的二进制表示形式是：
1 = 0001
3 = 0011
4 = 0100
8 = 1000
查询的 XOR 值为：
[0,1] = 1 xor 3 = 2
[1,2] = 3 xor 4 = 7
[0,3] = 1 xor 3 xor 4 xor 8 = 14
[3,3] = 8
示例 2：

输入：arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
输出：[8,0,4,4]


提示：

1 <= arr.length <= 3 * 10^4
1 <= arr[i] <= 10^9
1 <= queries.length <= 3 * 10^4
queries[i].length == 2
0 <= queries[i][0] <= queries[i][1] < arr.length
 */
pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix = vec![0; arr.len() + 1];
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] ^ arr[i];
    }
    let mut res = vec![];
    for query in queries {
        // prefix[l] ^ prefix[r + 1]
        res.push(prefix[query[0] as usize] ^ prefix[query[1] as usize + 1]);
    }
    res
}

/**
 * 强化练习 3：每个查询的最大异或值
给你一个 有序 数组 nums ，它由 n 个非负整数组成，同时给你一个整数 maximumBit 。你需要执行以下查询 n 次：

找到一个非负整数 k < 2^maximumBit ，使得 nums[0] XOR nums[1] XOR ... XOR nums[nums.length-1] XOR k 的结果 最大化 。
k 是第 i 个查询的答案。
从当前数组 nums 删除 最后 一个元素。
请你返回一个数组 answer ，其中 answer[i]是第 i 个查询的结果。



示例 1：

输入：nums = [0,1,1,3], maximumBit = 2
输出：[0,3,2,3]
解释：查询的答案如下：
第一个查询：nums = [0,1,1,3]，k = 0，因为 0 XOR 1 XOR 1 XOR 3 XOR 0 = 3 。
第二个查询：nums = [0,1,1]，k = 3，因为 0 XOR 1 XOR 1 XOR 3 = 3 。
第三个查询：nums = [0,1]，k = 2，因为 0 XOR 1 XOR 2 = 3 。
第四个查询：nums = [0]，k = 3，因为 0 XOR 3 = 3 。
示例 2：

输入：nums = [2,3,4,7], maximumBit = 3
输出：[5,2,6,5]
解释：查询的答案如下：
第一个查询：nums = [2,3,4,7]，k = 5，因为 2 XOR 3 XOR 4 XOR 7 XOR 5 = 7。
第二个查询：nums = [2,3,4]，k = 2，因为 2 XOR 3 XOR 4 XOR 2 = 7 。
第三个查询：nums = [2,3]，k = 6，因为 2 XOR 3 XOR 6 = 7 。
第四个查询：nums = [2]，k = 5，因为 2 XOR 5 = 7 。
示例 3：

输入：nums = [0,1,2,2,5,7], maximumBit = 3
输出：[4,3,6,4,6,7]


提示：

nums.length == n
1 <= n <= 105
1 <= maximumBit <= 20
0 <= nums[i] < 2maximumBit
nums​​​ 中的数字已经按 升序 排好序。
 */
pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut ans = vec![];
    let mut xor_sum = 0;
    let mask = (1 << maximum_bit) - 1;

    for num in nums {
        xor_sum ^= num;
        ans.push(xor_sum ^ mask);
    }
    ans.reverse();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_queries() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        assert_eq!(xor_queries(arr, queries), vec![2, 7, 14, 8]);
    }

    #[test]
    fn test_get_maximum_xor() {
        let nums = vec![0, 1, 2, 2, 5, 7];
        let maximum_bit = 3;
        assert_eq!(get_maximum_xor(nums, maximum_bit), vec![4, 3, 6, 4, 6, 7]);
    }
}