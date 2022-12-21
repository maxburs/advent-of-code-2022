fn total_visible(input: &str) -> usize {
    // [y][x]
    let mut board: Vec<Vec<(usize, bool)>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string().parse().unwrap(), false))
                .collect()
        })
        .collect();

    let mut sight_lines: Vec<Box<dyn Iterator<Item = (usize, usize)>>> = vec![];

    for y in 0..board.len() {
        println!("y: {y}");
        sight_lines.push(Box::new((0..board.len()).map(move |x| (x, y))));
        sight_lines.push(Box::new((0..board.len()).rev().map(move |x| (x, y))));
    }

    for x in 0..board[0].len() {
        println!("x: {x}");
        sight_lines.push(Box::new((0..board[0].len()).map(move |y| (x, y))));
        sight_lines.push(Box::new((0..board[0].len()).rev().map(move |y| (x, y))));
    }

    for sight_line in sight_lines {
        let mut tallest: Option<usize> = None;
        for (x, y) in sight_line {
            print!("({x},{y})");
            let height = board[y][x].0;
            if match tallest {
                None => true,
                Some(t) if height > t => true,
                _ => false,
            } {
                tallest = Some(height);
                board[y][x].1 = true;
            }
        }
        println!(" tallest: {:#?} ", tallest);
    }

    println!("{:#?}", board);

    board.iter().flatten().filter(|tree| tree.1).count()
}

#[test]
fn example() {
    assert_eq!(total_visible(include_str!("../example.txt")), 21);
}

fn main() {
    let result = total_visible(include_str!("../input.txt"));
    println!("result: {result}");
}
