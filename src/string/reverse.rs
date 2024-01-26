/// 541. 反转字符串II
/// 
/// https://leetcode.cn/problems/reverse-string-ii/
/// 
/// 思路
/// 这道题目其实也是模拟，实现题目中规定的反转规则就可以了。
///
/// 一些同学可能为了处理逻辑：每隔2k个字符的前k的字符，写了一堆逻辑代码或者再搞一个计数器，来统计2k，再统计前k个字符。
///
/// 其实在遍历字符串的过程中，只要让 i += (2 * k)，i 每次移动 2 * k 就可以了，然后判断是否需要有反转的区间。
///
/// 因为要找的也就是每2 * k 区间的起点，这样写，程序会高效很多。
///
/// 所以当需要固定规律一段一段去处理字符串的时候，要想想在在for循环的表达式上做做文章。
pub fn reverse_str(s: String, k: i32) -> String {
    let len = s.len();
    let k = k as usize;
    let mut s = s.chars().collect::<Vec<_>>();
    for i in (0..len).step_by(2 * k) {
        if i + k < len {
            reverse(&mut s, i, i + k - 1);
        }
        else {
            reverse(&mut s, i, len - 1);
        }
    }
    s.iter().collect::<String>()
}

fn reverse(s: &mut Vec<char>, mut begin: usize, mut end: usize){
    while begin < end {
        let temp = s[begin];
        s[begin] = s[end];
        s[end] = temp;
        begin += 1;
        end -= 1;
    }
}