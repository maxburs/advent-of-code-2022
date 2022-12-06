// Wrong: 13906
#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_move(letter: &char) -> Move {
    match letter {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        letter => panic!("Unexpected letter: {letter}"),
    }
}

fn calc_score(moves: &str) -> i32 {
    let mut total = 0;
    for line in moves.lines() {
        let them = parse_move(&line.chars().nth(0).unwrap());
        let us = line.chars().nth(2).unwrap();
        // dbg!((&them, &us));
        total += match us {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!(),
        };
        total += match (them, us) {
            // Rock
            (Move::Paper, 'X') | (Move::Rock, 'Y') | (Move::Scissors, 'Z') => 1,
            // Paper
            (Move::Scissors, 'X') | (Move::Paper, 'Y') | (Move::Rock, 'Z') => 2,
            // Scissors
            (Move::Rock, 'X') | (Move::Scissors, 'Y') | (Move::Paper, 'Z') => 3,
            _ => panic!(),
        };
        // dbg!(&total);
    }
    total
}

#[test]
fn example() {
    assert_eq!(calc_score(include_str!("../example.txt")), 12);
}

fn main() {
    let score = calc_score(include_str!("../input.txt"));

    println!("scope: {score}");
}
