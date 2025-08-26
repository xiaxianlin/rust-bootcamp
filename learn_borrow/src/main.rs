fn max(arr: &Vec<i32>) -> &i32 {
    let mut max: &i32 = &0;
    for item in arr.iter() {
        if item >= max {
            max = item;
        }
    }
    max
}

struct Article {
    content: String,
}

/* 处理边界借用检查冲突 */
fn handle_slice_borrow_checker_conflict() {
    // 借用冲突
    // let mut nums = [1, 2, 3, 4, 5];
    // let left = &mut nums[..2];
    // let right = &mut nums[2..];
    // left[0] = 10;
    // right[0] = 30;
    // println!("{:?}", nums);

    // 使用 split_at_mut 处理
    let mut nums = [1, 2, 3, 4, 5];
    let (left, right) = nums.split_at_mut(2);
    left[0] = 10;
    right[0] = 30;
    println!("nums1: {:?}", nums);

    // 使用作用域处理
    let mut nums = [1, 2, 3, 4, 5];
    {
        let left = &mut nums[..2];
        left[0] = 20;
    }
    {
        let right = &mut nums[2..];
        right[0] = 40;
    }
    println!("nums2: {:?}", nums);

    // 使用 unsafe 手动切片
    let mut nums = [1, 2, 3, 4, 5];
    let len = nums.len();
    let ptr = nums.as_mut_ptr();
    let left = unsafe { std::slice::from_raw_parts_mut(ptr, 2) };
    let right = unsafe { std::slice::from_raw_parts_mut(ptr.add(2), len - 2) };
    left[0] = 30;
    right[0] = 50;
    println!("nums3: {:?}", nums);
}

fn main() {
    println!("Hello, world!");
    let data = vec![1, 2, 3, 4, 5];
    println!("max: {}", max(&data));

    let mut article = Article {
        content: "test".to_string(),
    };
    let content = &mut article.content;
    *content = String::from("say hi");
    println!("content: {}", article.content);

    handle_slice_borrow_checker_conflict();
}
