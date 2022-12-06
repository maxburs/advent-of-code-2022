// Wrong: 11918455

fn find_most_calories(calories: &str) -> i32 {
    let lines = calories.lines();

    let mut most = 0;

    let mut items: Vec<i32> = Vec::new();

    for line in lines {
        if line == "" {
            let sum: i32 = items.iter().sum();
            if sum > most {
                most = sum;
            }
            items.clear()
        } else {
            items.push(line.parse().unwrap())
        }
    }

    most
}

#[test]
fn test() {
    let example = include_str!("../example.txt");
    assert_eq!(find_most_calories(example), 24_000)
}

fn main() {
    let input = include_str!("../input.txt");
    let most = find_most_calories(input);
    println!("most: {most}");
}
