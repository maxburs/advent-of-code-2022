#[derive(Debug)]
struct State {
    cycle: isize,
    x: isize,
    image: String,
}

impl State {
    fn step(&mut self) {
        self.cycle += 1;
        let position = (self.cycle - 1) % 40;
        let pixel = if position == self.x || position == self.x + 1 || position == self.x - 1 {
            '#'
        } else {
            '.'
        };
        self.image.push(pixel);
        if self.cycle % 40 == 0 {
            self.image.push('\n');
        }
        dbg!(self.cycle);
        dbg!(self.x);
        println!("{}", &self.image);
        println!();
    }
}

fn run_program(input: &str) -> State {
    let mut state = State {
        cycle: 0,
        x: 1,
        image: String::new(),
    };

    for line in input.lines() {
        let line: Vec<&str> = line.split(" ").collect();

        match line.as_slice() {
            ["noop"] => {
                state.step();
            }
            ["addx", amount] => {
                let amount: isize = amount.parse().unwrap();
                state.step();
                state.step();
                state.x += amount;
            }
            _ => panic!(),
        }
    }

    state
}

#[test]
fn example() {
    assert_eq!(
        run_program(include_str!("../example.txt")).image,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    );
}

fn main() {
    let result = run_program(include_str!("../input.txt")).image;
    println!("result: {result}");
}
