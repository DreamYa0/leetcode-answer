use std::collections::{HashMap, HashSet};

pub mod anagram;

/// 217. 存在重复元素
///
/// 给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。
///
/// 示例 1：
///
/// 输入：nums = [1,2,3,1]
///
/// 输出：true
///
/// 示例 2：
///
/// 输入：nums = [1,2,3,4]
///
/// 输出：false
///
/// 示例 3：
///
/// 输入：nums = [1,1,1,3,3,4,3,2,4,2]
///
/// 输出：true
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -109 <= nums[i] <= 109
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // 定义hash表用来统计数组中元素出现的次数
    let mut cns = HashMap::with_capacity(220);
    // 遍历数组
    for i in 0..nums.len() {
        // 如果元素出现次数大于2，返回true
        if *cns.get(&nums[i as usize]).or(Some(&0)).unwrap() >= 1 {
            return true;
        }
        // 否则，将元素出现次数加1
        cns.entry(nums[i as usize])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    false
}

/// 128. 最长连续序列
/// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
///
/// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
///
/// 示例 1：
///
/// 输入：nums = [100,4,200,1,3,2]
/// 输出：4
/// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
/// 示例 2：
///
/// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
/// 输出：9
///
/// 提示：
///
/// 0 <= nums.length <= 105
/// -109 <= nums[i] <= 109
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut nums = nums;
    nums.sort_unstable();
    // 定义最长连续序列长度
    let mut max_len = 0;
    // 定义当前连续序列长度
    let mut cur_len = 1;
    // 遍历数组
    for i in 1..nums.len() {
        // 如果当前元素和前一个元素相等，跳过
        if nums[i] == nums[i - 1] {
            continue;
        }
        // 如果当前元素和前一个元素相差1，当前连续序列长度加1
        if nums[i] == nums[i - 1] + 1 {
            cur_len += 1;
        } else {
            // 否则，更新最长连续序列长度
            max_len = max_len.max(cur_len);
            // 重置当前连续序列长度
            cur_len = 1;
        }
    }
    max_len.max(cur_len)
}

/// 290. 单词规律
///
/// 给定一种规律 pattern 和一个字符串 s ，判断 s 是否遵循相同的规律。
///
/// 这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 s 中的每个非空单词之间存在着双向连接的对应规律。
///
/// 示例1:
///
/// 输入: pattern = "abba", s = "dog cat cat dog"
///
/// 输出: true
///
/// 示例 2:
///
/// 输入:pattern = "abba", s = "dog cat cat fish"
///
/// 输出: false
///
/// 示例 3:
///
/// 输入: pattern = "aaaa", s = "dog cat cat dog"
///
/// 输出: false
///
/// 提示:
///
/// 1 <= pattern.length <= 300
///
/// pattern 只包含小写英文字母
///
/// 1 <= s.length <= 3000
///
/// s 只包含小写英文字母和 ' '
///
/// s 不包含 任何前导或尾随对空格
///
/// s 中每个单词都被 单个空格 分隔
///
/// 题目分析
///
/// 这道题是205.同构字符串的升级版，由字符之间的一一映射升级成了字符与字符串之间的一一映射。
///
/// 首先本质是一样的，要实现一一映射，就要用到两个哈希表分别记录字符到字符串的映射和字符串到字符的映射。
///
/// 其次，我们要对s中的单词进行提取，比较单词数量和pattern中的数量是否一致，如果数量上不一致，二者一定不匹配；
///
/// <img src="https://pic.leetcode.cn/1690248452-xPZuJG-image.png" />
pub fn word_pattern(pattern: String, s: String) -> bool {
    // 定义两个哈希表，分别用来记录字符到字符串的映射和字符串到字符的映射
    let mut p2s = HashMap::<char, String>::new();
    let mut s2p = HashMap::<String, char>::new();
    // 将字符串s按空格分割成单词
    let s: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != s.len() {
        // 如果单词数量和pattern中的数量不一致，返回false
        return false;
    }
    // 遍历pattern
    for i in 0..pattern.len() {
        // 如果字符到字符串的映射和字符串到字符的映射不一致，返回false
        if p2s
            .get(&pattern.chars().nth(i).unwrap())
            .unwrap_or(&s[i].to_string())
            != &s[i].to_string()
            || s2p.get(s[i]).unwrap_or(&pattern.chars().nth(i).unwrap())
                != &pattern.chars().nth(i).unwrap()
        {
            return false;
        }
        // 更新字符到字符串的映射和字符串到字符的映射
        p2s.insert(pattern.chars().nth(i).unwrap(), s[i].to_string());
        s2p.insert(s[i].to_string(), pattern.chars().nth(i).unwrap());
    }
    return true;
}

/// 532. 数组中的 k-diff 数对
///
/// 给你一个整数数组 nums 和一个整数 k，请你在数组中找出 不同的 k-diff 数对，并返回不同的 k-diff 数对 的数目。
///
/// k-diff 数对定义为一个整数对 (nums[i], nums[j]) ，并满足下述全部条件：
///
/// 0 <= i, j < nums.length
///
/// i != j
///
/// |nums[i] - nums[j]| == k
///
/// 注意，|val| 表示 val 的绝对值。
///
/// 示例 1：
///
/// 输入：nums = [3, 1, 4, 1, 5], k = 2
///
/// 输出：2
///
/// 解释：数组中有两个 2-diff 数对, (1, 3) 和 (3, 5)。
///
/// 尽管数组中有两个 1 ，但我们只应返回不同的数对的数量。
///
/// 示例 2：
///
/// 输入：nums = [1, 2, 3, 4, 5], k = 1
///
/// 输出：4
///
/// 解释：数组中有四个 1-diff 数对, (1, 2), (2, 3), (3, 4) 和 (4, 5) 。
///
/// 示例 3：
///
/// 输入：nums = [1, 3, 1, 5, 4], k = 0
///
/// 输出：1
///
/// 解释：数组中只有一个 0-diff 数对，(1, 1) 。
///
/// 提示：
///
/// 1 <= nums.length <= 104
///
/// -107 <= nums[i] <= 107
///
/// 0 <= k <= 107
pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    // 定义哈希表用来记录数组中元素出现的次数,数组中的元素分布比较广，所以这个地方不适合用数组来记录元素出现的次数
    let mut cns = HashMap::<i32, i32>::new();
    // 遍历数组
    for i in 0..nums.len() {
        // 更新元素出现的次数
        cns.entry(nums[i]).and_modify(|e| *e += 1).or_insert(1);
    }
    // 定义结果
    let mut res = 0;
    // 遍历数组
    for i in 0..nums.len() {
        // 如果k为0，只需要统计数组中元素出现次数大于1的元素个数
        if k == 0 {
            if *cns.get(&nums[i]).unwrap_or(&0) > 1 {
                res += 1;
                // 更新元素出现的次数, 避免重复统计
                cns.insert(nums[i] + k, 0);
            }
        } else {
            // 否则，统计数组中元素加k后的元素个数
            if *cns.get(&(nums[i] + k)).unwrap_or(&0) > 0 {
                res += 1;
                // 更新元素出现的次数, 避免重复统计
                cns.insert(nums[i] + k, 0);
            }
        }
    }
    res
}

/// 205. 同构字符串
///
/// 给定两个字符串 s 和 t ，判断它们是否是同构的。
///
/// 如果 s 中的字符可以按某种映射关系替换得到 t ，那么这两个字符串是同构的。
///
/// 每个出现的字符都应当映射到另一个字符，同时不改变字符的顺序。不同字符不能映射到同一个字符上，相同字符只能映射到同一个字符上，字符可以映射到自己本身。
///
/// 示例 1:
///
/// 输入：s = "egg", t = "add"
///
/// 输出：true
///
/// 示例 2：
///
/// 输入：s = "foo", t = "bar"
///
/// 输出：false
///
/// 示例 3：
///
/// 输入：s = "paper", t = "title"
///
/// 输出：true
///
/// 提示：
///
/// 1 <= s.length <= 5 * 104
///
/// t.length == s.length
///
/// s 和 t 由任意有效的 ASCII 字符组成
///
/// 解题思路
///
/// 首先复习一下数学中映射的相关概念定义。设集合 s , t 中的某字符为 x , y ，
///
/// 单射：对于任意 x ，都有唯一的 y 与之对应。
///
/// 满射：对于任意 y ，至少存在一个 x 与之对应。
///
/// 双射：既是单射又是满射，又称为一一对应。
///
/// <img src="https://pic.leetcode-cn.com/1656945936-BsSBMu-Slide1.png" />
///
/// 接下来，抽象理解题目给定条件，
///
/// “每个出现的字符都应当映射到另一个字符”。代表字符集合 s , t 之间是「满射」。
///
/// “相同字符只能映射到同一个字符上，不同字符不能映射到同一个字符上”。代表字符集合 s , t 之间是「单射」。
///
/// 因此， s 和 t 之间是「双射」，满足一一对应。考虑遍历字符串，使用哈希表 s2t , t2s 分别记录 s→t t→s 的映射，
/// 当发现任意「一对多」的关系时返回 false 即可。
pub fn is_isomorphic(s: String, t: String) -> bool {
    // 定义两个hash表来存储两个字符串中字符的映射关系
    let mut s2t = HashMap::<char, char>::new();
    let mut t2s = HashMap::<char, char>::new();
    if s.len() != t.len() {
        // 如果两个字符串长度不一致，返回false
        return false;
    }
    // t字符串对应的字符数组
    let t_array = t.chars().collect::<Vec<char>>();
    let s_array = s.chars().collect::<Vec<char>>();
    // 遍历字符串s
    for i in 0..s.len() {
        if s2t.get(&s_array[i]).unwrap_or(&t_array[i]) != &t_array[i]
            || t2s.get(&t_array[i]).unwrap_or(&s_array[i]) != &s.chars().nth(i).unwrap()
        {
            return false;
        }
        s2t.insert(s_array[i], t_array[i]);
        t2s.insert(t_array[i], s_array[i]);
    }
    true
}

/// 202.快乐数
/// 编写一个算法来判断一个数 n 是不是快乐数。
///
/// 「快乐数」 定义为：
///
/// 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
/// 然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
/// 如果这个过程 结果为 1，那么这个数就是快乐数。
/// 如果 n 是 快乐数 就返回 true ；不是，则返回 false 。
///
///
/// 示例 1：
///
/// 输入：n = 19
/// 输出：true
/// 解释：
/// 12 + 92 = 82
/// 82 + 22 = 68
/// 62 + 82 = 100
/// 12 + 02 + 02 = 1
/// 示例 2：
///
/// 输入：n = 2
/// 输出：false
///
/// 提示：
///
/// 1 <= n <= 231 - 1
///
/// 思路
/// 这道题目看上去貌似一道数学问题，其实并不是！
///
/// 题目中说了会 无限循环，那么也就是说求和的过程中，sum会重复出现，这对解题很重要！
///
/// 正如：关于哈希表，你该了解这些！ (opens new window)中所说，当我们遇到了要快速判断一个元素是否出现集合里的时候，就要考虑哈希法了。
///
/// 所以这道题目使用哈希法，来判断这个sum是否重复出现，如果重复了就是return false， 否则一直找到sum为1为止。
///
/// 判断sum是否重复出现就可以使用unordered_set。
///
/// 还有一个难点就是求和的过程，如果对取数值各个位上的单数操作不熟悉的话，做这道题也会比较艰难。
pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut hash_set = HashSet::new();
    loop {
        let sum = get_sum(n);
        if sum == 1 {
            return true;
        }
        if hash_set.contains(&sum) {
            // 如果和出现过了说明已经进入死循环了，需要马上退出
            return false;
        } else {
            hash_set.insert(sum);
        }
        n = sum;
    }
}

pub fn get_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10) * (n % 10);
        n /= 10;
    }
    sum
}

/// 1.两数之和
/// 
/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
/// 
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 
/// 你可以按任意顺序返回答案。
/// 
/// 示例 1：
/// 
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
/// 
/// 示例 2：
/// 
/// 输入：nums = [3,2,4], target = 6
/// 输出：[1,2]
/// 
/// 示例 3：
/// 
/// 输入：nums = [3,3], target = 6
/// 输出：[0,1]
/// 
/// 提示：
/// 
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 
/// 只会存在一个有效答案
/// 
/// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
/// 
/// 思路
/// 很明显暴力的解法是两层for循环查找，时间复杂度是O(n^2)。
///
/// 建议大家做这道题目之前，先做一下这两道
///
/// 242. 有效的字母异位词(opens new window)
/// 349. 两个数组的交集(opens new window)
/// 242. 有效的字母异位词 (opens new window)这道题目是用数组作为哈希表来解决哈希问题，349. 两个数组的交集 (opens new window)这道题目是通过set作为哈希表来解决哈希问题。
///
/// 首先我再强调一下 什么时候使用哈希法，当我们需要查询一个元素是否出现过，或者一个元素是否在集合里的时候，就要第一时间想到哈希法。
///
/// 本题呢，我就需要一个集合来存放我们遍历过的元素，然后在遍历数组的时候去询问这个集合，某元素是否遍历过，也就是 是否出现在这个集合。
///
/// 那么我们就应该想到使用哈希法了。
///
/// 因为本题，我们不仅要知道元素有没有遍历过，还要知道这个元素对应的下标，需要使用 key value结构来存放，key来存元素，value来存下标，那么使用map正合适。
///
/// 再来看一下使用数组和set来做哈希法的局限。
///
/// 数组的大小是受限制的，而且如果元素很少，而哈希值太大会造成内存空间的浪费。
/// set是一个集合，里面放的元素只能是一个key，而两数之和这道题目，不仅要判断y是否存在而且还要记录y的下标位置，因为要返回x 和 y的下标。所以set 也不能用。
/// 此时就要选择另一种数据结构：map ，map是一种key value的存储结构，可以用key保存数值，用value再保存数值所在的下标。
///
/// C++中map，有三种类型：
///
/// ```
/// 映射	底层实现	是否有序	数值是否可以重复	能否更改数值	查询效率	增删效率
/// std::map	红黑树	key有序	key不可重复	key不可修改	O(log n)	O(log n)
/// std::multimap	红黑树	key有序	key可重复	key不可修改	O(log n)	O(log n)
/// std::unordered_map	哈希表	key无序	key不可重复	key不可修改	O(1)	O(1)
/// std::unordered_map 底层实现为哈希表，std::map 和std::multimap 的底层实现是红黑树。
/// ```
///
/// 同理，std::map 和std::multimap 的key也是有序的（这个问题也经常作为面试题，考察对语言容器底层的理解）。 更多哈希表的理论知识请看关于哈希表，你该了解这些！ (opens new window)。
///
/// 这道题目中并不需要key有序，选择std::unordered_map 效率更高！ 使用其他语言的录友注意了解一下自己所用语言的数据结构就行。
///
/// 接下来需要明确两点：
///
/// map用来做什么
/// map中key和value分别表示什么
/// map目的用来存放我们访问过的元素，因为遍历数组的时候，需要记录我们之前遍历过哪些元素和对应的下标，这样才能找到与当前元素相匹配的（也就是相加等于target）
///
/// 接下来是map中key和value分别表示什么。
///
/// 这道题 我们需要 给出一个元素，判断这个元素是否出现过，如果出现过，返回这个元素的下标。
///
/// 那么判断元素是否出现，这个元素就要作为key，所以数组中的元素作为key，有key对应的就是value，value用来存下标。
///
/// 所以 map中的存储结构为 {key：数据元素，value：数组元素对应的下标}。
///
/// 在遍历数组的时候，只需要向map去查询是否有和目前遍历元素匹配的数值，如果有，就找到的匹配对，如果没有，就把目前遍历的元素放进map中，因为map存放的就是我们访问过的元素。
///
/// 过程如下：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20220711202638.png" />
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20230220223536.png" />
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, val) in nums.iter().enumerate() {
        match map.get(&(target - val)) {
            // i为原本map中的数据索引，index为新加入的数据索引
            Some(&i) => return vec![i as i32, index as i32],
            // 不存在则把数据的值作为key，索引作为value存入map
            None => map.insert(val, index),
        };
    }
    // 返回空数组
    vec![]
}

/// 第454题.四数相加II
/// 给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l) 能满足：
///
/// 0 <= i, j, k, l < n
/// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
///
/// 示例 1：
///
/// 输入：nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
/// 输出：2
/// 解释：
/// 两个元组如下：
/// 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
/// 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
/// 
/// 示例 2：
///
/// 输入：nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
/// 输出：1
///
///   提示：
///
/// n == nums1.length
/// n == nums2.length
/// n == nums3.length
/// n == nums4.length
/// 1 <= n <= 200
/// -228 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 228
/// 
/// 思路
/// 本题咋眼一看好像和0015.三数之和，0018.四数之和差不多，其实差很多。
///
/// 本题是使用哈希法的经典题目，而0015.三数之和，0018.四数之和并不合适使用哈希法，因为三数之和和四数之和这两道题目使用哈希法在不超时的情况下做到对结果去重是很困难的，很有多细节需要处理。
///
/// 而这道题目是四个独立的数组，只要找到A[i] + B[j] + C[k] + D[l] = 0就可以，不用考虑有重复的四个元素相加等于0的情况，所以相对于题目18. 四数之和，题目15.三数之和，还是简单了不少！
///
/// 如果本题想难度升级：就是给出一个数组（而不是四个数组），在这里找出四个元素相加等于0，答案中不可以包含重复的四元组，大家可以思考一下，后续的文章我也会讲到的。
///
/// 本题解题步骤：
///
/// 首先定义 一个unordered_map，key放a和b两数之和，value 放a和b两数之和出现的次数。
/// 遍历大A和大B数组，统计两个数组元素之和，和出现的次数，放到map中。
/// 定义int变量count，用来统计a+b+c+d = 0 出现的次数。
/// 在遍历大C和大D数组，找到如果 0-(c+d) 在map中出现过的话，就用count把map中key对应的value也就是出现次数统计出来。
/// 最后返回统计值 count 就可以了
pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    for ele in nums1 {
        // 遍历nums1和nums2，把两个数之和储存到map中，重复出现的增加计数
        for ele2 in &nums2 {
            map.insert(ele + ele2, map.get(&(ele + ele2)).unwrap_or(&0) + 1);
        }
    }

    for ele in nums3 {
        for ele2 in &nums4 {
            if let Some(&val) = map.get(&-(ele + ele2)) {
                count += val;
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn test_longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_find_pairs() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        assert_eq!(find_pairs(nums, k), 2);
    }

    #[test]
    fn test_is_isomorphic() {
        let s = "egg".to_string();
        let t = "add".to_string();
        assert_eq!(is_isomorphic(s, t), true);
    }

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let two_sum = two_sum(nums, target);
        println!("{:?}", two_sum)
    }

    #[test]
    fn test_four_sum_count() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        let result = four_sum_count(nums1, nums2, nums3, nums4);
        println!("result = {:?}", result);
    }
}
