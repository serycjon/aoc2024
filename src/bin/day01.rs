fn main() {
    let contents = std::fs::read_to_string("data/01_input").expect("Failed to read file");
    
    for line in contents.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();
        
        let (first, second) = (parts[0], parts[1]);
        println!("First: {}, Second: {}", first, second);
    }
}
