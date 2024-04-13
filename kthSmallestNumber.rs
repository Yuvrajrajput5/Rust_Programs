fn kth_smallest(num: &[i32], k: usize) -> i32 {
    let mut sorted_num = num.to_vec();
    sorted_num.sort();
    sorted_num[k - 1]
}

fn main() {
    let num = vec![1,2,3,4,5,6,7,8];
    let k = 4;
    let kth_smallest_element = kth_smallest(&num, k);
    println!("The {}th smallest element: {}", k, kth_smallest_element);
}