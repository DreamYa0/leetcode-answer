// 合并区间
// 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

//

// 示例 1：

// 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
// 输出：[[1,6],[8,10],[15,18]]
// 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
// 示例 2：

// 输入：intervals = [[1,4],[4,5]]
// 输出：[[1,5]]
// 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
//

// 提示：

// 1 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 104
// 这个问题可以通过以下步骤解决：

// 首先，按照区间的开始位置对所有区间进行排序。
// 然后，创建一个空的结果数组，并将第一个区间添加到结果数组中。
// 遍历剩余的每个区间，如果当前区间的开始位置小于或等于结果数组中最后一个区间的结束位置，
// 那么将结果数组中最后一个区间的结束位置更新为当前区间的结束位置和最后一个区间的结束位置中的较大值。
// 否则，将当前区间添加到结果数组中。
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut res = vec![intervals[0].clone()];
    for i in 1..intervals.len() {
        if res.last().unwrap()[1] < intervals[i][0] {
            res.push(intervals[i].clone());
        } else {
            let last_interval = res.last_mut().unwrap();
            last_interval[1] = last_interval[1].max(intervals[i][1]);
        }
    }
    res
}

#[test]
fn test_merge() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(
        merge(intervals),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}
