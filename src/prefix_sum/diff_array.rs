/**
 * 1094. 拼车
中等
相关标签
相关企业
提示
车上最初有 capacity 个空座位。车 只能 向一个方向行驶（也就是说，不允许掉头或改变方向）

给定整数 capacity 和一个数组 trips ,  trip[i] = [numPassengersi, fromi, toi] 表示第 i 次旅行有 numPassengersi 乘客，接他们和放他们的位置分别是 fromi 和 toi 。这些位置是从汽车的初始位置向东的公里数。

当且仅当你可以在所有给定的行程中接送所有乘客时，返回 true，否则请返回 false。



示例 1：

输入：trips = [[2,1,5],[3,3,7]], capacity = 4
输出：false
示例 2：

输入：trips = [[2,1,5],[3,3,7]], capacity = 5
输出：true


提示：

1 <= trips.length <= 1000
trips[i].length == 3
1 <= numPassengersi <= 100
0 <= fromi < toi <= 1000
1 <= capacity <= 105
 */
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    // 通过哈希表来统计每个站点应该上多少人和下多少人
    let mut cnt = vec![0; 1001];
    for i in 0..trips.len() {
        let trip = &trips[i];
        // 乘客
        let people = trip[0];
        // 上站
        let from = trip[1];
        // 下站
        let to = trip[2];
        // 记录上站人数
        cnt[from as usize] += people;
        // 记录下站人数
        cnt[to as usize] -= people;
    }
    // 统计某个站点人数最多的时候
    let mut sum: i32 = 0;
    for i in 0..cnt.len() {
        sum += cnt[i];
        if sum > capacity {
            // 如果在某个站点时人数超过了最大空位数
            return false;
        }
    }
    true
}

/**
 * 1109. 航班预订统计
中等
相关标签
相关企业
这里有 n 个航班，它们分别从 1 到 n 进行编号。

有一份航班预订表 bookings ，表中第 i 条预订记录 bookings[i] = [firsti, lasti, seatsi]
意味着在从 firsti 到 lasti （包含 firsti 和 lasti ）的 每个航班 上预订了 seatsi 个座位。

请你返回一个长度为 n 的数组 answer，里面的元素是每个航班预定的座位总数。



示例 1：

输入：bookings = [[1,2,10],[2,3,20],[2,5,25]], n = 5
输出：[10,55,45,25,25]
解释：
航班编号        1   2   3   4   5
预订记录 1 ：   10  10
预订记录 2 ：       20  20
预订记录 3 ：       25  25  25  25
总座位数：      10  55  45  25  25
因此，answer = [10,55,45,25,25]
示例 2：

输入：bookings = [[1,2,10],[2,2,15]], n = 2
输出：[10,25]
解释：
航班编号        1   2
预订记录 1 ：   10  10
预订记录 2 ：       15
总座位数：      10  25
因此，answer = [10,25]


提示：

1 <= n <= 2 * 104
1 <= bookings.length <= 2 * 104
bookings[i].length == 3
1 <= firsti <= lasti <= n
1 <= seatsi <= 104

解题思路

人数  10  +20+25  -10     -20     -25
     0-----1-------2------3--------5
变化 +10   +45    -10     -20     -25

1.换一种思路理解题意，将问题转换为：某公交车共有 n 站，第 i 条记录 bookings[i] = [i, j, k]
表示在 i 站上车 k 人，乘坐到 j 站，在 j+1 站下车，需要按照车站顺序返回每一站车上的人数
2.根据 1 的思路，定义 counter[] 数组记录每站的人数变化，counter[i] 表示第 i+1 站。
遍历 bookings[]：bookings[i] = [i, j, k] 表示在 i 站增加 k 人即 counters[i-1] += k，在 j+1 站减少 k 人即 counters[j] -= k
3.遍历（整理）counter[] 数组，得到每站总人数： 每站的人数为前一站人数加上当前人数变化 counters[i] += counters[i - 1]
 */
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut prefix = vec![0; n as usize + 2];
    let mut res = Vec::new();
    for booking in bookings {
        prefix[booking[0] as usize] += booking[2];
        prefix[booking[1] as usize + 1] -= booking[2];
    }
    for i in 1..=n as usize {
        if i > 1 {
            prefix[i] += prefix[i - 1];
        }
        res.push(prefix[i]);
    }
    res
}

/**
 * 253. 会议室 II
中等
相关标签
相关企业
提示
给你一个会议时间安排的数组 intervals ，每个会议时间都会包括开始和结束的时间 intervals[i] = [starti, endi] ，返回 所需会议室的最小数量 。



示例 1：

输入：intervals = [[0,30],[5,10],[15,20]]
输出：2
示例 2：

输入：intervals = [[7,10],[2,4]]
输出：1


提示：

1 <= intervals.length <= 104
0 <= starti < endi <= 106

解题思路
开会也可以理解成坐公交，都是占用某个资源。就拿题目给的第一组数组来分析。

intervals = [[0,30],[5,10],[15,20]]
第一个人从0上车，从30下车；
第二个人从5上车，10下车。。。

我们的问题转化为最多车上有几个人（也就是最多有多少会议室）。

显然：上车，车上人数+1；下车，车上人数-1 我们把intervals拆解一下

上车：[0, 1], [5, 1], [15, 1]

下车：[10, -1], [20, -1], [30, -1] 然后按照第一个数把上下车排好序

人数  1    2     1     2     1     0
     0----5----10----15----20-----30
变化 +1   +1    -1    +1    -1    -1
最多车上两个人。
 */
pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let mut prefix = Vec::new();
    for interval in intervals {
        prefix.push((interval[0], 1));
        prefix.push((interval[1], -1));
    }
    prefix.sort();
    let mut res = 0;
    let mut sum = 0;
    for (_, num) in prefix {
        sum += num;
        res = res.max(sum);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_pooling() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 4;
        let result = car_pooling(trips, capacity);
        assert_eq!(result, false);
    }

    #[test]
    fn test_corp_flight_bookings() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        let n = 5;
        assert_eq!(corp_flight_bookings(bookings, n), vec![10, 55, 45, 25, 25]);
    }

    #[test]
    fn test_min_meeting_rooms() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms(intervals), 2);
    }
}
