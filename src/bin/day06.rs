use std::collections::{HashMap, HashSet};

fn main() {
    // let contents = std::fs::read_to_string("data/06_example").expect("Failed to read file");
    let contents = std::fs::read_to_string("data/06_input").expect("Failed to read file");

    let lines: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut sum_2 = 0;

    let h = lines.len();
    let w = lines[0].len();

    let mut cur_pos: (i32, i32) = (0, 0);
    let mut cur_dir: (i32, i32) = (-1, 0);

    let mut map: HashMap<(i32, i32), bool> = HashMap::new();

    for (row, line) in lines.into_iter().enumerate() {
	for (col, c) in line.into_iter().enumerate() {
	    match c {
		'#' => { map.insert((row as i32, col as i32), true); },
		'^' => { cur_pos = (row as i32, col as i32); },
		_ => {}
	    }
	}
    }

    let init_pos = cur_pos.clone();
    let init_dir = cur_dir.clone();

    let (visited, _) = simulate(init_pos, init_dir, h, w, &map); 

    sum = visited.iter().map(|(pos, dir)| pos).collect::<HashSet<_>>().len();

    let mut possible_obstacles: HashSet<(i32, i32)> = HashSet::new();

    for (i, (pos, dir)) in visited.iter().enumerate() {
	if i % 10 == 0 {
	    println!("{}/{}", i, visited.len());
	}
	let new_obstacle_pos = step(*pos, *dir);
	if new_obstacle_pos == init_pos {
	    continue;
	}
	let mut new_map = map.clone();
	new_map.insert(new_obstacle_pos, true);
	let (_, cycles) = simulate(init_pos, init_dir, h, w, &new_map);
	if cycles {
	    possible_obstacles.insert(new_obstacle_pos);
	}
    }

    sum_2 = possible_obstacles.len();

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_2);
}

fn obstacle_in_front(pos: (i32, i32), dir: (i32, i32), map: &HashMap<(i32, i32), bool>) -> bool {
    map.get(&step(pos, dir)).is_some()
}

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    // -1, 0
    // 0, 1
    // 1, 0
    // 0, -1
    (dir.1, -dir.0)
}

fn step(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn in_bounds(pos: (i32, i32), h: usize, w: usize) -> bool {
    pos.0 >= 0 && pos.0 < (h as i32) && pos.1 >= 0 && pos.1 < (w as i32)
}

fn simulate(init_pos: (i32, i32), init_dir: (i32, i32), h: usize, w: usize, map: &HashMap<(i32, i32), bool>) -> (HashSet<((i32, i32), (i32, i32))>, bool) {
    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    let mut cur_pos = init_pos.clone();
    let mut cur_dir = init_dir.clone();

    let mut cycles = false;

    while in_bounds(cur_pos, h, w) {
	if visited.contains(&(cur_pos, cur_dir)) {
	    cycles = true;
	    break;
	}
	visited.insert((cur_pos, cur_dir));
	if obstacle_in_front(cur_pos, cur_dir, &map) {
	    cur_dir = turn_right(cur_dir);
	} else {
	    cur_pos = step(cur_pos, cur_dir);
	}
    }

    (visited, cycles)
}
