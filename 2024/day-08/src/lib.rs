use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let (frequencies, max_y, max_x) = parse_input(input);
    let mut antis = vec![];
    for frequency in frequencies {
        for combination in frequency.positions.iter().permutations(2) {
            antis.push(find_antinode(*combination[0], *combination[1]));
        }
    }
    antis = antis
        .iter()
        .filter(|c| c.x >= 0 && c.y >= 0 && c.x < max_x && c.y < max_y)
        .cloned()
        .collect();
    antis.sort();
    antis.dedup();
    return format!("{}", antis.len());
}

pub fn process_part2(input: &str) -> String {
    let (frequencies, max_y, max_x) = parse_input(input);
    let mut antis: Vec<Coordinate> = vec![];
    for frequency in frequencies {
        for combination in frequency.positions.iter().permutations(2) {
            let mut next = find_antinodes(*combination[0], *combination[1], max_x, max_y);
            antis.append(&mut next);
        }
    }
    antis.sort();
    antis.dedup();
    return format!("{}", antis.len());
}

#[derive(Clone)]
struct Map {
    chars: Vec<Vec<char>>,
}

impl Map {
    fn clear_item(&mut self, y: usize, x: usize) {
        self.chars[y][x] = '.';
    }
    fn not_empty(map: Map) -> bool {
        for line in map.chars {
            for char in line {
                if char != '.' {
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Coordinate {
    y: isize,
    x: isize,
}

#[derive(Clone, Debug)]
struct Frequency {
    frequency: char,
    positions: Vec<Coordinate>,
}

fn parse_input(input: &str) -> (Vec<Frequency>, isize, isize) {
    let mut frequencies = vec![];
    let mut map = create_map(input);
    let mut frequency: Frequency;
    while Map::not_empty(map.clone()) {
        (map, frequency) = find_frequency(map.clone());
        frequencies.push(frequency);
    }
    return (
        frequencies,
        map.chars.len() as isize,
        map.chars[0].len() as isize,
    );
}

fn create_map(input: &str) -> Map {
    Map {
        chars: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn find_frequency(mut map: Map) -> (Map, Frequency) {
    let mut frequency = Frequency {
        frequency: ' ',
        positions: vec![],
    };
    'outer: for (y, line) in map.clone().chars.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char.is_ascii_alphanumeric() {
                frequency.frequency = char;
                frequency.positions.push(Coordinate {
                    y: y as isize,
                    x: x as isize,
                });
                map.clear_item(y, x);
                break 'outer;
            }
        }
    }
    for (y, line) in map.clone().chars.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == frequency.frequency {
                frequency.positions.push(Coordinate {
                    y: y as isize,
                    x: x as isize,
                });
                map.clear_item(y, x);
            }
        }
    }
    return (map, frequency);
}

fn find_antinode(a: Coordinate, b: Coordinate) -> Coordinate {
    let x1 = (a.x - b.x) + a.x;
    //let x2 = (b.x - a.x) + b.x;
    let y1 = (a.y - b.y) + a.y;
    //let y2 = (b.y - a.y) + b.y;
    return Coordinate { x: x1, y: y1 }; //, Coordinate { x: x2, y: y2 });
}

fn find_antinodes(a: Coordinate, b: Coordinate, max_x: isize, max_y: isize) -> Vec<Coordinate> {
    let mut antinodes = vec![];
    let delta_x = a.x - b.x;
    let delta_y = a.y - b.y;
    for i in 0.. {
        let x = a.x + delta_x * i;
        let y = a.y + delta_y * i;
        if x >= max_x || y >= max_y || x < 0 || y < 0 {
            break;
        }
        antinodes.push(Coordinate { x, y });
    }
    return antinodes;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("14".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("34".to_string(), process_part2(&file));
    }
}
