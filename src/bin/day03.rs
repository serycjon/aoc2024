fn main() {
    let contents = std::fs::read_to_string("data/03_input").expect("Failed to read file");

    // let contents = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    // println!("{}", parse_pair("34,15").unwrap().1.1);

    let mut sum = 0;
    let mut sum2 = 0;
    let mut enabled = true;
    for line in contents.lines() {
	let mut results = Vec::new();
	let mut remaining = line;

	while !remaining.is_empty() {
	    match parse_instruction(remaining) {
		Ok((rest, matched)) => {
		    results.push(matched);
		    remaining = rest;
		},
		Err(_) => {
		    remaining = &remaining[1..];
		}
	    }
	}

	for inst in results {
	    match inst {
		Instruction::Mult((x, y)) => {
		    sum += x * y;
		    if enabled {
			sum2 += x * y;
		    }
		},
		Instruction::Enable(state) => {
		    enabled = state;
		}
	    }
	}
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn parse_number(input: &str) -> nom::IResult<&str, i32> {
    let (input, number_str) = nom::character::complete::digit1(input)?;
    Ok((input, number_str.parse().unwrap()))
}

fn parse_pair(input: &str) -> nom::IResult<&str, (i32, i32)> {
    let (input, (x, y)) = nom::sequence::separated_pair(
	parse_number,
	nom::bytes::complete::tag(","),
	parse_number
    )(input)?;
    Ok((input, (x, y)))
}

fn parse_mult(input: &str) -> nom::IResult<&str, (i32, i32)> {
    let (input, (x, y)) = nom::sequence::delimited(
	nom::bytes::complete::tag("mul("),
	parse_pair,
	nom::bytes::complete::tag(")"))(input)?;
    Ok((input, (x, y)))
}

fn parse_do(input: &str) -> nom::IResult<&str, bool> {
    let (input, _) = nom::bytes::complete::tag("do()")(input)?;
    Ok((input, true))
}

fn parse_dont(input: &str) -> nom::IResult<&str, bool> {
    let (input, _) = nom::bytes::complete::tag("don't()")(input)?;
    Ok((input, false))
}

enum Instruction {
    Mult((i32, i32)),
    Enable(bool)
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, res) = nom::branch::alt(
	(
	    nom::combinator::map(parse_mult, |(first, second)| Instruction::Mult((first, second))),
	    nom::combinator::map(parse_do, Instruction::Enable),
	    nom::combinator::map(parse_dont, Instruction::Enable)
	))(input)?;
    Ok((input, res))
}
