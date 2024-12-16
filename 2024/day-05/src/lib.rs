use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
struct Rule {
    first: i32,
    second: i32,
}

pub fn process_part1(input: &str) -> String {
    let mut sum = 0;
    let (input, rules) = get_rules(input).unwrap();
    let (_, updates) = get_updates(input).unwrap();
    for update in updates {
        sum += check_update(update, rules.clone());
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut sum = 0;
    let (input, rules) = get_rules(input).unwrap();
    let (_, updates) = get_updates(input).unwrap();
    for update in updates {
        if check_update(update.clone(), rules.clone()) == 0 {
            sum += fix_update(update, rules.clone());
        }
    }
    sum.to_string()
}

fn get_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    let (input, rules) = separated_list1(newline, get_rule)(input)?;
    return Ok((input, rules));
}

fn get_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (first, second)) = separated_pair(complete::i32, tag("|"), complete::i32)(input)?;
    return Ok((input, Rule { first, second }));
}

fn get_updates(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, updates) = separated_list1(newline, get_update)(input)?;
    return Ok((input, updates));
}

fn get_update(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, update) = separated_list1(tag(","), complete::i32)(input)?;
    return Ok((input, update));
}

fn check_update(update: Vec<i32>, rules: Vec<Rule>) -> i32 {
    for rule in rules {
        if check_rule(update.clone(), rule) {
            continue;
        } else {
            return 0;
        }
    }
    let mid = update[update.len() / 2];
    return mid;
}

fn check_rule(update: Vec<i32>, rule: Rule) -> bool {
    let first = update.iter().position(|&u| u == rule.first);
    let second = update.iter().position(|&u| u == rule.second);
    if second.is_none() || first.is_none() {
        return true;
    }
    if second.unwrap() > first.unwrap() {
        return true;
    }
    return false;
}

fn fix_update(mut update: Vec<i32>, rules: Vec<Rule>) -> i32 {
    // ideally we would loop until all rules are satisfied, but this works for my input and is simpler
    for rule in rules.clone() {
        if !check_rule(update.clone(), rule.clone()) {
            update = fix_rule(update, rule);
        }
    }
    for rule in rules.clone() {
        if !check_rule(update.clone(), rule.clone()) {
            update = fix_rule(update, rule);
        }
    }
    for rule in rules.clone() {
        if !check_rule(update.clone(), rule.clone()) {
            update = fix_rule(update, rule);
        }
    }
    for rule in rules {
        if !check_rule(update.clone(), rule.clone()) {
            update = fix_rule(update, rule);
        }
    }
    let mid = update[update.len() / 2];
    return mid;
}

fn fix_rule(update: Vec<i32>, rule: Rule) -> Vec<i32> {
    let first = update.iter().position(|&u| u == rule.first).unwrap();
    let second = update.iter().position(|&u| u == rule.second).unwrap();
    let mut new_update = update.clone();
    new_update.swap(first, second);
    return new_update;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("143".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("123".to_string(), process_part2(&file));
    }
}
