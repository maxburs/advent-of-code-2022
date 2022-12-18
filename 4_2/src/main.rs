fn split_on<'a>(text: &'a str, on: &str) -> (&'a str, &'a str) {
    let index = text.find(on).unwrap();
    (text.get(..index).unwrap(), text.get((index + 1)..).unwrap())
}

fn superset_count(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let (left, right) = split_on(line, ",");
        let (left_start, left_end) = split_on(left, "-");
        let (right_start, right_end) = split_on(right, "-");
        let left_start = left_start.parse::<i32>().unwrap();
        let left_end = left_end.parse::<i32>().unwrap();
        let right_start = right_start.parse::<i32>().unwrap();
        let right_end = right_end.parse::<i32>().unwrap();
        // dbg!(left_start, left_end, right_start, right_end);
        if dbg!(!(left_end < right_start || right_end < left_start)) {
            count += 1;
        }
    }
    count
}

#[test]
fn example() {
    assert_eq!(superset_count(include_str!("../example.txt")), 4);
}

fn main() {
    let count = superset_count(include_str!("../input.txt"));
    dbg!(count);
}
