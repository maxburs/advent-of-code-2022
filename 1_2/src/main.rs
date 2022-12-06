use std::collections::BinaryHeap;

fn top_3_calories_sum(calories: &str) -> i32 {
    let lines = calories.lines();

    let mut totals: BinaryHeap<i32> = BinaryHeap::new();

    let mut items: Vec<i32> = Vec::new();

    for line in lines {
        if line == "" {
            let sum: i32 = items.iter().sum();
            totals.push(sum);
            items.clear()
        } else {
            items.push(line.parse().unwrap())
        }
    }

    totals.push(items.iter().sum());

    // dbg!(&totals);
    // dbg!(&totals.iter().take(3).collect::<Vec<&i32>>());

    // dbg!(&items);

    totals.pop().unwrap() + totals.pop().unwrap() + totals.pop().unwrap()
}

#[test]
fn test() {
    let example = include_str!("../example.txt");
    assert_eq!(top_3_calories_sum(example), 45_000)
}

fn main() {
    let input = include_str!("../input.txt");
    let top3 = top_3_calories_sum(input);
    println!("top3: {top3}");
}
