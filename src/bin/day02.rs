fn main() {
    let contents = std::fs::read_to_string("data/02_input").expect("Failed to read file");

    let mut safe_count = 0;
    let mut damp_safe_count = 0;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();

	if is_safe(&levels) {
	    safe_count += 1;
	}

	for i in 0..levels.len() {
	    let mut dampened = levels.clone();
	    dampened.remove(i);
	    if is_safe(&dampened) {
		damp_safe_count += 1;
		break;
	    }
	}
    }
    println!("Part 1: {}", safe_count);
    println!("Part 2: {}", damp_safe_count);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut sign = 0;
    let mut safe = true;
    for (i, &item) in levels.iter().enumerate() {
	if i == 0 {
	    continue;
	}

	let diff = item - levels[i - 1];
	let diff_sign = num::signum(diff);

	if diff.abs() < 1 || diff.abs() > 3 {
	    safe = false;
	    break;
	}
	if sign != 0 && diff_sign != sign {
	    safe = false;
	    break;
	}
	sign = diff_sign;
    }
    return safe;
}
