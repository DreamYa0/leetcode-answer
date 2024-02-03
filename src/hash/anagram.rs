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
}
