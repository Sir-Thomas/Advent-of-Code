use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::IResult;

pub fn process_part1(mut input: &str) -> String {
    let mut result;
    let mut sum = 0;
    while input.len() > 0 {
        (input, result) = find_and_multiply(input).unwrap_or((&input[1..], 0));
        sum += result;
    }

    return sum.to_string();
}

pub fn process_part2(_input: &str) -> String {
    "works".to_string()
}

fn find_and_multiply(input: &str) -> IResult<&str, i32> {
    let (input, _) = take_until("mul(")(input)?;
    let (input, _) = tag("mul(")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (x * y)))
}
