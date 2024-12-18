#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

pub fn process_part1(input: &str) -> String {
    let mut map = get_map(input);
    let y = input.lines().position(|line| line.contains('^')).unwrap();
    let x = map[y].iter().position(|&c| c == '^').unwrap();
    let mut dir = Direction::Up;
    let mut position = Position { x, y };
    //print_map(map.clone());
    while position.x < map[0].len() && position.y < map.len() {
        (map, dir, position) = move_guard(map.clone(), dir.clone(), position.clone());
        //print_map(map.clone());
    }
    return count_x(map);
}

pub fn process_part2(input: &str) -> String {
    let map = get_map(input);
    let y = input.lines().position(|line| line.contains('^')).unwrap();
    let x = map[y].iter().position(|&c| c == '^').unwrap();
    let dir = Direction::Up;
    let position = Position { x, y };
    let mut sum = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, _char) in line.iter().enumerate() {
            sum += check_for_loop(map.clone(), dir.clone(), position.clone(), i, j);
        }
    }
    return sum.to_string();
}

// this is super slow, but I'm too lazy to think of a faster way right now
fn check_for_loop(
    mut map: Vec<Vec<char>>,
    mut dir: Direction,
    mut position: Position,
    i: usize,
    j: usize,
) -> i32 {
    map[i][j] = '#';
    for _ in 0..10000 {
        if position.x < map[0].len() && position.y < map.len() {
            (map, dir, position) = move_guard(map.clone(), dir.clone(), position.clone());
        } else {
            return 0;
        }
    }
    return 1;
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn move_guard(
    mut map: Vec<Vec<char>>,
    mut dir: Direction,
    position: Position,
) -> (Vec<Vec<char>>, Direction, Position) {
    if (position.y == map.len() - 1 && dir == Direction::Down)
        || (position.y == 0 && dir == Direction::Up)
        || (position.x == map[0].len() - 1 && dir == Direction::Right)
        || (position.x == 0 && dir == Direction::Left)
    {
        map[position.y][position.x] = 'X';
        return (
            map.clone(),
            dir,
            Position {
                x: map[0].len(),
                y: map.len(),
            },
        );
    }
    let target: Position;
    target = match dir {
        Direction::Up => Position {
            x: position.x,
            y: position.y - 1,
        },
        Direction::Down => Position {
            x: position.x,
            y: position.y + 1,
        },
        Direction::Left => Position {
            x: position.x - 1,
            y: position.y,
        },
        Direction::Right => Position {
            x: position.x + 1,
            y: position.y,
        },
    };
    if map[target.y][target.x] == '#' {
        dir = turn(dir);
        return (map, dir, position);
    } else {
        map[position.y][position.x] = 'X';
        return (map, dir, target);
    }
}

fn turn(dir: Direction) -> Direction {
    return match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    };
}

fn count_x(map: Vec<Vec<char>>) -> String {
    map.iter()
        .fold(0, |acc1: i32, line| {
            acc1 + (line.iter().fold(
                0,
                |acc2: i32, &char| if char == 'X' { acc2 + 1 } else { acc2 },
            ))
        })
        .to_string()
}

fn print_map(map: Vec<Vec<char>>) {
    for line in map {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("41".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("6".to_string(), process_part2(&file));
    }
}
