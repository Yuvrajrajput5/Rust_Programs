fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();  
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); 
    
    let reversed = s.chars().rev().collect::<String>();
    
    s == reversed 
}

fn main() {
    let test_cases = vec![
        "Yuvraj jarvuY",
        "Yuvraj",
        "Singh",
        "12321",
        "this is Pelindrome",
    ];

    for test_case in test_cases {
        println!("Pelindrome: '{}'  {} ", test_case, is_palindrome(test_case));
    }
}