use std::collections::{HashMap, HashSet};

pub mod anagram;
pub mod rolling_hash;

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
        // 重复的不只统计一次，即是出现次数大于2次的元素也只会统计一次
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

pub fn find_pairs_ii(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut cns = HashMap::<i32, i32>::new();
    let mut res = 0;
    if k == 0 {
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                if i > 1 && nums[i] == nums[i - 2] {
                    // 重复的数据忽略掉不重复统计
                    continue;
                }
                res += 1;
            }
        }
    } else {
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                // 重复的直接跳过
                continue;
            }
            if cns.contains_key(&(nums[i])) {
                res += 1;
            }
            cns.insert(nums[i] + k, 1);
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

/**
 * 006. 差的绝对值为 K 的数对数目
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
    // key为nums[i] - k 或 nums[i] + k, value为nums[i]出现的次数 nums[i] - k可能出现负数，所以这里不能使用数组来作哈希表
    let mut cnt = HashMap::new();
    let mut res = 0;
    for i in 0..nums.len() {
        res += cnt.get(&(nums[i] - k)).unwrap_or(&0) + cnt.get(&(nums[i] + k)).unwrap_or(&0);
        cnt.insert(nums[i], cnt.get(&nums[i]).unwrap_or(&0) + 1);
    }
    res
}

/**
 * 强化练习 1 ：唯一元素的和
给你一个整数数组 nums 。数组中唯一元素是那些只出现 恰好一次 的元素。

请你返回 nums 中唯一元素的 和 。



示例 1：

输入：nums = [1,2,3,2]
输出：4
解释：唯一元素为 [1,3] ，和为 4 。
示例 2：

输入：nums = [1,1,1,1,1]
输出：0
解释：没有唯一元素，和为 0 。
示例 3 ：

输入：nums = [1,2,3,4,5]
输出：15
解释：唯一元素为 [1,2,3,4,5] ，和为 15 。


提示：

1 <= nums.length <= 100
1 <= nums[i] <= 100
 */
pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 101];
    for n in nums {
        cnt[n as usize] += 1;
    }
    let mut sum = 0;
    for (i, n) in cnt.iter().enumerate() {
        if *n as usize == 1 {
            sum += i;
        }
    }
    sum as i32
}

/**
 * 强化练习 3 ：检查是否所有字符出现次数相同
给你一个字符串 s ，如果 s 是一个 好 字符串，请你返回 true ，否则请返回 false 。

如果 s 中出现过的 所有 字符的出现次数 相同 ，那么我们称字符串 s 是 好 字符串。



示例 1：

输入：s = "abacbc"
输出：true
解释：s 中出现过的字符为 'a'，'b' 和 'c' 。s 中所有字符均出现 2 次。
示例 2：

输入：s = "aaabb"
输出：false
解释：s 中出现过的字符为 'a' 和 'b' 。
'a' 出现了 3 次，'b' 出现了 2 次，两者出现次数不同。


提示：

1 <= s.length <= 1000
s 只包含小写英文字母。
 */
pub fn are_occurrences_equal(s: String) -> bool {
    let mut cnt = vec![0; 26];
    let s = s.into_bytes();
    for c in s {
        cnt[(c - b'a') as usize] += 1;
    }
    let (mut left, mut right) = (0, cnt.len() - 1);
    while left < right {
        if cnt[left] == 0 {
            left += 1;
        } else if cnt[right] == 0 {
            right -= 1;
        } else if cnt[left] != cnt[right] {
            return false;
        } else {
            left += 1;
            right -= 1;
        }
    }
    true
}

/**
 * 强化练习 3：按照频率将数组升序排序
给你一个整数数组 nums ，请你将数组按照每个值的频率 升序 排序。如果有多个值的频率相同，请你按照数值本身将它们 降序 排序。

请你返回排序后的数组。



示例 1：

输入：nums = [1,1,2,2,2,3]
输出：[3,1,1,2,2,2]
解释：'3' 频率为 1，'1' 频率为 2，'2' 频率为 3 。
示例 2：

输入：nums = [2,3,1,3,2]
输出：[1,3,3,2,2]
解释：'2' 和 '3' 频率都为 2 ，所以它们之间按照数值本身降序排序。
示例 3：

输入：nums = [-1,1,-6,4,5,-6,1,4,1]
输出：[5,-1,4,4,-6,-6,1,1,1]


提示：

1 <= nums.length <= 100
-100 <= nums[i] <= 100
 */
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut cnt = vec![0; 201];
    for n in nums.iter() {
        // 统计频次
        cnt[(*n + 100) as usize] += 1;
    }
    // 比较器排序
    nums.sort_unstable_by(|a, b| {
        if cnt[(*a + 100) as usize] != cnt[(*b + 100) as usize] {
            // 如果频次不相等，那么就按照频次从大到小排
            return cnt[(*a + 100) as usize].cmp(&cnt[(*b + 100) as usize]);
        }
        // 如果频次相同则按照自身数据大小从小到大排
        b.cmp(a)
    });
    nums
}

/**
 * 强化练习 6：设计哈希集合
不使用任何内建的哈希表库设计一个哈希集合（HashSet）。

实现 MyHashSet 类：

void add(key) 向哈希集合中插入值 key 。
bool contains(key) 返回哈希集合中是否存在这个值 key 。
void remove(key) 将给定值 key 从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。

示例：

输入：
["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
[[], [1], [2], [1], [3], [2], [2], [2], [2]]
输出：
[null, null, null, true, false, null, true, null, false]

解释：
MyHashSet myHashSet = new MyHashSet();
myHashSet.add(1);      // set = [1]
myHashSet.add(2);      // set = [1, 2]
myHashSet.contains(1); // 返回 True
myHashSet.contains(3); // 返回 False ，（未找到）
myHashSet.add(2);      // set = [1, 2]
myHashSet.contains(2); // 返回 True
myHashSet.remove(2);   // set = [1]
myHashSet.contains(2); // 返回 False ，（已移除）


提示：

0 <= key <= 106
最多调用 104 次 add、remove 和 contains
 */
#[allow(dead_code)]
struct MyHashSet {
    val: Vec<u64>,
}

#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        Self {
            val: vec![0; 15626],
        }
    }

    fn add(&mut self, key: i32) {
        // 将key对应的位置置为1
        self.val[key as usize / 64] |= 1 << key as usize % 64;
    }

    fn remove(&mut self, key: i32) {
        // 将key对应的位置置为0
        self.val[key as usize / 64] &= !(1 << key as usize % 64);
    }

    fn contains(&self, key: i32) -> bool {
        // 判断key对应的位置是否为1
        self.val[key as usize / 64] & 1 << key as usize % 64 > 0
    }
}

/**
强化练习 7：设计哈希映射
不使用任何内建的哈希表库设计一个哈希映射（HashMap）。

实现 MyHashMap 类：

MyHashMap() 用空映射初始化对象
void put(int key, int value) 向 HashMap 插入一个键值对 (key, value) 。如果 key 已经存在于映射中，则更新其对应的值 value 。
int get(int key) 返回特定的 key 所映射的 value ；如果映射中不包含 key 的映射，返回 -1 。
void remove(key) 如果映射中存在 key 的映射，则移除 key 和它所对应的 value 。


示例：

输入：
["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
输出：
[null, null, null, 1, -1, null, 1, null, -1]

解释：
MyHashMap myHashMap = new MyHashMap();
myHashMap.put(1, 1); // myHashMap 现在为 [[1,1]]
myHashMap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(1);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(3);    // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
myHashMap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
myHashMap.get(2);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
myHashMap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
myHashMap.get(2);    // 返回 -1（未找到），myHashMap 现在为 [[1,1]]


提示：

0 <= key, value <= 106
最多调用 104 次 put、get 和 remove 方法
 */
#[allow(dead_code)]
struct MyHashMap {
    val: Vec<i32>,
}

#[allow(dead_code)]
impl MyHashMap {
    fn new() -> Self {
        Self {
            val: vec![-1; 1000001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.val[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.val[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.val[key as usize] = -1;
    }
}

/**
 * 强化练习 1：缺失的第一个正数

给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。


示例 1：

输入：nums = [1,2,0]
输出：3
解释：范围 [1,2] 中的数字都在数组中。
示例 2：

输入：nums = [3,4,-1,1]
输出：2
解释：1 在数组中，但 2 没有。
示例 3：

输入：nums = [7,8,9,11,12]
输出：1
解释：最小的正数 1 没有出现。


提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
 */
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    for i in 0..nums.len() {
        // 我们可以采取这样的思路：就把 1 这个数放到下标为 0 的位置， 2 这个数放到下标为 1 的位置，按照这种思路整理一遍数组。
        // 然后我们再遍历一次数组，第 1 个遇到的它的值不等于下标的那个数，就是我们要找的缺失的第一个正数。
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[(nums[i] - 1) as usize] != nums[i]
        {
            let a = nums[i] as usize - 1;
            nums.swap(a, i)
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    // 如果[1,N]中都找不到那么就返回N + 1
    nums.len() as i32 + 1
}

/**
 * LCR 120. 寻找文件副本
简单
相关标签
相关企业
设备中存有 n 个文件，文件 id 记于数组 documents。若文件 id 相同，则定义为该文件存在副本。请返回任一存在副本的文件 id。



示例 1：

输入：documents = [2, 5, 3, 0, 5, 0]
输出：0 或 5


提示：

0 ≤ documents[i] ≤ n-1
2 <= n <= 100000
 */
pub fn find_repeat_document(documents: Vec<i32>) -> i32 {
    let mut documents = documents;
    for i in 0..documents.len() {
        while documents[i] != documents[documents[i] as usize] {
            let a = documents[i] as usize;
            documents.swap(a, i);
        }
        // 如果当前位置的索引和值不相等，且和documents[documents[i]位置的值相等，那么就找到了重复的文件
        if documents[i] as usize != i && documents[i] == documents[documents[i] as usize] {
            return documents[i];
        }
    }
    -1
}

/**
 * 448. 找到所有数组中消失的数字
简单
相关标签
相关企业
提示
给你一个含 n 个整数的数组 nums ，其中 nums[i] 在区间 [1, n] 内。
请你找出所有在 [1, n] 范围内但没有出现在 nums 中的数字，并以数组的形式返回结果。



示例 1：

输入：nums = [4,3,2,7,8,2,3,1]
输出：[5,6]
示例 2：

输入：nums = [1,1]
输出：[2]


提示：

n == nums.length
1 <= n <= 105
1 <= nums[i] <= n
进阶：你能在不使用额外空间且时间复杂度为 O(n) 的情况下解决这个问题吗? 你可以假定返回的数组不算在额外空间内。
 */
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        let num = nums[i];
        // 如果nums[i] - 1 位置的数大于0，就把变为负数
        if nums[(num.abs() - 1) as usize] > 0 {
            nums[(num.abs() - 1) as usize] *= -1;
        }
    }

    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        // 没有变为负数的数字就是没出现的数字
        if nums[i] > 0 {
            res.push(i as i32 + 1)
        }
    }

    res
}

/**
 * 1442. 形成两个异或相等数组的三元组数目
给你一个整数数组 arr 。

现需要从数组中取三个下标 i、j 和 k ，其中 (0 <= i < j <= k < arr.length) 。

a 和 b 定义如下：

a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
注意：^ 表示 按位异或 操作。

请返回能够令 a == b 成立的三元组 (i, j , k) 的数目。



示例 1：

输入：arr = [2,3,1,6,7]
输出：4
解释：满足题意的三元组分别是 (0,1,2), (0,2,2), (2,3,4) 以及 (2,4,4)
示例 2：

输入：arr = [1,1,1,1,1]
输出：10
示例 3：

输入：arr = [2,3]
输出：0
示例 4：

输入：arr = [1,3,5,7,9]
输出：3
示例 5：

输入：arr = [7,11,12,9,5,2,7,17,22]
输出：8


提示：

1 <= arr.length <= 300
1 <= arr[i] <= 10^8

解题思路：

<img src="https://pic.leetcode-cn.com/1621275894-bLqcng-384ed430a407e5370c5590b44ee21c9.png" />
 */
pub fn count_triplets(arr: Vec<i32>) -> i32 {
    // 定义异或值数组
    let mut pre_xor = vec![0; arr.len() + 1];
    for i in 0..arr.len() {
        pre_xor[i + 1] = arr[i] ^ pre_xor[i];
    }
    let mut res = 0;
    // 我们知道 a ⊕ a = 0的，由于题目让我们找到满足 a == b 的坐标，那么当 a 等于 b 时满足什么性质?
    // a ⊕ b = 0! 我们就可以得到arr[i] ^...^ arr[j-1]^ arr[j] ^...^ arr[k] = 0。
    // 因此在 i 之前的前缀异或值到 k 时不会变。这是法三的核心！！
    // 因为【i，k】的区间异或值为0，可以得到： preXor[i-1] == preXor[k]
    // 其另一点重点在于在区间 [i, k]内 j 在哪并不重要, 因为无论 j 在哪，i 到 k 的异或值都等于 0. 不影响结果。
    for i in 1..=arr.len() {
        for k in i + 1..=arr.len() {
            if pre_xor[i - 1] == pre_xor[k] {
                res += (k - i) as i32;
            }
        }
    }
    res
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

    #[test]
    fn test_count_k_difference() {
        let nums = vec![1, 3];
        let k = 3;
        let result = count_k_difference(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_are_occurrences_equal() {
        let s = "tveixwaeoezcf".to_string();
        let result = are_occurrences_equal(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_first_missing_positive() {
        let nums = vec![3, 4, -1, 1];
        let result = first_missing_positive(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_repeat_document() {
        let documents = vec![0, 1, 2, 3, 4, 11, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let result = find_repeat_document(documents);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_find_pairs_ii() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        let result = find_pairs_ii(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_triplets() {
        let arr = vec![2, 3, 1, 6, 7];
        let result = count_triplets(arr);
        assert_eq!(result, 4);
    }
}
