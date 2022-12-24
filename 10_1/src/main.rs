struct State {
    cycle: isize,
    total_strength: isize,
    x: isize,
}

impl State {
    fn step(&mut self) {
        self.cycle += 1;
        if (self.cycle + 20) % 40 == 0 {
            self.total_strength += self.x * self.cycle;
        }
    }
}

fn run_program(input: &str) -> State {
    let mut state = State { cycle: 0, total_strength: 0, x: 1 };

    for line in input.lines() {
        let line: Vec<&str> = line.split(" ").collect();

        match line.as_slice() {
            ["noop"] => {
                state.step();
            },
            ["addx", amount] => {
                let amount: isize = amount.parse().unwrap();
                state.step();
                state.step();
                state.x += amount;
            },
            _ => panic!(),
        }
    }

    state
}

#[test]
fn example() {
    assert_eq!(run_program(include_str!("../example.txt")).total_strength, 13140);
}

fn main() {
    let result = run_program(include_str!("../input.txt")).total_strength;
    println!("result: {result}");
}
