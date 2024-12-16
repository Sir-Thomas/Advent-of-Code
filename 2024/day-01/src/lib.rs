pub fn process_part1(input: &str) -> String {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = (Vec::<i32>::new(), Vec::<i32>::new());
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        list1.push(x);
        list2.push(y);
    }
    list2.sort();
    list1.sort();

    let mut diff = 0;
    for items in list1.iter().zip(list2.iter()) {
        let (x, y) = items;
        diff += (x - y).abs();
    }
    return diff.to_string();
}

pub fn process_part2(input: &str) -> String {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = (Vec::<i32>::new(), Vec::<i32>::new());
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        list1.push(x);
        list2.push(y);
    }
    list2.sort();
    list1.sort();

    let mut similarity = 0;
    for i in list1.iter() {
        let j = list2.iter().filter(|&z| z == i).count();
        similarity += i * j as i32;
    }
    return similarity.to_string();
}
