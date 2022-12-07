use std::collections::HashSet;

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

    let mut lines = rucksacks.lines();

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let b_set: HashSet<char> = HashSet::from_iter(b.chars());
        let c_set: HashSet<char> = HashSet::from_iter(c.chars());
        let intersection = a
            .chars()
            .filter(|item| b_set.contains(item) && c_set.contains(item));

        total += priority(&intersection.into_iter().next().unwrap()) as i32;
    }

    total
}

#[test]
fn example() {
    assert_eq!(what_the_heck(include_str!("../example.txt")), 70);
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
