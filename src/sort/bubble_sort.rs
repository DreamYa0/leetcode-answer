/// 冒泡排序
pub fn bubble_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    // 记录最后一次交换的位置
    let mut last_exchange_index = 0;
    // 无序数列的边界，每次比较只需要比到这里为止
    let mut sort_border = len - 1;
    for _ in 0..len - 1 {
        // 有序标记，每一轮的初始是true
        let mut is_sorted = true;
        for j in 0..sort_border {
            if nums[j] > nums[j + 1] {
                // 交换
                let temp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = temp;
                // 有元素交换，所以不是有序，标记变为false
                is_sorted = false;
                // 把无序数列的边界更新为最后一次交换元素的位置
                last_exchange_index = j;
            }
        }
        // 更新无序数列的边界为最后一次交换元素的位置
        sort_border = last_exchange_index;
        // 如果有序标记为true，说明本轮遍历没有交换，已经是有序数列，直接跳出大循环
        if is_sorted {
            break;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array() {
        let nums = vec![5, 2, 3, 1];
        let res = bubble_sort(nums);
        assert_eq!(res, vec![1, 2, 3, 5]);
    }
}