use std::collections::HashMap;

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
    }

    xs.sort();
    ys.sort();

    let mut counts = HashMap::new();

    let mut sum = 0;
    for parts in xs.iter().zip(ys.iter()) {
	counts.entry(parts.1).and_modify(|count| *count += 1).or_insert(1);
	sum += (parts.0 - parts.1).abs();
    }

    let mut sum2 = 0;
    for x in xs {
	sum2 += x * counts.get(&x).cloned().unwrap_or(0);
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
