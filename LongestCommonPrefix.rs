fn longestCommonPrefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new(); 
    }

    let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();
    for (i, &byte) in shortest.as_bytes().iter().enumerate() 
    {
        if !strs.iter().all(|s| s.as_bytes().get(i) == Some(&byte)) {
            return shortest[..i].to_string(); 
        }
    }
    shortest.to_string()
}

fn main() {
    let strings = vec!["Yuvrajr".to_string(), "Yuvrajsingh".to_string(), "Yuv".to_string()];
    println!("longest common prefix is: {}", longestCommonPrefix(strings));
}