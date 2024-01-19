/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
/// 复杂度分析
/// 时间复杂度：O(n)，其中 n 为 s 的长度。注意 left 至多增加 n 次，所以整个二重循环至多循环 O(n) 次。
/// 空间复杂度：O(∣Σ∣)，其中 ∣Σ∣ 为字符集合的大小，本题中字符均为 ASCII 字符，所以 ∣Σ∣≤128
pub fn length_of_longest_substring(st: String) -> i32 {
    let s = st.as_bytes();
    let mut ans = 0;
    let mut left = 0;
    let mut window = vec![false; 128]; // 也可以用 HashSet，这里为了效率用的 Vec
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        while window[c] {
            // 加入 c 后，窗口内会有重复元素
            window[s[left] as usize] = false;
            left += 1;
        }
        window[c] = true;
        ans = ans.max(right - left + 1); // 更新窗口长度最大值
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let st = "abcabcbb".to_string();
        let length = length_of_longest_substring(st);
        println!("{:?}", length)
    }
}