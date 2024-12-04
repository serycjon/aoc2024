fn main() {
    let contents = std::fs::read_to_string("data/04_input").expect("Failed to read file");

    let lines: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut sum = 0;
    let mut sum_2 = 0;
    let directions: Vec<Vec<i32>> = vec![
	vec![0, 1],
	vec![1, 1],
	vec![1, 0],
	vec![1, -1],
	vec![0, -1],
	vec![-1, -1],
	vec![-1, 0],
	vec![-1, 1]
	];

    let directions_2: Vec<Vec<i32>> = vec![
	vec![1, 1],
	vec![1, -1],
	vec![-1, -1],
	vec![-1, 1]
	];

    for y in 0..h {
	for x in 0..w {
	    for direction in &directions {
		if search(&lines, x, y, direction) {
		    sum += 1;
		}
	    }
	    let mut found_directions = 0;
	    for direction in &directions_2 {
		if search_2(&lines, x, y, direction) {
		    found_directions += 1;
		}
	    }
	    if found_directions == 2 {
		sum_2 += 1;
	    }
	}
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_2);
}

fn search(lines: &Vec<Vec<char>>, x: usize, y: usize, direction: &Vec<i32>) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;

    let needle = "XMAS";

    for c in needle.chars() {
	if !in_bounds(x, y, lines) {
	    return false;
	}
	if lines[y as usize][x as usize] != c {
	    return false;
	}

	x += direction[0];
	y += direction[1];
    }
    
    true
}

fn search_2(lines: &Vec<Vec<char>>, x: usize, y:usize, direction: &Vec<i32>) -> bool {
    let mut x = (x as i32) - direction[0];
    let mut y = (y as i32) - direction[1];

    let needle = "MAS";

    for c in needle.chars() {
	if !in_bounds(x, y, lines) {
	    return false;
	}
	if lines[y as usize][x as usize] != c {
	    return false;
	}

	x += direction[0];
	y += direction[1];
    }
    
    true
}

fn in_bounds(x: i32, y: i32, lines: &Vec<Vec<char>>) -> bool {
    if y < 0 || y >= (lines.len() as i32) {
	return false;
    }
    if x < 0 || x >= (lines[y as usize].len() as i32) {
	return false;
    }
    return true;
}
