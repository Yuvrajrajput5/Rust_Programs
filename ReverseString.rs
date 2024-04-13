fn reverseString(input: &str) -> String {
    let mut ans = String::new();
    for c in input.chars().rev() {
        ans.push(c);
    }
    ans
}

fn main() {
    let string = "Yuvraj Singh";
    let newstr= reverseString(string);
    println!("Reversed Version : {}", newstr);
}