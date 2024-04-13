fn shortestWord(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "Hello everyone, my name is Yuvraj Singh";
    match shortestWord(sentence) {
        Some(shortest) => println!("shortest word: {}", shortest),
        None => println!("Word not Found!!"),
    }
}