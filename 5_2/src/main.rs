use regex::Regex;

fn init_stacks(input: &str) -> Vec<Vec<char>> {
    let mut input = input.lines().rev();

    // Remove legend

    let mut stacks: Vec<Vec<char>> = (0..(input.next().unwrap().len() + 1) / 4)
        .map(|_| vec![])
        .collect();

    let crates = input.map(|row| {
        let mut row = row.chars().into_iter();
        let mut crate_row: Vec<char> = vec![];

        row.next();
        crate_row.push(row.next().unwrap());
        row.next();

        while row.next().is_some() {
            row.next();
            crate_row.push(row.next().unwrap());
            row.next();
        }

        crate_row
    });

    for row in crates {
        for (i, c) in row.into_iter().enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

fn print_crates(crates: &Vec<Vec<char>>) {
    let height: usize = crates.iter().map(|c| c.len()).max().unwrap();
    for i in (0..height).rev() {
        for (j, stack) in crates.iter().enumerate() {
            if j != 0 {
                print!(" ");
            }
            if let Some(c) = stack.get(i) {
                print!("[{}]", c);
            } else {
                print!("   ");
            }
        }
        println!();
    }
    for i in 0..crates.len() {
        if i != 0 {
            print!(" ");
        }
        print!(" {} ", i + 1);
    }
    println!();
}

fn stacks_top(input: &str) -> String {
    let mut input = input.split("\n\n");
    let mut stacks = init_stacks(input.next().unwrap());

    let move_regex: regex::Regex = Regex::new(r"[^\d]+(\d+)[^\d]+(\d+)[^\d]+(\d+)").unwrap();

    print_crates(&stacks);
    println!();

    for line in input.next().unwrap().lines() {
        // let amount: usize = line.get(5..6).unwrap().parse().unwrap();
        // let from: usize = line.get(12..13).unwrap().parse().unwrap();
        // let to: usize = line.get(17..18).unwrap().parse().unwrap();

        let matches = move_regex.captures(line).unwrap();
        // dbg!([matches.get(0).unwrap().as_str(), matches.get(1).unwrap().as_str(), matches.get(2).unwrap().as_str()]);
        let matches = [matches.get(1), matches.get(2), matches.get(3)]
            .iter()
            .map(|num| num.unwrap().as_str().parse().unwrap())
            .collect::<Vec<usize>>();
        if let [amount, from, to] = &matches[..] {
            // if *from == 5 || *to == 5 {
            // println!();
            // }
            // for _ in 0..*amount {
            let from_stack = &mut stacks[from - 1];
            // dbg!((from_stack.len() - amount)..);
            let mut c: Vec<char> = from_stack.drain((from_stack.len() - amount)..).collect();
            stacks[to - 1].append(&mut c);
            // } else {
            //     println!("{}", line);
            //     panic!();
            // }
            // }
            // if *from == 5 || *to == 5 {
            print_crates(&stacks);
            // println!("{}", line);
            println!("move {} from {} to {}", amount, from, to);
            // println!("len: {}", stacks[4].len());
            println!();
            // }
        } else {
            panic!();
        }
        // print_crates(&stacks);
    }

    let mut res: Vec<char> = vec![];

    for column in stacks {
        if let Some(c) = column.last() {
            res.push(*c);
        }
    }

    res.iter().collect()
}

#[test]
fn example() {
    assert_eq!(stacks_top(include_str!("../example.txt")), "MCD");
}

fn main() {
    let count = stacks_top(include_str!("../input.txt"));
    dbg!(count);
}
