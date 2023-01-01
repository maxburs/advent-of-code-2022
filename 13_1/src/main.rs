use either::Either::{self, Left, Right};

#[derive(Clone)]
struct List(Vec<Either<usize, Box<List>>>);
type Packet = List;

struct Input {
    pairs: Vec<(Packet, Packet)>,
}

#[derive(PartialEq, Debug)]
enum Sorted {
    True,
    TrueFinal,
    False,
}

fn parse_digit(input: &mut std::iter::Peekable<std::str::Chars>) -> usize {
    let mut chars: Vec<char> = vec![];

    while let Some(v) = input.next_if(|c| c.is_ascii_digit()) {
        chars.push(v);
    }

    chars.iter().collect::<String>().parse().unwrap()
}

fn parse_list(input: &mut std::iter::Peekable<std::str::Chars>) -> List {
    input.next();
    let mut list = List(vec![]);
    'outer: loop {
        let next = input.peek().unwrap();
        dbg!(next);
        match next {
            ']' => {
                input.next();
                break 'outer;
            }
            ',' => {
                input.next();
            }
            '[' => {
                list.0.push(either::Right(Box::new(parse_list(input))));
            }
            c => {
                if c.is_ascii_digit() {
                    list.0.push(either::Left(parse_digit(input)));
                } else {
                    panic!("Unexpected {c}");
                }
            }
        };
    }

    list
}

fn parse_input(input: &str) -> Input {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_list(&mut l.chars().peekable()));

    let mut input = Input { pairs: vec![] };

    while let (Some(p1), Some(p2)) = (packets.next(), packets.next()) {
        input.pairs.push((p1, p2));
    }

    input
}

fn are_list_items_ordered(a: &Either<usize, Box<List>>, b: &Either<usize, Box<List>>) -> Sorted {
    if let (Left(a), Left(b)) = (a, b) {
        return if a == b {
            Sorted::True
        } else if a > b {
            Sorted::False
        } else {
            Sorted::TrueFinal
        };
    }

    let a: List = match a {
        Left(_) => List(vec![a.clone()]),
        Right(a) => a.as_ref().clone(),
    };

    let b: List = match b {
        Left(_) => List(vec![b.clone()]),
        Right(b) => b.as_ref().clone(),
    };

    are_lists_ordered(&a, &b)
}

fn are_lists_ordered(a: &List, b: &List) -> Sorted {
    for (i, left) in a.0.iter().enumerate() {
        let right = b.0.get(i);
        match right {
            Some(right) => {
                let sorted = are_list_items_ordered(&left, &right);
                if sorted == Sorted::False || sorted == Sorted::TrueFinal {
                    return sorted;
                }
            }
            None => return Sorted::False,
        }
    }

    if a.0.len() == b.0.len() {
        Sorted::True
    } else {
        Sorted::TrueFinal
    }
}

fn solve(input: &str) -> usize {
    let input = parse_input(&input);

    let mut total = 0;

    for (i, (left, right)) in input.pairs.iter().enumerate() {
        let ordered = are_lists_ordered(&left, &right);
        let index = i + 1;
        // dbg!(index, &ordered);
        if ordered != Sorted::False {
            total += index;
        }
    }

    total
}

#[test]
fn example() {
    assert_eq!(solve(include_str!("../example.txt")), 13);
}

fn main() {
    let result = solve(include_str!("../input.txt"));
    println!("result: {result}");
}
