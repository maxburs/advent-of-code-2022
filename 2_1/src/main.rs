// Wrong: 13906
#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

fn parse_moves(line: &str) -> (Move, Move) {
    (
        match line.chars().nth(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            letter => panic!("Unexpected letter: {letter}")
        },
        match line.chars().nth(2).unwrap() {
            'Y' => Move::Paper,
            'X' => Move::Rock,
            'Z' => Move::Scissors,
            letter => panic!("Unexpected letter: {letter}")
        },
    )
}

fn calc_score(moves: &str) -> i32 {
    let mut total = 0;
    for line in moves.lines() {
        let (them, us) = parse_moves(line);
        // dbg!((&them, &us));
        total += match us {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        total += match (them, us) {
            (Move::Paper, Move::Paper)
            | (Move::Rock, Move::Rock)
            | (Move::Scissors, Move::Scissors) => 3,
            (Move::Paper, Move::Scissors)
            | (Move::Rock, Move::Paper)
            | (Move::Scissors, Move::Rock) => 6,
            (Move::Paper, Move::Rock)
            | (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper) => 0,
        };
        // dbg!(&total);
    }
    total
}

#[test]
fn example() {
    assert_eq!(calc_score(include_str!("../example.txt")), 15);
}

fn main() {
    let score = calc_score(include_str!("../input.txt"));

    println!("scope: {score}");
}
