fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut mer = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
            mer.push(arr1[i]);
            i += 1;
        
    }
    mer.extend_from_slice(&arr1[i..]);
    mer.extend_from_slice(&arr2[j..]);
    mer
}

fn main() {
    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged: {:?}", merged);
}