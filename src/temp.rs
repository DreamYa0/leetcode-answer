pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // 1.定义左右指针
    let mut left = 0;
    let mut right = nums.len();
    // [left, right) 左闭右开区间
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid;
        }
    }
    -1
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }
    let left = left_bound(&nums, target);
    let right = right_bound(&nums, target);
    vec![left, right]
}

fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
    // 定义左右指针
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32;
    // [left, right) 左闭右开区间
    while left < right {
        let mid = (right + left) >> 1;
        if nums[mid as usize] == target {
            right = mid;
        } else if nums[mid as usize] > target {
            right = mid;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        }
    }

    if left < 0 || left >= nums.len() as i32 {
        return -1;
    }

    return if nums[left as usize] != target {
        -1
    } else {
        left as i32
    };
}

fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
    // 定义左右指针
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32;
    // [left, right) 左闭右开区间
    while left < right {
        let mid = (right + left) >> 1;
        if nums[mid as usize] == target {
            left = mid + 1;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else if nums[mid as usize] > target {
            right = mid;
        }
    }

    if left - 1 < 0 || left - 1 >= nums.len() as i32 {
        return -1;
    }

    return if nums[(left - 1) as usize] != target {
        -1
    } else {
        left as i32 - 1
    };
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = (right + left) >> 1;
        if nums[mid] <= nums[right] {
            // 左边有序
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    nums[left]
}
