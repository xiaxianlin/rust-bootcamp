fn max(data: &[u8]) -> &u8 {
    let mut max = &data[0];
    for i in &data[1..] {
        if i > max {
            max = i;
        }
    }
    max
}

fn string_slice_rev1(data: &str) -> String {
    data.chars().rev().collect()
}

fn string_slice_rev2(data: &str) -> String {
    let bytes = data.as_bytes();
    let mut rev = Vec::with_capacity(bytes.len());
    for &b in bytes.iter().rev() {
        rev.push(b);
    }

    String::from_utf8(rev).unwrap()
}

fn submatrix_sum(
    matrix: &Vec<Vec<i32>>,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) -> i32 {
    let mut sum = 0;

    for row in &matrix[row_start..row_end] {
        for val in &row[col_start..col_end] {
            sum += *val;
        }
    }

    sum
}

fn main() {
    println!("Hello, world!");

    let arr = [3, 7, 2, 9, 5];
    println!("最大值: {:?}", max(&arr));

    let s = "Hello!";
    println!("原字符串: {}", s);
    println!("反转后1: {}", string_slice_rev1(s));
    println!("反转后2: {}", string_slice_rev2(s));

    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];
    let sum = submatrix_sum(&matrix, 1, 3, 2, 5); // 取 matrix[1..3][2..5]
    println!("子矩阵的和: {}", sum);
}
