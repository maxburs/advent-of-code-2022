use std::collections::HashSet;

#[derive(Debug)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Knot {
        Knot { x: 0, y: 0 }
    }
    fn follow(&mut self, head: &Knot) {
        let x_diff = self.x - head.x;
        if x_diff < 0 {
            self.x -= 1
        } else if x_diff > 0 {
            self.x += 1
        }

        let y_diff = self.y - head.y;
        if y_diff < 0 {
            self.y -= 1
        } else if y_diff > 0 {
            self.y += 1
        }
    }
}

struct Board {
    head: Knot,
    tail: Vec<Knot>,
}

struct PrintSize {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

impl Board {
    fn print(&self, _visited: &HashSet<(i32, i32)>, print_size: &Option<PrintSize>) {
        if let Some(print_size) = print_size {
            println!();
            for y in (print_size.y_start..print_size.y_end).rev() {
                for x in print_size.x_start..print_size.x_end {
                    let mut letter: Option<String> = None;

                    if self.head.x == x && self.head.y == y {
                        letter = Some("H".to_string());
                    } else {
                        for (i, knot) in self.tail.iter().enumerate() {
                            if knot.x == x && knot.y == y {
                                letter = Some((i + 1).to_string());
                                break;
                            }
                        }
                    }
                    if letter.is_none() && x == 0 && y == 0 {
                        letter = Some("s".to_string());
                    }

                    print!("{}", letter.unwrap_or(".".to_string()));
                }
                println!();
            }
        }
    }
}

fn print_visited(print_size: &Option<PrintSize>, visited: &HashSet<(i32, i32)>) {
    if let Some(print_size) = print_size {
        println!();
        for y in (print_size.y_start..print_size.y_end).rev() {
            for x in print_size.x_start..print_size.x_end {
                if visited.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn positions_visited(input: &str, print_size: Option<PrintSize>) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut board = Board {
        head: Knot::new(),
        tail: (0..8).map(|_| Knot::new()).collect(),
    };

    if print_size.is_some() {
        println!();
        println!("== Initial State ==");
        println!();
    }
    board.print(&visited, &print_size);

    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let amount: i32 = line.next().unwrap().parse().unwrap();

        if print_size.is_some() {
            println!();
            println!("== {direction} {amount} ==");
            println!();
        }

        for _ in 0..amount {
            match direction {
                "R" => {
                    board.head.x += 1;
                    let mut prev_x = board.head.x;
                    let mut prev_y = board.head.y;
                    for tail in board.tail.iter_mut() {
                        if prev_x == tail.x + 2 {
                            tail.x += 1;
                            tail.y = prev_y;
                        }
                        prev_x = tail.x;
                        prev_y = tail.y;
                    }
                }
                "L" => {
                    board.head.x -= 1;
                    let mut prev_x = board.head.x;
                    let mut prev_y = board.head.y;
                    for tail in board.tail.iter_mut() {
                        if prev_x == tail.x - 2 {
                            tail.x -= 1;
                            tail.y = prev_y;
                        }
                        prev_x = tail.x;
                        prev_y = tail.y;
                    }
                }
                "U" => {
                    board.head.y += 1;
                    let mut prev_x = board.head.x;
                    let mut prev_y = board.head.y;
                    for tail in board.tail.iter_mut() {
                        if prev_y == tail.y + 2 {
                            dbg!(prev_x, prev_y, &tail);
                            tail.y += 1;
                            tail.x = prev_x;
                        }
                        prev_x = tail.x;
                        prev_y = tail.y;
                    }
                    board.print(&visited, &print_size);
                }
                "D" => {
                    board.head.y -= 1;
                    let mut prev_x = board.head.x;
                    let mut prev_y = board.head.y;
                    for tail in board.tail.iter_mut() {
                        if prev_y == tail.y - 2 {
                            tail.y -= 1;
                            tail.x = prev_x;
                        }
                        prev_x = tail.x;
                        prev_y = tail.y;
                    }
                }
                _ => panic!(),
            };
            let tail = board.tail.last().unwrap();
            visited.insert((tail.x, tail.y));

        }
    }
    print_visited(&print_size, &visited);

    visited.len()
}

#[test]
fn example() {
    // assert_eq!(
    //     positions_visited(
    //         include_str!("../example.txt"),
    //         PrintSize {
    //             width: 6,
    //             height: 5
    //         }
    //         .into(),
    //     ),
    //     1
    // );
    assert_eq!(
        positions_visited(
            include_str!("../example_2.txt"),
            PrintSize {
                x_start: -11,
                x_end: 15,
                y_start: -5,
                y_end: 16,
            }
            .into(),
        ),
        36
    );
}

fn main() {
    let result = positions_visited(include_str!("../input.txt"), None);
    println!("result: {result}");
}
