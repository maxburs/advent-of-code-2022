use std::cmp::Ordering;
use std::fmt::Debug;
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("inspections", &self.inspections)
            .field("items", &self.items)
            .finish()
    }
}

enum OperationValue {
    Val(usize),
    Old,
}

impl OperationValue {
    fn eval(&self, old: usize) -> usize {
        match self {
            OperationValue::Old => old,
            OperationValue::Val(val) => *val,
        }
    }
}

#[derive(Debug)]
struct State {
    monkeys: Vec<Monkey>,
}

impl State {
    fn run_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let items: Vec<usize> = self.monkeys[i].items.drain(..).collect();
            for item in items {
                let item = self.monkeys[i].operation.as_ref()(item);
                let item = item / 3;

                let next = self.monkeys[i].test.as_ref()(item);

                self.monkeys[next].items.push(item);

                self.monkeys[i].inspections += 1;
            }
        }
    }
}

fn parse_operation_value(val: &str) -> OperationValue {
    if val == "old" {
        OperationValue::Old
    } else {
        OperationValue::Val(val.parse().unwrap())
    }
}

fn get_eol_num(lines: &mut dyn Iterator<Item = &str>) -> usize {
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn init_monkeys(input: &str) -> State {
    let mut lines = input.lines().filter(|l| !l.is_empty() && !l.starts_with("Monkey"));

    let mut monkeys: Vec<Monkey> = vec![];

    loop {
        let items = match lines.next() {
            None => break,
            Some(l) => l,
        };

        dbg!(&items, items.starts_with("Monkey"));

        let items = items
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .collect::<Vec<&str>>();

        dbg!(&items);

        let items: Vec<usize> = items.iter().map(|item| item.parse().unwrap()).collect();

        let mut operation = lines.next().unwrap().split_once("= ").unwrap().1.split(" ");

        let left = operation.next().unwrap();
        let operator = operation.next().unwrap();
        let right = operation.next().unwrap();

        dbg!(&left, &operator, &right);

        let operator: fn(a: usize, b: usize) -> usize = match operator {
            "+" => Add::add,
            "-" => Sub::sub,
            "/" => Div::div,
            "*" => Mul::mul,
            op => panic!("Unexpected operator {op}"),
        };

        let left = parse_operation_value(&left);
        let right = parse_operation_value(&right);

        let operation = move |item: usize| -> usize {
            let left = left.eval(item);
            let right = right.eval(item);
            operator(left, right)
        };

        let divisible_by: usize = get_eol_num(&mut lines);
        let if_true: usize = get_eol_num(&mut lines);
        let if_false: usize = get_eol_num(&mut lines);

        let test = move |item: usize| -> usize {
            if item % divisible_by == 0 {
                if_true
            } else {
                if_false
            }
        };

        dbg!(format!("monkey #{}", monkeys.len()));

        monkeys.push(Monkey {
            items,
            operation: Box::new(operation),
            test: Box::new(test),
            inspections: 0,
        })
    }

    State { monkeys }
}

// After round 1, the monkeys are holding items with these worry levels:

// Monkey 0: 20, 23, 27, 26
// Monkey 1: 2080, 25, 167, 207, 401, 1046
// Monkey 2:
// Monkey 3:
#[test]
fn example1() {
    let mut monkeys = init_monkeys(include_str!("../example.txt"));

    dbg!(&monkeys);

    monkeys.run_round();

    dbg!(&monkeys);

    assert_eq!(&monkeys.monkeys[0].items, &[20, 23, 27, 26]);
    assert_eq!(&monkeys.monkeys[1].items, &[2080, 25, 167, 207, 401, 1046]);

    // assert_eq!(calc(include_str!("../example.txt")), 19);
}

#[test]
fn example2() {
    let mut monkeys = init_monkeys(include_str!("../example.txt"));

    for _ in 0..20 {
        monkeys.run_round()
    }

    assert_eq!(
        &monkeys
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<usize>>(),
        &[101, 95, 7, 105]
    );
}

fn main() {
    let mut monkeys = init_monkeys(include_str!("../input.txt"));

    for _ in 0..20 {
        monkeys.run_round()
    }

    let mut inspections = monkeys
        .monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_by(|a, b| b.cmp(a));

    dbg!(&inspections);

    let monkey_business = inspections[0] * inspections[1];

    println!("monkey_business: {monkey_business}");
}
