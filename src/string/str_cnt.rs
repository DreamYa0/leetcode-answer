use std::collections::HashMap;

/**
 * 面试题 01.01. 判定字符是否唯一
简单
相关标签
相关企业
提示
实现一个算法，确定一个字符串 s 的所有字符是否全都不同。

示例 1：

输入: s = "leetcode"
输出: false
示例 2：

输入: s = "abc"
输出: true
限制：

0 <= len(s) <= 100
s[i]仅包含小写字母
如果你不使用额外的数据结构，会很加分。
 */
pub fn is_unique(astr: String) -> bool {
    let mut cnt = vec![0; 26];
    for c in astr.chars() {
        let idx = (c as u8 - b'a') as usize;
        if cnt[idx] > 0 {
            return false;
        }
        cnt[idx] += 1;
    }
    true
}

/// 383. 赎金信
/// 给你两个字符串：ransomNote 和 magazine ，判断 ransomNote 能不能由 magazine 里面的字符构成。
///
/// 如果可以，返回 true ；否则返回 false 。
///
/// magazine 中的每个字符只能在 ransomNote 中使用一次。
///
///
/// 示例 1：
///
/// 输入：ransomNote = "a", magazine = "b"
/// 输出：false
/// 示例 2：
///
/// 输入：ransomNote = "aa", magazine = "ab"
/// 输出：false
/// 示例 3：
///
/// 输入：ransomNote = "aa", magazine = "aab"
/// 输出：true
///
/// 提示：
///
/// 1 <= ransomNote.length, magazine.length <= 105
/// ransomNote 和 magazine 由小写英文字母组成
/// 思路
/// 这道题目和242.有效的字母异位词 (opens new window)很像，242.有效的字母异位词 (opens new window)相当于求 字符串a 和 字符串b 是否可以相互组成 ，而这道题目是求 字符串a能否组成字符串b，而不用管字符串b 能不能组成字符串a。
///
/// 本题判断第一个字符串ransom能不能由第二个字符串magazines里面的字符构成，但是这里需要注意两点。
///
/// 第一点“为了不暴露赎金信字迹，要从杂志上搜索各个需要的字母，组成单词来表达意思”  这里说明杂志里面的字母不可重复使用。
///
/// 第二点 “你可以假设两个字符串均只含有小写字母。” 说明只有小写字母，这一点很重要
///
/// 哈希解法
/// 因为题目说只有小写字母，那可以采用空间换取时间的哈希策略，用一个长度为26的数组来记录magazine里字母出现的次数。
///
/// 然后再用ransomNote去验证这个数组是否包含了ransomNote所需要的所有字母。
///
/// 依然是数组在哈希法中的应用。
///
/// 一些同学可能想，用数组干啥，都用map完事了，其实在本题的情况下，使用map的空间消耗要比数组大一些的，因为map要维护红黑树或者哈希表，而且还要做哈希函数，是费时的！数据量大的话就能体现出来差别了。 所以数组更加简单直接有效！
/// 时间复杂度: O(n)
/// 空间复杂度: O(1)
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }

    // 存放 a - z 26个字母的acii码
    let mut temp = [0; 26];
    for b in magazine.as_bytes() {
        // 计数
        temp[(b - b'a') as usize] += 1;
    }

    for b in ransom_note.as_bytes() {
        // 扣减计数
        temp[(b - b'a') as usize] -= 1;
    }

    for i in temp {
        if i < 0 {
            return false;
        }
    }

    return true;
}

/// 242.有效的字母异位词
/// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
///
/// 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
///
///
/// 示例 1:
///
/// 输入: s = "anagram", t = "nagaram"
/// 输出: true
/// 示例 2:
///
/// 输入: s = "rat", t = "car"
/// 输出: false
///
/// 提示:
///
/// 1 <= s.length, t.length <= 5 * 104
/// s 和 t 仅包含小写字母
///
/// 进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
/// 思路
/// 先看暴力的解法，两层for循环，同时还要记录字符是否重复出现，很明显时间复杂度是 O(n^2)。
///
/// 暴力的方法这里就不做介绍了，直接看一下有没有更优的方式。
///
/// 数组其实就是一个简单哈希表，而且这道题目中字符串只有小写字符，那么就可以定义一个数组，来记录字符串s里字符出现的次数。
///
/// 如果对哈希表的理论基础关于数组，set，map不了解的话可以看这篇：关于哈希表，你该了解这些！(opens new window)
///
/// 需要定义一个多大的数组呢，定一个数组叫做record，大小为26 就可以了，初始化为0，因为字符a到字符z的ASCII也是26个连续的数值。
///
/// 为了方便举例，判断一下字符串s= "aee", t = "eae"。
///
/// 操作动画如下：
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/242.%E6%9C%89%E6%95%88%E7%9A%84%E5%AD%97%E6%AF%8D%E5%BC%82%E4%BD%8D%E8%AF%8D.gif" />
///
/// 242.有效的字母异位词
///
/// 定义一个数组叫做record用来上记录字符串s里字符出现的次数。
///
/// 需要把字符映射到数组也就是哈希表的索引下标上，因为字符a到字符z的ASCII是26个连续的数值，所以字符a映射为下标0，相应的字符z映射为下标25。
///
/// 再遍历 字符串s的时候，只需要将 s[i] - ‘a’ 所在的元素做+1 操作即可，并不需要记住字符a的ASCII，只要求出一个相对数值就可以了。 这样就将字符串s中字符出现的次数，统计出来了。
///
/// 那看一下如何检查字符串t中是否出现了这些字符，同样在遍历字符串t的时候，对t中出现的字符映射哈希表索引上的数值再做-1的操作。
///
/// 那么最后检查一下，record数组如果有的元素不为零0，说明字符串s和t一定是谁多了字符或者谁少了字符，return false。
///
/// 最后如果record数组所有元素都为零0，说明字符串s和t是字母异位词，return true。
///
/// 时间复杂度为O(n)，空间上因为定义是的一个常量大小的辅助数组，所以空间复杂度为O(1)。
pub fn is_anagram(s: String, t: String) -> bool {
    // 初始化一个存放26个字母的数组
    let mut record = [0; 26];
    let base_char = 'a';

    // 统计s字符串中每个字符出现的次数
    for byte in s.bytes() {
        record[byte as usize - base_char as usize] += 1;
    }

    // 从record减去t字符串中字符出现的次数
    for byte in t.bytes() {
        record[byte as usize - base_char as usize] -= 1;
    }

    // 判断数组中字符统计次数是不是都为0，如果都为0说明 s和t是由相同的字符组成
    record.iter().filter(|x| **x != 0).count() == 0
}

/// 49.字母异位词分组
/// 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
///
/// 字母异位词 是由重新排列源单词的所有字母得到的一个新单词。
///
///
/// 示例 1:
///
/// 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
/// 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
/// 示例 2:
///
/// 输入: strs = [""]
/// 输出: [[""]]
/// 示例 3:
///
/// 输入: strs = ["a"]
/// 输出: [["a"]]
///
/// 提示：
///
/// 1 <= strs.length <= 104
/// 0 <= strs[i].length <= 100
/// strs[i] 仅包含小写字母
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();
    strs.into_iter().for_each(|x| {
        // 统计x字符串中字符出现的次数，通过这种方式可以判断字符串是不是异位词。
        let mut xx = [0; 26];
        x.bytes().for_each(|x| xx[(x - b'a') as usize] += 1);
        // 如果map中有xx条目，则在xx条目的value值中追加x，如果没有xx条目则创建一个新的。
        map.entry(xx).or_insert(vec![]).push(x)
    });
    map.values().cloned().collect()
}

/**
 * 面试题 01.02. 判定是否互为字符重排

给定两个由小写字母组成的字符串 s1 和 s2，请编写一个程序，确定其中一个字符串的字符重新排列后，能否变成另一个字符串。

示例 1：

输入: s1 = "abc", s2 = "bca"
输出: true
示例 2：

输入: s1 = "abc", s2 = "bad"
输出: false
说明：

0 <= len(s1) <= 100
0 <= len(s2) <= 100
 */
pub fn check_permutation(s1: String, s2: String) -> bool {
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    if s1.len() != s2.len() {
        return false;
    }
    let mut cnt = vec![0; 26];
    for c in s1 {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }
    for c in s2 {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] -= 1;
        if cnt[idx] < 0 {
            return false;
        }
    }
    cnt.iter().all(|&x| x == 0)
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
 * 1832. 判断句子是否为全字母句

全字母句 指包含英语字母表中每个字母至少一次的句子。

给你一个仅由小写英文字母组成的字符串 sentence ，请你判断 sentence 是否为 全字母句 。

如果是，返回 true ；否则，返回 false 。



示例 1：

输入：sentence = "thequickbrownfoxjumpsoverthelazydog"
输出：true
解释：sentence 包含英语字母表中每个字母至少一次。
示例 2：

输入：sentence = "leetcode"
输出：false


提示：

1 <= sentence.length <= 1000
sentence 由小写英语字母组成
 */
pub fn check_if_pangram(sentence: String) -> bool {
    let mut cnt = vec![1; 26];
    for c in sentence.chars() {
        cnt[(c as u8 - b'a') as usize] -= 1;
    }
    cnt.iter().all(|&x| x <= 0)
}

/**
 * 2053. 数组中第 K 个独一无二的字符串

独一无二的字符串 指的是在一个数组中只出现过 一次 的字符串。

给你一个字符串数组 arr 和一个整数 k ，请你返回 arr 中第 k 个 独一无二的字符串 。如果 少于 k 个独一无二的字符串，那么返回 空字符串 "" 。

注意，按照字符串在原数组中的 顺序 找到第 k 个独一无二字符串。



示例 1:

输入：arr = ["d","b","c","b","c","a"], k = 2
输出："a"
解释：
arr 中独一无二字符串包括 "d" 和 "a" 。
"d" 首先出现，所以它是第 1 个独一无二字符串。
"a" 第二个出现，所以它是 2 个独一无二字符串。
由于 k == 2 ，返回 "a" 。
示例 2:

输入：arr = ["aaa","aa","a"], k = 1
输出："aaa"
解释：
arr 中所有字符串都是独一无二的，所以返回第 1 个字符串 "aaa" 。
示例 3：

输入：arr = ["a","b","a"], k = 3
输出：""
解释：
唯一一个独一无二字符串是 "b" 。由于少于 3 个独一无二字符串，我们返回空字符串 "" 。


提示：

1 <= k <= arr.length <= 1000
1 <= arr[i].length <= 5
arr[i] 只包含小写英文字母。
 */
pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut cnt = std::collections::HashMap::new();
    for s in &arr {
        *cnt.entry(s).or_insert(0) += 1;
    }
    let mut k = k;
    for s in &arr {
        if cnt[s] == 1 {
            k -= 1;
            if k == 0 {
                return s.clone();
            }
        }
    }
    "".to_string()
}

/**
 * 771. 宝石与石头

 给你一个字符串 jewels 代表石头中宝石的类型，另有一个字符串 stones 代表你拥有的石头。
 stones 中每个字符代表了一种你拥有的石头的类型，你想知道你拥有的石头中有多少是宝石。

字母区分大小写，因此 "a" 和 "A" 是不同类型的石头。



示例 1：

输入：jewels = "aA", stones = "aAAbbbb"
输出：3
示例 2：

输入：jewels = "z", stones = "ZZ"
输出：0


提示：

1 <= jewels.length, stones.length <= 50
jewels 和 stones 仅由英文字母组成
jewels 中的所有字符都是 唯一的
 */
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels = jewels.as_bytes();
    let stones = stones.as_bytes();
    let mut cnt = vec![0; 128];
    for x in stones {
        cnt[*x as usize] += 1
    }
    let mut res = 0;
    for j in jewels {
        if cnt[*j as usize] > 0 {
            res += cnt[*j as usize];
            // 如果已经统计过了，就置为0，避免jewels有重复字符导致重复统计
            cnt[*j as usize] = 0;
        }
    }
    res
}

/**
 * 强化练习 1：强化练习
给定一个字符串，编写一个函数判定其是否为某个回文串的排列之一。

回文串是指正反两个方向都一样的单词或短语。排列是指字母的重新排列。

回文串不一定是字典当中的单词。



示例1：

输入："tactcoa"
输出：true（排列有"tacocat"、"atcocta"，等等）
 */
pub fn can_permute_palindrome(s: String) -> bool {
    let mut map = std::collections::HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut odd_count = 0;
    for (_, count) in map {
        if count % 2 != 0 {
            odd_count += 1;
        }
    }
    odd_count <= 1
}

/**
 * 389. 找不同
简单
相关标签
相关企业
给定两个字符串 s 和 t ，它们只包含小写字母。

字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。

请找出在 t 中被添加的字母。



示例 1：

输入：s = "abcd", t = "abcde"
输出："e"
解释：'e' 是那个被添加的字母。
示例 2：

输入：s = "", t = "y"
输出："y"


提示：

0 <= s.length <= 1000
t.length == s.length + 1
s 和 t 只包含小写字母
 */
pub fn find_the_difference(s: String, t: String) -> char {
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let mut cnt = vec![0; 26];
    for c in s {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    for c in t {
        cnt[(c as u8 - b'a') as usize] -= 1;
    }
    for (i, v) in cnt.iter().enumerate() {
        if *v == -1 {
            return (i as u8 + b'a') as char;
        }
    }
    ' '
}

/**
 * 451. 根据字符出现频率排序

给定一个字符串 s ，根据字符出现的 频率 对其进行 降序排序 。一个字符出现的 频率 是它出现在字符串中的次数。

返回 已排序的字符串 。如果有多个答案，返回其中任何一个。



示例 1:

输入: s = "tree"
输出: "eert"
解释: 'e'出现两次，'r'和't'都只出现一次。
因此'e'必须出现在'r'和't'之前。此外，"eetr"也是一个有效的答案。
示例 2:

输入: s = "cccaaa"
输出: "cccaaa"
解释: 'c'和'a'都出现三次。此外，"aaaccc"也是有效的答案。
注意"cacaca"是不正确的，因为相同的字母必须放在一起。
示例 3:

输入: s = "Aabb"
输出: "bbAa"
解释: 此外，"bbaA"也是一个有效的答案，但"Aabb"是不正确的。
注意'A'和'a'被认为是两种不同的字符。


提示:

1 <= s.length <= 5 * 105
s 由大小写英文字母和数字组成
 */
pub fn frequency_sort(s: String) -> String {
    let mut hash = HashMap::new();
    let s = s.chars().collect::<Vec<char>>();
    for c in s {
        hash.insert(c, hash.get(&c).unwrap_or(&0) + 1);
    }
    // 对hash进行排序
    let mut vec: Vec<(&char, &i32)> = hash.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    // 定义结果
    let mut res = vec![];
    for (c, n) in vec {
        for _ in 0..*n {
            res.push(*c);
        }
    }
    res.iter().collect()
}

/**
 * 423. 从英文中重建数字

给你一个字符串 s ，其中包含字母顺序打乱的用英文单词表示的若干数字（0-9）。按 升序 返回原始的数字。



示例 1：

输入：s = "owoztneoer"
输出："012"
示例 2：

输入：s = "fviefuro"
输出："45"


提示：

1 <= s.length <= 105
s[i] 为 ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"] 这些字符之一
s 保证是一个符合题目要求的字符串
 */
pub fn original_digits(s: String) -> String {
    let mut tab = [0; 256];
    let mut ans = [0; 10];
    for c in s.into_bytes() {
        tab[c as usize] += 1
    }
    helper(&mut tab, &mut ans[0], 'z', "zero");
    helper(&mut tab, &mut ans[2], 'w', "two");
    helper(&mut tab, &mut ans[4], 'u', "four");
    helper(&mut tab, &mut ans[6], 'x', "six");
    helper(&mut tab, &mut ans[8], 'g', "eight");
    helper(&mut tab, &mut ans[1], 'o', "one");
    helper(&mut tab, &mut ans[3], 'r', "three");
    helper(&mut tab, &mut ans[5], 'f', "five");
    helper(&mut tab, &mut ans[7], 'v', "seven");
    helper(&mut tab, &mut ans[9], 'e', "nine");
    let mut res = String::with_capacity(ans.iter().sum::<usize>());
    for i in 0..10 {
        for _ in 0..ans[i] {
            res.push((i as u8 + b'0') as char);
        }
    }
    res
}

fn helper(tab: &mut [usize], ans: &mut usize, check: char, s: &str) {
    let cnt = tab[check as usize];
    if cnt > 0 {
        *ans = cnt;
        for c in s.bytes() {
            tab[c as usize] -= cnt;
        }
    }
}

/**
 * 551. 学生出勤记录 I

给你一个字符串 s 表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。记录中只含下面三种字符：

'A'：Absent，缺勤
'L'：Late，迟到
'P'：Present，到场
如果学生能够 同时 满足下面两个条件，则可以获得出勤奖励：

按 总出勤 计，学生缺勤（'A'）严格 少于两天。
学生 不会 存在 连续 3 天或 连续 3 天以上的迟到（'L'）记录。
如果学生可以获得出勤奖励，返回 true ；否则，返回 false 。



示例 1：

输入：s = "PPALLP"
输出：true
解释：学生缺勤次数少于 2 次，且不存在 3 天或以上的连续迟到记录。
示例 2：

输入：s = "PPALLL"
输出：false
解释：学生最后三天连续迟到，所以不满足出勤奖励的条件。


提示：

1 <= s.length <= 1000
s[i] 为 'A'、'L' 或 'P'
 */
pub fn check_record(s: String) -> bool {
    let mut hash = HashMap::new();
    for c in s.chars() {
        *hash.entry(c).or_insert(0) += 1;
    }
    hash.get(&'A').unwrap_or(&0) < &2 && !s.contains("LLL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        let can_construct = can_construct(ransom_note, magazine);
        println!("{}", can_construct)
    }

    #[test]
    fn test_check_permutation() {
        let s1 = "abc".to_string();
        let s2 = "bca".to_string();
        assert_eq!(check_permutation(s1, s2), true);
    }

    #[test]
    fn test_are_occurrences_equal() {
        let s = "tveixwaeoezcf".to_string();
        let result = are_occurrences_equal(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_kth_distinct() {
        let arr = vec![
            "d".to_string(),
            "b".to_string(),
            "c".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];
        let k = 2;
        assert_eq!(kth_distinct(arr, k), "a".to_string());
    }

    #[test]
    fn test_num_jewels_in_stones() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        assert_eq!(num_jewels_in_stones(jewels, stones), 3);
    }

    #[test]
    fn test_frequency_sort() {
        let s = "tree".to_string();
        assert_eq!(frequency_sort(s), "eert".to_string());
    }
}
