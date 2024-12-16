use std::{slice::Iter, str::SplitWhitespace};

pub fn process_part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();
        if a - b > 0 && a - b < 4 {
            if decreasing(b, iter) {
                sum += 1;
            }
        } else if a - b < 0 && a - b > -4 {
            if increasing(b, iter) {
                sum += 1;
            }
        }
    }
    return sum.to_string();
}

fn decreasing(a: i32, mut iter: SplitWhitespace) -> bool {
    let b = iter.next();
    if b.is_none() {
        return true;
    }
    let b = b.unwrap().parse::<i32>().unwrap();
    if a - b > 0 && a - b < 4 {
        return decreasing(b, iter);
    } else {
        return false;
    }
}

fn increasing(a: i32, mut iter: SplitWhitespace) -> bool {
    let b = iter.next();
    if b.is_none() {
        return true;
    }
    let b = b.unwrap().parse::<i32>().unwrap();
    if a - b < 0 && a - b > -4 {
        return increasing(b, iter);
    } else {
        return false;
    }
}

pub fn process_part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        //println!("{}", sum);
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        if increasing(a, iter) {
            /* println!("increasing no damp"); */
            sum += 1;
            continue;
        }
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        if decreasing(a, iter) {
            /* println!("increasing no damp"); */
            sum += 1;
            continue;
        }
        let levels: Vec<&str> = line.split_whitespace().collect();
        for i in 0..(levels.len()) {
            let mut levels: Vec<&str> = line.split_whitespace().collect();
            levels.remove(i);
            let mut iter1 = levels.iter();
            let mut iter2 = levels.iter();
            let a1 = iter1.next().unwrap().parse::<i32>().unwrap();
            let a2 = iter2.next().unwrap().parse::<i32>().unwrap();
            if increasing_dampen(a1, iter1) {
                /* println!("{}", line);
                println!("removed index {}", i);
                println!("increasing");
                println!(); */
                sum += 1;
                break;
            } else if decreasing_dampen(a2, iter2) {
                /* println!("{}", line);
                println!("removed index {}", i);
                println!("decreasing");
                println!(); */
                sum += 1;
                break;
            }
            // println!("{}", line);
        }
    }
    return sum.to_string();
}

fn decreasing_dampen(a: i32, mut iter: Iter<&str>) -> bool {
    let b = iter.next();
    if b.is_none() {
        return true;
    }
    let b = b.unwrap().parse::<i32>().unwrap();
    if a - b > 0 && a - b < 4 {
        return decreasing_dampen(b, iter);
    } else {
        return false;
    }
}

fn increasing_dampen(a: i32, mut iter: Iter<&str>) -> bool {
    let b = iter.next();
    if b.is_none() {
        return true;
    }
    let b = b.unwrap().parse::<i32>().unwrap();
    if a - b < 0 && a - b > -4 {
        return increasing_dampen(b, iter);
    } else {
        return false;
    }
}
