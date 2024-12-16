pub fn process_part1(input: &str) -> String {
    let mut sum = 0;
    let chars_lists: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    for i in 0..chars_lists.len() {
        for j in 0..chars_lists[i].len() {
            if chars_lists[i][j] == 'X' {
                sum += check_all(chars_lists.clone(), i as isize, j as isize);
            }
        }
    }
    sum.to_string()
}

fn check_all(chars_list: Vec<Vec<char>>, i: isize, j: isize) -> i32 {
    let mut sum = 0;
    sum += check_dir(chars_list.clone(), i, j, 1, -1);
    sum += check_dir(chars_list.clone(), i, j, 1, 0);
    sum += check_dir(chars_list.clone(), i, j, 1, 1);
    sum += check_dir(chars_list.clone(), i, j, 0, -1);
    sum += check_dir(chars_list.clone(), i, j, 0, 1);
    sum += check_dir(chars_list.clone(), i, j, -1, -1);
    sum += check_dir(chars_list.clone(), i, j, -1, 0);
    sum += check_dir(chars_list.clone(), i, j, -1, 1);
    return sum;
}

fn check_dir(chars_list: Vec<Vec<char>>, i: isize, j: isize, x: isize, y: isize) -> i32 {
    let c1 = get_char(chars_list.clone(), i + x, j + y);
    let c2 = get_char(chars_list.clone(), i + 2 * x, j + 2 * y);
    let c3 = get_char(chars_list, i + 3 * x, j + 3 * y);
    return match (c1, c2, c3) {
        (Ok('M'), Ok('A'), Ok('S')) => 1,
        _ => 0,
    };
}

fn get_char(chars_list: Vec<Vec<char>>, i: isize, j: isize) -> Result<char, String> {
    if i < chars_list.len() as isize && i >= 0 {
        if j < chars_list[i as usize].len() as isize && j >= 0 {
            return Ok(chars_list[i as usize][j as usize]);
        }
    }
    return Err("error".to_string());
}

pub fn process_part2(input: &str) -> String {
    let mut sum = 0;
    let chars_lists: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    for i in 0..chars_lists.len() {
        for j in 0..chars_lists[i].len() {
            if chars_lists[i][j] == 'A' {
                sum += check_x(chars_lists.clone(), i as isize, j as isize);
            }
        }
    }
    sum.to_string()
}

fn check_x(chars_list: Vec<Vec<char>>, i: isize, j: isize) -> i32 {
    let x1 = get_char(chars_list.clone(), i - 1, j - 1);
    let x2 = get_char(chars_list.clone(), i + 1, j + 1);
    let x3 = get_char(chars_list.clone(), i - 1, j + 1);
    let x4 = get_char(chars_list.clone(), i + 1, j - 1);
    return match (x1, x2, x3, x4) {
        (Ok('M'), Ok('S'), Ok('M'), Ok('S')) => 1,
        (Ok('S'), Ok('M'), Ok('M'), Ok('S')) => 1,
        (Ok('M'), Ok('S'), Ok('S'), Ok('M')) => 1,
        (Ok('S'), Ok('M'), Ok('S'), Ok('M')) => 1,
        _ => 0,
    };
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("18".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("9".to_string(), process_part2(&file));
    }
}
