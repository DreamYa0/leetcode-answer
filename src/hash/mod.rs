use std::collections::{HashMap, HashSet};

pub mod cyclic_hash;
pub mod design_hash;
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

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let (n1, n2) = (nums1.len(), nums2.len());
    let (mut index1, mut index2) = (0__usize, 0__usize);
    while index1 != n1 && index2 != n2 {
        if nums1[index1][0] == nums2[index2][0] {
            res.push(vec![nums1[index1][0], nums1[index1][1] + nums2[index2][1]]);
            index1 += 1;
            index2 += 1;
        } else if nums1[index1][0] < nums2[index2][0] {
            res.push(vec![nums1[index1][0], nums1[index1][1]]);
            index1 += 1;
        } else {
            res.push(vec![nums2[index2][0], nums2[index2][1]]);
            index2 += 1;
        }
    }
    while index1 != n1 {
        res.push(vec![nums1[index1][0], nums1[index1][1]]);
        index1 += 1;
    }
    while index2 != n2 {
        res.push(vec![nums2[index2][0], nums2[index2][1]]);
        index2 += 1;
    }
    res
}

/// 219. 存在重复元素 II
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    // 定义哈希表,k为值，v为值对应的数组下标
    let mut hash_map = HashMap::new();
    // 遍历数组
    for (idx, v) in nums.iter().enumerate() {
        if (idx as i32).abs_diff(*hash_map.get(v).or(Some(&i32::MAX)).unwrap()) as i32 <= k {
            return true;
        } else {
            hash_map.insert(v, idx as i32);
        }
    }
    false
}

/// 1118. 一月有多少天
///
/// 预备知识
///
/// 闰年是公历中的名词。闰年分为普通闰年和世纪闰年，闰年的定义：
///
/// 普通闰年：公历年份是 4 的倍数，且不是 100 的倍数的，为闰年（如 2004 年就是闰年）。
///
/// 世纪闰年：公历年份是整百数的，必须是 400 的倍数才是世纪闰年（如 1900 年不是世纪闰年，2000 年是世纪闰年）。
///
/// 闰年是为了弥补因人为历法规定造成的年度天数与地球实际公转周期的时间差而设立的。补上时间差的年份为闰年。
/// 闰年共有 366 天（ 1-12 月分别为 31 天，29 天，31 天，30 天，31 天，30 天，31 天，31 天，30 天，31 天，30 天，31 天）。
///
/// [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  闰年 2 月有 29 天
///
/// [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  非闰年 2 月有 28 天
pub fn number_of_days(year: i32, month: i32) -> i32 {
    // 闰年
    let year_one = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 非闰年
    let year_two = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        year_one[month as usize - 1]
    } else {
        year_two[month as usize - 1]
    }
}

/// 349.两个数组的交集
/// 给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。
///
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,2,1], nums2 = [2,2]
/// 输出：[2]
/// 示例 2：
///
/// 输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// 输出：[9,4]
/// 解释：[4,9] 也是可通过的
///
/// 提示：
///
/// 1 <= nums1.length, nums2.length <= 1000
/// 0 <= nums1[i], nums2[i] <= 1000
///
/// 思路
/// 由于nums1和nums2的长度<=1000 且 数据大小范围是 0-1000，分布也不算太广所以可以使用数组这种哈希结构，否则需要使用Set这种数据结构
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 创建一个临时数组，由于nums1和nums2中最大数<=1000,所以创建一个1005大小的数组足够用
    let mut temp = [0; 1005];
    if nums1.len() < nums2.len() {
        // 如果发现nums1大小小于nums2，就交换一下他们
        let temp_nums = nums1;
        nums1 = nums2;
        nums2 = temp_nums;
    }

    for num in nums1.iter() {
        // nums1中的数据在数组中记录为1，重复的数字记录一次就可以了
        temp[*num as usize] = 1;
    }

    let mut set = std::collections::HashSet::with_capacity(nums1.len());
    for num in nums2 {
        if temp[num as usize] == 1 {
            set.insert(num);
        }
    }

    set.into_iter().collect()
}

/// 给你一个长度为 n 的整数数组 nums ，请你判断在 最多 改变 1 个元素的情况下，该数组能否变成一个非递减数列。
///
/// 我们是这样定义一个非递减数列的： 对于数组中任意的 i (0 <= i <= n-2)，总满足 nums[i] <= nums[i + 1]。
///
/// 示例 1:
/// 输入: nums = [4,2,3]
/// 输出: true
/// 解释: 你可以通过把第一个 4 变成 1 来使得它成为一个非递减数列。
///
/// 示例 2:
/// 输入: nums = [4,2,1]
/// 输出: false
/// 解释: 你不能在只改变一个元素的情况下将其变为非递减数列。
///
/// 提示：
/// n == nums.length
/// 1 <= n <= 104
/// -105 <= nums[i] <= 105
///
/// 这道题给了我们一个数组，说我们最多有1次修改某个数字的机会，
///   问能不能将数组变为非递减数组。题目中给的例子太少，不能覆盖所有情况，我们再来看下面三个例子：
/// 	4，2，3
/// 	-1，4，2，3
/// 	2，3，3，2，4
///
/// 我们通过分析上面三个例子可以发现，当我们发现后面的数字小于前面的数字产生冲突后，
/// [1]有时候需要修改前面较大的数字(比如前两个例子需要修改4)，
/// [2]有时候却要修改后面较小的那个数字(比如前第三个例子需要修改2)，
/// 那么有什么内在规律吗？是有的，判断修改那个数字其实跟再前面一个数的大小有关系，
/// 首先如果再前面的数不存在，比如例子1，4前面没有数字了，我们直接修改前面的数字为当前的数字2即可。
///
/// 而当再前面的数字存在，并且小于当前数时，比如例子2，-1小于2，我们还是需要修改前面的数字4为当前数字2；
/// 如果再前面的数大于当前数，比如例子3，3大于2，我们需要修改当前数2为前面的数3。
pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let len: usize = nums.len();
    // 记录前一个数比后一个数大的次数
    let mut count = 0;
    for i in 0..len - 1 {
        if nums[i] > nums[i + 1] {
            count += 1;
            // 从第二个数开始，如果前一个数比后一个数大，就把前一个数变成后一个数
            if i > 0 && nums[i - 1] > nums[i + 1] {
                nums[i + 1] = nums[i];
            }
        }
    }
    return count < 2;
}

/// 724. 寻找数组的中心下标
/// 给你一个整数数组 nums ，请计算数组的 中心下标 。
///
/// 数组 中心下标 是数组的一个下标，其左侧所有元素相加的和等于右侧所有元素相加的和。
///
/// 如果中心下标位于数组最左端，那么左侧数之和视为 0 ，因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。
///
/// 如果数组有多个中心下标，应该返回 最靠近左边 的那一个。如果数组不存在中心下标，返回 -1 。
///
///
///
/// 示例 1：
///
/// 输入：nums = [1, 7, 3, 6, 5, 6]
/// 输出：3
/// 解释：
/// 中心下标是 3 。
/// 左侧数之和 sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11 ，
/// 右侧数之和 sum = nums[4] + nums[5] = 5 + 6 = 11 ，二者相等。
///
/// 示例 2：
///
/// 输入：nums = [1, 2, 3]
/// 输出：-1
/// 解释：
/// 数组中不存在满足此条件的中心下标。
///
/// 示例 3：
///
/// 输入：nums = [2, 1, -1]
/// 输出：0
/// 解释：
/// 中心下标是 0 。
/// 左侧数之和 sum = 0 ，（下标 0 左侧不存在元素），
/// 右侧数之和 sum = nums[1] + nums[2] = 1 + -1 = 0 。
///
/// 在Rust中，你可以通过以下步骤来找到数组的中心下标：
///
/// 首先，计算数组的总和。
/// 然后，从左到右遍历数组，对于每个元素，如果它的左侧所有元素的和等于总和减去它和它右侧所有元素的和，那么这个元素就是中心下标。
/// 如果遍历完数组都没有找到中心下标，那么返回-1。
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        // num元素右侧元素之和为：total - left_sum - num
        if left_sum == total - left_sum - num {
            return i as i32;
        }
        // 左侧元素之和
        left_sum += num;
    }
    -1
}

/// 旋转数组
/// 给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
/// 示例 1:
/// 输入: nums = [1,2,3,4,5,6,7], k = 3
/// 输出: [5,6,7,1,2,3,4]
/// 解释:
/// 向右轮转 1 步: [7,1,2,3,4,5,6]
/// 向右轮转 2 步: [6,7,1,2,3,4,5]
/// 向右轮转 3 步: [5,6,7,1,2,3,4]
/// 示例 2:
/// 输入：nums = [-1,-100,3,99], k = 2
/// 输出：[3,99,-1,-100]
/// 解释:
/// 向右轮转 1 步: [99,-1,-100,3]
/// 向右轮转 2 步: [3,99,-1,-100]
/// 提示：
/// 1 <= nums.length <= 105
/// -231 <= nums[i] <= 231 - 1
/// 0 <= k <= 105
/// 进阶：
/// 尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
/// 你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    // 核心点 k % len 因为k有可能超过数组长度
    let k = k as usize % len;
    reverse(nums, 0, len);
    reverse(nums, 0, k as usize);
    reverse(nums, k as usize, len);
}

/// 反转数组 , 左闭，右闭 区间
fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        let temp = nums[start];
        nums[start] = nums[end - 1];
        nums[end - 1] = temp;
        start += 1;
        end -= 1;
    }
}

/// 396. 旋转函数
/// 给定一个长度为 n 的整数数组 nums 。
///
/// 假设 arrk 是数组 nums 顺时针旋转 k 个位置后的数组，我们定义 nums 的 旋转函数  F 为：
///
/// F(k) = 0 * arrk[0] + 1 * arrk[1] + ... + (n - 1) * arrk[n - 1]
/// 返回 F(0), F(1), ..., F(n-1)中的最大值 。
///
/// 生成的测试用例让答案符合 32 位 整数。
///
/// 示例 1:
///
/// 输入: nums = [4,3,2,6]
/// 输出: 26
/// 解释:
/// F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
/// F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
/// F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
/// F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
/// 所以 F(0), F(1), F(2), F(3) 中的最大值是 F(3) = 26 。
/// 示例 2:
///
/// 输入: nums = [100]
/// 输出: 0
///
/// 提示:
///
/// n == nums.length
/// 1 <= n <= 105
/// -100 <= nums[i] <= 100
///
/// 解题思路
/// 假设数组为[a, b, c, d, e]，长度为n，数组和为sum，可以发现旋转函数如下：
///
/// F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e
/// F(1) = 1 * a + 2 * b + 3 * c + 4 * d + 0 * e
/// F(2) = 2 * a + 3 * b + 4 * c + 0 * d + 1 * e
/// ...
/// F(1) - F(0) = a + b + c + d + e - 5 * e
/// F(2) - F(1) = a + b + c + d + e - 5 * d
/// ...
/// 所以 F(k) = F(k - 1) + sum - n * nums[n - k]，遍历过程每次取最大值即可。
/// 当k = 0时，数组不回发生顺时针旋转，所以F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e，所以可以先计算出F(0)的值。
/// 向右旋转一次，就相当于把当前结果加上整个数组的和，再减去数组大小乘以当前最后一位
pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    // 数组和
    let sum: i32 = nums.iter().sum();
    // 数组长度
    let n = nums.len();
    // 计算F(0)
    let mut f = nums
        .iter()
        .enumerate()
        // 这个闭包接受两个参数，第一个参数是累加器的当前值，第二个参数是一个元组，这个元组包含一个元素的索引和这个元素的值。
        // 闭包的作用是将元素的值与其索引的乘积加到累加器中。
        .fold(0, |acc, (i, x)| acc + (i as i32) * x);
    // 记录最大的结果
    let mut res = f;
    for i in 1..n {
        // 计算F(1) 到 F(n - 1)
        f = f + sum - (n as i32) * nums[n - i];
        res = res.max(f)
    }
    res
}

/**
 * 121. 买卖股票的最佳时机

给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。

你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。

返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。


```
示例 1：

输入：[7,1,5,3,6,4]
输出：5
解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
     注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
示例 2：

输入：prices = [7,6,4,3,1]
输出：0
解释：在这种情况下, 没有交易完成, 所以最大利润为 0。


提示：

1 <= prices.length <= 105
0 <= prices[i] <= 104
```
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut res = 0;
    for p in prices {
        min_price = min_price.min(p);
        res = res.max(p - min_price);
    }
    res
}

/**
 * 169. 多数元素
简单
相关标签
相关企业
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。


```
示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2


提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109


进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
```
 */
pub fn majority_element(nums: Vec<i32>) -> i32 {
    // 定义哈希表统计元素出现的次数
    let mut map: HashMap<i32, u32> = HashMap::new();
    let mut res = 0;
    let mut max_count = 0;
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            res = num;
        }
    }
    res
}

/**
 * 强化练习 3 ：单调数列
如果数组是单调递增或单调递减的，那么它是 单调 的。

如果对于所有 i <= j，nums[i] <= nums[j]，那么数组 nums 是单调递增的。
如果对于所有 i <= j，nums[i]> = nums[j]，那么数组 nums 是单调递减的。

当给定的数组 nums 是单调数组时返回 true，否则返回 false。



示例 1：

输入：nums = [1,2,2,3]
输出：true
示例 2：

输入：nums = [6,5,4,4]
输出：true
示例 3：

输入：nums = [1,3,2]
输出：false


提示：

1 <= nums.length <= 105
-105 <= nums[i] <= 105
 */
pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let (mut inc, mut dec) = (true, true);
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            dec = false;
        } else if nums[i] < nums[i - 1] {
            inc = false;
        }
    }
    inc || dec
}

/**
 * 313. 解压缩编码列表
简单
相关标签
相关企业
提示
给你一个以行程长度编码压缩的整数列表 nums 。

考虑每对相邻的两个元素 [freq, val] = [nums[2*i], nums[2*i+1]] （其中 i >= 0 ），
每一对都表示解压后子列表中有 freq 个值为 val 的元素，你需要从左到右连接所有子列表以生成解压后的列表。

请你返回解压后的列表。



示例 1：

输入：nums = [1,2,3,4]
输出：[2,4,4,4]
解释：第一对 [1,2] 代表着 2 的出现频次为 1，所以生成数组 [2]。
第二对 [3,4] 代表着 4 的出现频次为 3，所以生成数组 [4,4,4]。
最后将它们串联到一起 [2] + [4,4,4] = [2,4,4,4]。
示例 2：

输入：nums = [1,1,2,3]
输出：[1,3,3]


提示：

2 <= nums.length <= 100
nums.length % 2 == 0
1 <= nums[i] <= 100
 */
pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for i in (0..nums.len()).step_by(2) {
        // nums[i + 1] 为数组中初始化的值，nums[i] 为个数
        res.append(&mut vec![nums[i + 1]; nums[i] as usize]);
    }
    res
}

/**
 * 面试题 16.24. 数对和
中等
相关标签
相关企业
提示
设计一个算法，找出数组中两数之和为指定值的所有整数对。一个数只能属于一个数对。

示例 1:

输入: nums = [5,6,5], target = 11
输出: [[5,6]]
示例 2:

输入: nums = [5,6,5,6], target = 11
输出: [[5,6],[5,6]]
提示：

nums.length <= 100000
-10^5 <= nums[i], target <= 10^5
 */
pub fn pair_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut hash = HashMap::new();
    let mut res = Vec::new();
    for i in 0..nums.len() {
        if let Some(v) = hash.get(&(target - nums[i])) {
            let mut data = vec![0, 0];
            data[1] = nums[i];
            data[0] = target - nums[i];
            res.push(data);
            if *v == 1 {
                hash.remove(&(target - nums[i]));
            } else {
                hash.insert(target - nums[i], v - 1);
            }
        } else {
            hash.insert(nums[i], hash.get(&nums[i]).unwrap_or(&0) + 1);
        }
    }
    res
}

/**
 * 136. 只出现一次的数字

给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。



示例 1 ：

输入：nums = [2,2,1]
输出：1
示例 2 ：

输入：nums = [4,1,2,1,2]
输出：4
示例 3 ：

输入：nums = [1]
输出：1


提示：

1 <= nums.length <= 3 * 104
-3 * 104 <= nums[i] <= 3 * 104
除了某个元素只出现一次以外，其余每个元素均出现两次。
 */
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    for n in nums {
        hash.insert(n, hash.get(&n).unwrap_or(&0) + 1);
    }
    for (k, v) in hash {
        if v == 1 {
            return k;
        }
    }
    -1
}

/**
 * 137. 只出现一次的数字 II

给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。



示例 1：

输入：nums = [2,2,3,2]
输出：3
示例 2：

输入：nums = [0,1,0,1,0,1,99]
输出：99


提示：

1 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
nums 中，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次
 */
pub fn single_number_ii(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut nums = nums;
    nums.sort_unstable();
    if nums[0] != nums[1] {
        return nums[0];
    }
    if nums[nums.len() - 1] != nums[nums.len() - 2] {
        return nums[nums.len() - 1];
    }
    for i in 1..nums.len() - 1 {
        if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
            return nums[i];
        }
    }
    -1
}

/**
 * 260. 只出现一次的数字 III

给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。

你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。



示例 1：

输入：nums = [1,2,1,3,2,5]
输出：[3,5]
解释：[5, 3] 也是有效的答案。
示例 2：

输入：nums = [-1,0]
输出：[-1,0]
示例 3：

输入：nums = [0,1]
输出：[1,0]


提示：

2 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
除两个只出现一次的整数外，nums 中的其他数字都出现两次
 */
pub fn single_number_iii(nums: Vec<i32>) -> Vec<i32> {
    let xor_all = nums.iter().fold(0, |xor, &x| xor ^ x);
    let lowbit = xor_all & -xor_all;
    let mut ans = vec![0, 0];
    for &x in &nums {
        if (x & lowbit) == 0 {
            // 分组异或
            ans[0] ^= x;
        } else {
            ans[1] ^= x;
        }
    }
    ans
}

/**
 * 599. 两个列表的最小索引总和

假设 Andy 和 Doris 想在晚餐时选择一家餐厅，并且他们都有一个表示最喜爱餐厅的列表，每个餐厅的名字用字符串表示。

你需要帮助他们用最少的索引和找出他们共同喜爱的餐厅。 如果答案不止一个，则输出所有答案并且不考虑顺序。 你可以假设答案总是存在。



示例 1:

输入: list1 = ["Shogun", "Tapioca Express", "Burger King", "KFC"]，
list2 = ["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]
输出: ["Shogun"]
解释: 他们唯一共同喜爱的餐厅是“Shogun”。
示例 2:

输入:list1 = ["Shogun", "Tapioca Express", "Burger King", "KFC"]，
list2 = ["KFC", "Shogun", "Burger King"]
输出: ["Shogun"]
解释: 他们共同喜爱且具有最小索引和的餐厅是“Shogun”，它有最小的索引和1(0+1)。


提示:

1 <= list1.length, list2.length <= 1000
1 <= list1[i].length, list2[i].length <= 30
list1[i] 和 list2[i] 由空格 ' ' 和英文字母组成。
list1 的所有字符串都是 唯一 的。
list2 中的所有字符串都是 唯一 的。
 */
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut hash = HashMap::new();
    for (i, v) in list1.iter().enumerate() {
        hash.insert(v, i);
    }
    let mut min = i32::MAX;
    let mut res = Vec::new();
    for (i, v) in list2.iter().enumerate() {
        if let Some(n) = hash.get(&v) {
            let cur = (*n + i) as i32;
            if cur < min {
                min = (n + i) as i32;
                res.clear();
                res.push(v.clone());
            } else if cur == min {
                res.push(v.clone());
            }
        }
    }
    res
}

/**
 * 747. 至少是其他数字两倍的最大数
简单
相关标签
相关企业
提示
给你一个整数数组 nums ，其中总是存在 唯一的 一个最大整数 。

请你找出数组中的最大元素并检查它是否 至少是数组中每个其他数字的两倍 。如果是，则返回 最大元素的下标 ，否则返回 -1 。



示例 1：

输入：nums = [3,6,1,0]
输出：1
解释：6 是最大的整数，对于数组中的其他整数，6 至少是数组中其他元素的两倍。6 的下标是 1 ，所以返回 1 。
示例 2：

输入：nums = [1,2,3,4]
输出：-1
解释：4 没有超过 3 的两倍大，所以返回 -1 。


提示：

2 <= nums.length <= 50
0 <= nums[i] <= 100
nums 中的最大元素是唯一的
 */
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    // 最大元素
    let mut max = 0;
    // 最大元素坐标
    let mut max_idx = 0;
    // 第二大元素
    let mut s_max = 0;
    for (i, v) in nums.iter().enumerate() {
        if *v > max {
            s_max = max;
            max = *v;
            max_idx = i;
        } else if *v > s_max {
            s_max = *v;
        }
    }
    if max >= s_max * 2 {
        return max_idx as i32;
    }
    -1
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
    fn test_find_pairs_ii() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        let result = find_pairs_ii(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = [1, 2, 3, 1].to_vec();
        let contains_nearby_duplicate = contains_nearby_duplicate(nums, 3);
        println!("{:?}", contains_nearby_duplicate)
    }

    #[test]
    fn test_intersection() {
        let nums1 = [1, 2, 2, 1].to_vec();
        let nums2 = [2, 2].to_vec();
        let intersection = intersection(nums1, nums2);
        println!("{:?}", intersection)
    }

    #[test]
    fn test_check_possibility() {
        let nums = vec![4, 2, 3];
        let check_possibility = check_possibility(nums);
        println!("{:?}", check_possibility)
    }

    #[test]
    fn test_pivot_index() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        println!("{:?}", nums)
    }

    #[test]
    fn test_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max_profit = max_profit(prices);
        assert_eq!(max_profit, 5);
    }

    #[test]
    fn test_majority_element() {
        let nums = vec![3, 2, 3];
        let majority_element = majority_element(nums);
        assert_eq!(majority_element, 3);
    }

    #[test]
    fn test_decompress_rl_elist() {
        let nums = vec![1, 2, 3, 4];
        let decompress_rl_elist = decompress_rl_elist(nums);
        assert_eq!(decompress_rl_elist, vec![2, 4, 4, 4]);
    }

    #[test]
    fn test_pair_sums() {
        let nums = vec![2, 1, 8, 6, 5, 7, -1, 3, 5, 5];
        let target = 7;
        let pair_sums = pair_sums(nums, target);
        assert_eq!(pair_sums, vec![vec![-1, 8], vec![1, 6], vec![2, 5]]);
    }

    #[test]
    fn test_find_restaurant() {
        let list1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let list2 = vec![
            "Piatti".to_string(),
            "The Grill at Torrey Pines".to_string(),
            "Hungry Hunter Steakhouse".to_string(),
            "Shogun".to_string(),
        ];
        let find_restaurant = find_restaurant(list1, list2);
        assert_eq!(find_restaurant, vec!["Shogun".to_string()]);
    }
}
