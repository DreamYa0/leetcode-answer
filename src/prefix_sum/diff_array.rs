use std::collections::BTreeMap;

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
    // 定义差分数组
    let mut diff = vec![0; 1001];
    for i in 0..trips.len() {
        let trip = &trips[i];
        // 乘客
        let people = trip[0];
        // 上站
        let from = trip[1];
        // 下站
        let to = trip[2];
        // 记录上站人数
        diff[from as usize] += people;
        // 记录下站人数
        diff[to as usize] -= people;
    }
    // 统计某个站点人数最多的时候
    let mut sum: i32 = 0;
    for i in 0..diff.len() {
        sum += diff[i];
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
    let mut diff = vec![0; n as usize + 2];
    let mut res = Vec::new();
    for booking in bookings {
        diff[booking[0] as usize] += booking[2];
        diff[booking[1] as usize + 1] -= booking[2];
    }
    for i in 1..=n as usize {
        if i > 1 {
            diff[i] += diff[i - 1];
        }
        res.push(diff[i]);
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
    let mut diff = Vec::new();
    for interval in intervals {
        diff.push((interval[0], 1));
        diff.push((interval[1], -1));
    }
    diff.sort();
    let mut res = 0;
    let mut sum = 0;
    for (_, num) in diff {
        sum += num;
        res = res.max(sum);
    }
    res
}

/**
 * 2381. 字母移位 II
中等
相关标签
相关企业
提示
给你一个小写英文字母组成的字符串 s 和一个二维整数数组 shifts ，其中 shifts[i] = [starti, endi, directioni] 。
对于每个 i ，将 s 中从下标 starti 到下标 endi （两者都包含）所有字符都进行移位运算，
如果 directioni = 1 将字符向后移位，如果 directioni = 0 将字符向前移位。

将一个字符 向后 移位的意思是将这个字符用字母表中 下一个 字母替换（字母表视为环绕的，所以 'z' 变成 'a'）。
类似的，将一个字符 向前 移位的意思是将这个字符用字母表中 前一个 字母替换（字母表是环绕的，所以 'a' 变成 'z' ）。

请你返回对 s 进行所有移位操作以后得到的最终字符串。



示例 1：

输入：s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
输出："ace"
解释：首先，将下标从 0 到 1 的字母向前移位，得到 s = "zac" 。
然后，将下标从 1 到 2 的字母向后移位，得到 s = "zbd" 。
最后，将下标从 0 到 2 的字符向后移位，得到 s = "ace" 。
示例 2:

输入：s = "dztz", shifts = [[0,0,0],[1,1,1]]
输出："catz"
解释：首先，将下标从 0 到 0 的字母向前移位，得到 s = "cztz" 。
最后，将下标从 1 到 1 的字符向后移位，得到 s = "catz" 。


提示：

1 <= s.length, shifts.length <= 5 * 104
shifts[i].length == 3
0 <= starti <= endi < s.length
0 <= directioni <= 1
s 只包含小写英文字母。
 */
pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    // 定义差分数组,初始化diff[0]=s[0]-'a'
    let mut diff = vec![s[0] as i32 - 'a' as i32];
    for i in 1..s.len() {
        diff.push(s[i] as i32 - s[i - 1] as i32);
    }
    // 补0防止 shift[1] as usize + 1 越界
    diff.push(0);
    for shift in shifts.iter() {
        // 替代if判断
        let n = shift[2] * 2 - 1;
        diff[shift[0] as usize] += n;
        diff[shift[1] as usize + 1] -= n;
    }
    for i in 0..s.len() {
        diff[i + 1] += diff[i];
        s[i] = char::from_u32(((diff[i] % 26 + 26) % 26) as u32 + 'a' as u32).unwrap();
    }
    s.iter().collect::<String>()
}

/**
 * 2406. 将区间分为最少组数
中等
相关标签
相关企业
提示
给你一个二维整数数组 intervals ，其中 intervals[i] = [lefti, righti] 表示 闭 区间 [lefti, righti] 。

你需要将 intervals 划分为一个或者多个区间 组 ，每个区间 只 属于一个组，且同一个组中任意两个区间 不相交 。

请你返回 最少 需要划分成多少个组。

如果两个区间覆盖的范围有重叠（即至少有一个公共数字），那么我们称这两个区间是 相交 的。比方说区间 [1, 5] 和 [5, 8] 相交。



示例 1：

输入：intervals = [[5,10],[6,8],[1,5],[2,3],[1,10]]
输出：3
解释：我们可以将区间划分为如下的区间组：
- 第 1 组：[1, 5] ，[6, 8] 。
- 第 2 组：[2, 3] ，[5, 10] 。
- 第 3 组：[1, 10] 。
可以证明无法将区间划分为少于 3 个组。
示例 2：

输入：intervals = [[1,3],[5,6],[8,10],[11,13]]
输出：1
解释：所有区间互不相交，所以我们可以把它们全部放在一个组内。


提示：

1 <= intervals.length <= 105
intervals[i].length == 2
1 <= lefti <= righti <= 106
 */
pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    // 因为需要保持差分数组是从小到大有序的，所有这里不能用HashMap
    let mut diff = BTreeMap::new();
    for p in intervals {
        diff.insert(p[0], diff.get(&p[0]).unwrap_or(&0) + 1);
        diff.insert(p[1] + 1, diff.get(&(p[1] + 1)).unwrap_or(&0) - 1);
    }
    let (mut res, mut sum) = (0, 0);
    for (_, v) in diff {
        sum += v;
        res = res.max(sum);
    }
    res
}

/**
 * 2772. 使数组中的所有元素都等于零
中等
相关标签
提示
给你一个下标从 0 开始的整数数组 nums 和一个正整数 k 。

你可以对数组执行下述操作 任意次 ：

从数组中选出长度为 k 的 任一 子数组，并将子数组中每个元素都 减去 1 。
如果你可以使数组中的所有元素都等于 0 ，返回  true ；否则，返回 false 。

子数组 是数组中的一个非空连续元素序列。



示例 1：

输入：nums = [2,2,3,1,1,0], k = 3
输出：true
解释：可以执行下述操作：
- 选出子数组 [2,2,3] ，执行操作后，数组变为 nums = [1,1,2,1,1,0] 。
- 选出子数组 [2,1,1] ，执行操作后，数组变为 nums = [1,1,1,0,0,0] 。
- 选出子数组 [1,1,1] ，执行操作后，数组变为 nums = [0,0,0,0,0,0] 。
示例 2：

输入：nums = [1,3,1,1], k = 2
输出：false
解释：无法使数组中的所有元素等于 0 。


提示：

1 <= k <= nums.length <= 105
0 <= nums[i] <= 106
 */
pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
    // 建单差分数组
    let mut diff = vec![0; nums.len() + 1];
    // 原数组第一个数就是差分数组的第一个数
    diff[0] = nums[0];
    for i in 1..nums.len() {
        // 求差分数组
        diff[i] = nums[i] - nums[i - 1];
    }
    // 计算 [i, i + k -1] 等 k个连续的数同时减一个数 d, 相当于 差分数组的 i 位置 - d, i + k 位置 + d；
    for i in 0..=nums.len() - k as usize {
        if diff[i] == 0 {
            // 如果 diff[i]=0，无需操作，遍历下一个数。
            continue;
        }
        if diff[i] < 0 {
            // 如果 diff[i]<0，由于无法让元素值增大，返回 false
            return false;
        }
        let tmp = diff[i];
        diff[i] -= tmp;
        diff[i + k as usize] += tmp;
    }
    // 判断经过一系列 连续k个位置同时减去一个数，最后整体是否都为0；
    for i in 0..nums.len() {
        if diff[i] != 0 {
            return false;
        }
    }
    true
}

/**
 * 2528. 最大化城市的最小电量
困难
相关标签
相关企业
提示
给你一个下标从 0 开始长度为 n 的整数数组 stations ，其中 stations[i] 表示第 i 座城市的供电站数目。

每个供电站可以在一定 范围 内给所有城市提供电力。换句话说，如果给定的范围是 r ，在城市 i 处的供电站可以给所有满足 |i - j| <= r 且 0 <= i, j <= n - 1 的城市 j 供电。

|x| 表示 x 的 绝对值 。比方说，|7 - 5| = 2 ，|3 - 10| = 7 。
一座城市的 电量 是所有能给它供电的供电站数目。

政府批准了可以额外建造 k 座供电站，你需要决定这些供电站分别应该建在哪里，这些供电站与已经存在的供电站有相同的供电范围。

给你两个整数 r 和 k ，如果以最优策略建造额外的发电站，返回所有城市中，最小电量的最大值是多少。

这 k 座供电站可以建在多个城市。



示例 1：

输入：stations = [1,2,4,5,0], r = 1, k = 2
输出：5
解释：
最优方案之一是把 2 座供电站都建在城市 1 。
每座城市的供电站数目分别为 [1,4,4,5,0] 。
- 城市 0 的供电站数目为 1 + 4 = 5 。
- 城市 1 的供电站数目为 1 + 4 + 4 = 9 。
- 城市 2 的供电站数目为 4 + 4 + 5 = 13 。
- 城市 3 的供电站数目为 5 + 4 = 9 。
- 城市 4 的供电站数目为 5 + 0 = 5 。
供电站数目最少是 5 。
无法得到更优解，所以我们返回 5 。
示例 2：

输入：stations = [4,4,4,4], r = 0, k = 3
输出：4
解释：
无论如何安排，总有一座城市的供电站数目是 4 ，所以最优解是 4 。


提示：

n == stations.length
1 <= n <= 105
0 <= stations[i] <= 105
0 <= r <= n - 1
0 <= k <= 109
 */
pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let len = stations.len();
    // 定义前缀和数组
    let mut prefix = vec![0; len + 1];
    for i in 0..len {
        // 求前缀和
        prefix[i + 1] = prefix[i] + stations[i];
    }
    let mut mn = i32::MAX;
    // 电量
    let mut power = vec![0; len];
    for i in 0..len {
        power[i] = prefix[(i as i32 + r + 1).min(len as i32) as usize] - prefix[(i as i32 - r).max(0) as usize];
        mn = mn.min(power[i]);
    }
    let mut left = mn;
    let mut right = mn + k + 1;
    // (left,right) 左开右开区间
    while left + 1 < right {
        let mid = (left + right) >> 1;
        if check(mid, &power, len, r as usize, k as usize) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left as i64
}

fn check(mid: i32, power: &Vec<i32>, len: usize, r: usize, k: usize) -> bool {
    // 建立差分数组
    let mut diff = vec![0; len + 1];
    let mut sum_d = 0;
    let mut need = 0;
    for i in 0..len {
        // 累加差分值
        sum_d += diff[i];
        let m = mid - power[i] - sum_d;
        // 需要m个供电站
        if m > 0 {
            need += m;
            if need as usize > k {
                // 提前退出这样快一些
                return false;
            }
            sum_d += m;
            if i + r * 2 + 1 < len {
                // 更新差分值
                diff[i + r * 2 + 1] -= m;
            }
        }
    }
    true
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

    #[test]
    fn test_shifting_letters() {
        let s = "abc".to_string();
        let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
        assert_eq!(shifting_letters(s, shifts), "ace");
    }

    #[test]
    fn test_min_groups() {
        let intervals = vec![
            [5, 10].to_vec(),
            [6, 8].to_vec(),
            [1, 5].to_vec(),
            [2, 3].to_vec(),
            [1, 10].to_vec(),
        ];
        assert_eq!(min_groups(intervals), 3)
    }
}
