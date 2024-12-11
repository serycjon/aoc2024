use num::BigInt;

fn main() {
    // let contents = std::fs::read_to_string("data/07_example").expect("Failed to read file");
    let contents = std::fs::read_to_string("data/07_input").expect("Failed to read file");

    let equations: Vec<(BigInt, Vec<BigInt>)> = contents.lines().map(|line| parse(line)).collect();

    let mut sum = BigInt::from(0);
    let mut sum_2 = BigInt::from(0);

    for (target, nums) in equations {
	if satisfiable(nums[0].clone(), &target, &nums[1..]) {
	    sum += target.clone();
	}
	if satisfiable_p2(nums[0].clone(), &target, &nums[1..]) {
	    sum_2 += target;
	}
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_2);
}

fn parse(line: &str) -> (BigInt, Vec<BigInt>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let target: BigInt = BigInt::parse_bytes(parts[0].as_bytes(), 10).unwrap();
    let nums: Vec<BigInt> = parts[1].split_whitespace().map(|x| BigInt::parse_bytes(x.as_bytes(), 10).unwrap()).collect();

    (target, nums)
}

fn satisfiable(cur: BigInt, test: &BigInt, nums: &[BigInt]) -> bool {
    if nums.len() == 0 {
	return cur == *test;
    }

    let next_num = &nums[0];
    let rest = &nums[1..];
    return satisfiable(cur.clone() + next_num.clone(), test, rest) || satisfiable(cur * next_num, test, rest);
}

fn satisfiable_p2(cur: BigInt, test: &BigInt, nums: &[BigInt]) -> bool {
    if nums.len() == 0 {
	return cur == *test;
    }

    let next_num = &nums[0];
    let rest = &nums[1..];
    return satisfiable_p2(cur.clone() + next_num.clone(), test, rest) ||
	satisfiable_p2(cur.clone() * next_num.clone(), test, rest) ||
	satisfiable_p2(concatenate_bigints(&cur, next_num), test, rest)
}

fn concatenate_bigints(a: &BigInt, b: &BigInt) -> BigInt {
    // wow ugly... by claude.ai... was too lazy to do this in some better way
    let concat_str = format!("{}{}", a, b);
    BigInt::parse_bytes(concat_str.as_bytes(), 10).unwrap()
}

