use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    numbers: VecDeque<i64>,
}

pub fn process_part1(input: &str) -> String {
    let (_, equations) = parse_equations(input).unwrap();
    let mut sum = 0;
    for equation in equations {
        //dbg!(equation.clone());
        if check_equation(equation.test_value, 0, equation.numbers) {
            //println!("success");
            sum += equation.test_value;
        }
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, equations) = parse_equations(input).unwrap();
    let mut sum = 0;
    for equation in equations {
        //dbg!(equation.clone());
        if check_equation_with_concat(equation.test_value, 0, equation.numbers) {
            //println!("success");
            sum += equation.test_value;
        }
    }
    sum.to_string()
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, test_value) = complete::i64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, numbers) = separated_list1(space1, complete::i64)(input)?;
    return Ok((
        input,
        Equation {
            test_value,
            numbers: VecDeque::from(numbers),
        },
    ));
}

fn check_equation(result: i64, current: i64, mut list: VecDeque<i64>) -> bool {
    if current == result && list.len() == 0 {
        return true;
    }
    if list.len() == 0 || current > result {
        return false;
    }
    let next = list.pop_front().unwrap();
    if check_equation(result, current * next, list.clone()) {
        return true;
    }
    if check_equation(result, current + next, list.clone()) {
        return true;
    }
    return false;
}

fn check_equation_with_concat(result: i64, current: i64, mut list: VecDeque<i64>) -> bool {
    if current == result && list.len() == 0 {
        return true;
    }
    if list.len() == 0 || current > result {
        return false;
    }
    let next = list.pop_front().unwrap();
    if check_equation_with_concat(result, current * next, list.clone()) {
        return true;
    }
    if check_equation_with_concat(result, current + next, list.clone()) {
        return true;
    }
    if check_equation_with_concat(result, concat(current, next), list.clone()) {
        return true;
    }
    return false;
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("3749".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("11387".to_string(), process_part2(&file));
    }
}
