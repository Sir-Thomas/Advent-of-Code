pub fn process_part1(input: &str) -> String {
    let mut disk = parse_disk(input);
    let disk = disk.move_blocks();
    disk.calculate_checksum().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut disk = parse_disk(input);
    println!("{:?}", disk.find_empty_sections(disk.map.len()));
    println!("{:?}", disk.find_files());
    "works".to_string()
}

fn parse_disk(input: &str) -> Disk {
    let mut input_type = InputType::File;
    let mut file_count = 0;
    let mut disk = Disk { map: vec![] };
    for char in input.chars() {
        for i in 0..char.to_digit(10).unwrap() {
            disk.map.push(match input_type {
                InputType::File => Some(file_count),
                InputType::Empty => None,
            });
        }
        if input_type == InputType::File {
            file_count += 1;
            input_type = InputType::Empty;
        } else {
            input_type = InputType::File;
        }
    }
    return disk;
}

#[derive(PartialEq)]
enum InputType {
    File,
    Empty,
}

#[derive(Debug)]
struct Disk {
    map: Vec<Option<u64>>
}

impl Disk {
    fn calculate_checksum(&self) -> u64 {
        let mut sum = 0;
        for (i, item) in self.map.iter().enumerate() {
            if item.is_none() {
                continue;
            }
            sum += item.unwrap() * i as u64;
        }
        return sum;
    }

    fn move_blocks(&self) -> Disk {
        let mut new_disk = Disk { map: vec![] };
        let (mut i, mut j) = (0, self.map.len()-1);
        while i <= j {
            while self.map[j].is_none() {
                j -= 1;
            }
            if self.map[i].is_some() {
                new_disk.map.push(self.map[i]);
                i += 1;
            }
            if self.map[i].is_none() && self.map[j].is_some() {
                new_disk.map.push(self.map[j]);
                i += 1;
                j -= 1;
            }
        }
        return new_disk;
    }

    fn move_files(&self) -> Disk {
        let new_disk = Disk { map: vec![] };

        let empty_sections = self.find_empty_sections();
        let files = self.find_files();

        
        return new_disk;
    }

    fn move_file(&self) -> Disk {
        let new_disk = Disk { map: vec![] };

        let empty_sections = self.find_empty_sections();
        let files = self.find_files();

        

        return new_disk;
    }

    fn find_empty_sections(&self, end: usize) -> Vec<Section> {
        let mut empty_sections = vec![];
        let mut i = 0;
        while i < end {
            if self.map[i].is_none() {
                let size = self.find_empty_size(i);
                empty_sections.push(Section { size, start: i });
                i += size;
            } else {
                i += 1;
            }
        }
        return empty_sections;
    }

    fn find_empty_size(&self, mut i: usize) -> usize {
        let mut size = 0;
        while self.map[i].is_none() && i < self.map.len() {
            size += 1;
            i += 1;
        }
        return size;
    }

    fn find_files(&self) -> Vec<Section> {
        let mut files = vec![];
        let mut i = self.map.len()-1;
        while i >= 0 {
            if self.map[i].is_some() {
                files.push(self.find_file_size_and_start(i));
                if i <= files.last().unwrap().size {
                    break;
                }
                i -= files.last().unwrap().size;
            } else {
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        return files;
    }

    fn find_file_size_and_start(&self, mut i: usize) -> Section {
        let mut size = 0;
        let char = self.map[i];
        while self.map[i] == char {
            size += 1;
            if i > 0 {
                i -= 1;
            } else {
                return Section { size, start: 0 };
            }
        }
        return Section { size, start: i-1 };
    }
}

#[derive(Debug)]
struct Section {
    start: usize,
    size: usize
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("1928".to_string(), process_part1(&file));
    }

    #[test]
    fn part2() {
        let file = fs::read_to_string("./test.txt").unwrap();
        assert_eq!("2858".to_string(), process_part2(&file));
    }
}
