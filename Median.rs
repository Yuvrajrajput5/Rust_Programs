fn findMedian(num: &[i32]) -> f64 {
    let len = num.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (num[mid - 1] + num[mid]) as f64 / 2.0
    } else {
        num[len / 2] as f64
    }
}

fn main() {
    let Numbers = vec![1, 2, 3, 4, 5, 6, 12, 54, 96];

    println!("Median is: {}", findMedian(&Numbers));
 
}