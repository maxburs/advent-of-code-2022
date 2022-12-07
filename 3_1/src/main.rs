use std::{ collections::HashSet};

fn priority(item_type: &char) -> u8 {
    let val = *item_type as u8;
    // dbg!(item_type, val);
    match val {
        b'a'..=b'z' => val - b'a' + 1,
        b'A'..=b'Z' => val - b'A' + 27u8,
        _ => panic!("Unexpected value: {item_type}"),
    }
}

fn what_the_heck(rucksacks: &str) -> i32 {
    let mut total = 0;

    for line in rucksacks.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left: HashSet<char> = HashSet::from_iter(left.chars());
        let right: HashSet<char> = HashSet::from_iter(right.chars());

        let intersection = left.intersection(&right);

        total += priority(&intersection.into_iter().next().unwrap()) as i32;
    }

    total
}

#[test]
fn example() {
    assert_eq!(what_the_heck(include_str!("../example.txt")), 157);
}

fn main() {
    // println!("Hello, world!");

    // for letter in 'a'..='z' {
    //     dbg!(letter);
    //     dbg!(priority(&letter));
    // }
    // for letter in 'A'..='Z' {
    //     dbg!(letter);
    //     dbg!(priority(&letter));
    // }

    let result = what_the_heck(include_str!("../input.txt"));

    dbg!(result);
}
