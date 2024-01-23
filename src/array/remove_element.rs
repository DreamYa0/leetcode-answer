/// 双指针法
/// 
/// 双指针法（快慢指针法）： 通过一个快指针和慢指针在一个for循环下完成两个for循环的工作。
///
/// 定义快慢指针
///
/// 快指针：寻找新数组的元素 ，新数组就是不含有目标元素的数组
/// 慢指针：指向更新 新数组下标的位置
/// 很多同学这道题目做的很懵，就是不理解 快慢指针究竟都是什么含义，所以一定要明确含义，后面的思路就更容易理解了。
///
/// 很多同学不了解
/// 双指针法（快慢指针法）在数组和链表的操作中是非常常见的，很多考察数组、链表、字符串等操作的面试题，都使用双指针法。
/// 删除过程如下：
/// <img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/27.%E7%A7%BB%E9%99%A4%E5%85%83%E7%B4%A0-%E5%8F%8C%E6%8C%87%E9%92%88%E6%B3%95.gif" alt="image.png" style="zoom:50%;" />
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut temp = 0;
    // 遍历数组
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[temp] = nums[i];
            temp += 1;
        }
    }
    // 返回temp位置的索引就是数组的长度
    temp as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let val = 0;
        let len = remove_element(&mut nums, val);
        println!("{:?}", len);
    }
}
