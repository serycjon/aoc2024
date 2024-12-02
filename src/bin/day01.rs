fn main() {
    let contents = std::fs::read_to_string("data/01_input").expect("Failed to read file");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    
    for line in contents.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();
        
	xs.push(parts[0]);
	ys.push(parts[1]);
        // let (first, second) = (parts[0], parts[1]);
        // println!("First: {}, Second: {}", first, second);
    }

    xs.sort();
    ys.sort();

    let mut sum = 0;
    for parts in xs.iter().zip(ys.iter()) {
	sum += (parts.0 - parts.1).abs();
	// println!("{}", (parts.0 - parts.1).abs());
    }
    println!("Part 1: {}", sum);
}
