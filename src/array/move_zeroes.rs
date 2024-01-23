/// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
/// 
/// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
/// 
/// 示例 1:
/// 输入: nums = [0,1,0,3,12]
/// 输出: [1,3,12,0,0]
/// 
/// 示例 2:
/// 输入: nums = [0]
/// 输出: [0]
/// 
/// 提示:
/// 1 <= nums.length <= 104
/// -231 <= nums[i] <= 231 - 1
/// 
/// 进阶：你能尽量减少完成的操作次数吗？
/// 
/// 可以设置一个指针,就是专业收集不是零的数 收集一遍后,后面的一定是0,就再将空出来的位置设置为0,就解决问题了
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 定义一个指针来记录非零元素的位置
    let mut temp = 0;
    // 开始收集不是零的数
    // [0,1,0,3,12]
    // --> [1,0,0,3,12]
    // --> [1,3,0,0,12]
    // --> [1,3,12,0,0]
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[temp] = nums[i];
            temp += 1;
        }
    }

    // 对num中temp位置之前的数据进行排序
    nums[..temp].sort();

    // 收集完毕后,后面自然就都是0了
    for i in temp..nums.len() {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 12, 3];
        move_zeroes(&mut nums);
        println!("{:?}", nums)
    }
}