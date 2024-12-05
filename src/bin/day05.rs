use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = std::fs::read_to_string("data/05_input").expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut sum = 0;
    let mut sum_2 = 0;


    let mut parsing_rules = true;
    let mut post: HashMap<&str, HashSet<&str>> = HashMap::new();  // which must be after the value?
    for line in lines {
	if line.len() == 0 {
	    parsing_rules = false;
	    continue;
	}
	if parsing_rules {
	    let rule: Vec<&str> = line.split("|").collect();
	    post.entry(rule[0]).or_default().insert(rule[1]);
	} else {
	    let pages: Vec<&str> = line.split(",").collect();
	    if valid(&pages, &post) {
		let middle: i32 = pages[pages.len() / 2].parse().expect("Error parsing the middle page");
		sum += middle;
	    } else {
		let middle: i32 = middle(&pages, &post);
		sum_2 += middle;
	    }
	}
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_2);
}

fn middle(pages: &Vec<&str>, post:&HashMap<&str, HashSet<&str>>) -> i32 {
    let pageset: HashSet<&str> = pages.iter().cloned().collect();
    let target = pages.len() / 2;
    for page in &pageset {
	match post.get(page) {
	    Some(must_be_after) => {
		let count_after = must_be_after.intersection(&pageset).count();
		if count_after == target {
		    return page.parse().expect("Error parsing the middle page");
		}
	    },
	    None => {},
	}
    };
    println!("Unreachable code reached... whoopsie...");
    return -42;
}

fn valid(pages: &Vec<&str>, post:&HashMap<&str, HashSet<&str>>) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();

    for page in pages {
	match post.get(page) {
	    Some(must_be_after) => {
		if !must_be_after.is_disjoint(&seen) {
		    return false;
		}
	    },
	    None => {},
	}
	seen.insert(page);
    };
    true
}
