fn first_Occurrence(num: &[i32], target: i32) -> Option<usize> 
{
    let mut l = 0;
    let mut r = num.len() - 1;
    let mut ans = None;

    while l <= r {
        let mid = l + (r - l) / 2;
        if num[mid] == target {
            ans = Some(mid);
            r = mid - 1; 
        } else if num[mid] < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    ans
}

fn main() 
{
    let num = vec![1, 2, 2, 3, 4, 5, 5, 6, 7];
    let target = 6;
    match first_Occurrence(&num, target) {
        Some(index) => println!("first occurrence :  {} is on index {}", target, index),
        None => println!("{} NOT FOUND IN ARRAY!!", target),
    }
}