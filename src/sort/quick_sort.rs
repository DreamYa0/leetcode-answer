/// 快速排序
pub fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    do_quick_sort(&mut nums, 0, (len - 1) as i32);
    nums
}

fn do_quick_sort(nums: &mut Vec<i32>, start: i32, end: i32) {
    // 递归结束条件
    if start >= end {
        return;
    }
    // 得到基准元素位置
    let pivot_index = partition(nums, start, end);
    // 根据基准元素，分成两部分递归排序
    do_quick_sort(nums, start, pivot_index - 1);
    do_quick_sort(nums, pivot_index + 1, end);
}

/// 选择基准元素，并返回基准元素位置
fn partition(arr: &mut Vec<i32>, start_index: i32, end_index: i32) -> i32 {
    // 取第一个位置的元素作为基准元素
    let pivot = arr[start_index as usize];
    let (mut left, mut right) = (start_index as usize, end_index as usize);
    while left != right {
        // 控制right指针比较并左移,右边的数大于基准数则指针左移，如果小于或等于基准数则停止移动，等待被交换
        while left < right && arr[right] > pivot {
            right -= 1;
        }
        // 控制right指针比较并右移，左边的数小于或等于基准数则指针右移，如果大于基准数则停止移动，等待被交换
        while left < right && arr[left] <= pivot {
            left += 1;
        }
        // 交换left和right指向的元素
        if left < right {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
        }
    }
    // pivot和指针重合点交换
    arr[start_index as usize] = arr[left];
    arr[left] = pivot;
    left as i32
}

fn _partition_single(arr: &mut Vec<i32>, start_index: i32, end_index: i32) -> i32 {
    let pivot = arr[start_index as usize];
    // mark指针代表小于基准元素的区域边界
    let mut mark = start_index;
    for i in start_index + 1..end_index {
        if arr[i as usize] < pivot {
            // 小于基准元素的区域边界右移
            mark += 1;
            // 交换元素
            let temp = arr[mark as usize];
            arr[mark as usize] = arr[i as usize];
            arr[i as usize] = temp;
        }
    }
    // 将基准元素交换到中间
    arr[start_index as usize] = arr[mark as usize];
    // 将mark指针所在位置的元素交换到基准元素位置
    arr[mark as usize] = pivot;
    mark
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let nums = vec![5, 2, 3, 1];
        let res = quick_sort(nums);
        assert_eq!(res, vec![1, 2, 3, 5]);
    }
}
