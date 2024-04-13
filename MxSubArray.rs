fn maxSubarraySum(nums: &[i32]) -> i32 {
    let mut Msum = 0;
    let mut Csum = 0;
    for &num in nums {
        Csum = Csum.max(0) + num;
        Msum = Msum.max(Csum);
    }
    Msum
}

fn main() {
    let nums = [4, -5, 3, -2, 1, 5, -6, 3];
    let max_sum = maxSubarraySum(&nums);
    println!("Max subarray Sum will be: {}", max_sum);
}