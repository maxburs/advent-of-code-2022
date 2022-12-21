use std::collections::HashSet;

struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Knot {
        Knot { x: 0, y: 0 }
    }
}

struct Board {
    head: Knot,
    tail: Vec<Knot>,
}

struct PrintSize {
    width: i32,
    height: i32,
}

impl Board {
    fn print(&self, _visited: &HashSet<(i32, i32)>, print_size: &Option<PrintSize>, header: &str) {
        if let Some(print_size) = print_size {
            println!();
            println!("== {header} ==");
            println!();
            for y in (0..print_size.width).rev() {
                for x in 0..print_size.height {
                    let mut letter: Option<String> = None;

                    if self.head.x == x && self.head.y == y {
                        letter = Some("H".to_string());
                    } else {
                        for (i, knot) in self.tail.iter().enumerate() {
                            if knot.x == x && knot.x == y {
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

fn positions_visited(input: &str, print_size: Option<PrintSize>) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut board = Board {
        head: Knot::new(),
        tail: (0..8).map(|_| Knot::new()).collect(),
    };

    board.print(&visited, &print_size, "== Initial State ==");

    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let amount: i32 = line.next().unwrap().parse().unwrap();
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
                            tail.y += 1;
                            tail.x = prev_x;
                        }
                        prev_x = tail.x;
                        prev_y = tail.y;
                    }
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
            for tail in board.tail.iter() {
                visited.insert((tail.x, tail.y));
            }

            board.print(&visited, &print_size, &format!("{direction} {amount}"));
        }
    }

    visited.len()
}

#[test]
fn example() {
    assert_eq!(positions_visited(include_str!("../example.txt")), 36);
    assert_eq!(positions_visited(include_str!("../example_2.txt")), 36);
}

fn main() {
    let result = positions_visited(include_str!("../input.txt"));
    println!("result: {result}");
}
