use std::{collections::HashMap, fmt::Formatter};

mod parse;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl std::fmt::Debug for Coordinate {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "({},{})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq)]
enum InnerCaveItem {
    Rock,
    Sand,
}

struct Cave {
    items: HashMap<Coordinate, InnerCaveItem>,
    x_min: isize,
    x_max: isize,
    y_max: isize,
}

impl Cave {
    fn get(&self, c: &Coordinate) -> CaveItem {
        match self.items.get(c) {
            None => CaveItem::Air,
            Some(InnerCaveItem::Rock) => CaveItem::Rock,
            Some(InnerCaveItem::Sand) => CaveItem::Sand,
        }
    }
    fn set(&mut self, c: Coordinate, item: CaveItem) {
        if item == CaveItem::Rock && c.y > self.y_max {
            self.y_max = c.y;
        }

        if c.x > self.x_max {
            self.x_max = c.x;
        }
        if c.x < self.x_min {
            self.x_min = c.x;
        }
        match item {
            CaveItem::Air => self.items.remove(&c),
            CaveItem::Rock => self.items.insert(c, InnerCaveItem::Rock),
            CaveItem::Sand => self.items.insert(c, InnerCaveItem::Sand),
        };
    }
    fn print(&self) {
        println!("x_min: {}, x_max: {}, y_max: {}", self.x_min, self.x_min, self.y_max);
        for y in 0..=self.y_max + 1 {
            // print!("{}", y);
            for x in self.x_min..=self.x_max {
                let char = match self.get(&Coordinate { x, y }) {
                    CaveItem::Air => '.',
                    CaveItem::Rock => '#',
                    CaveItem::Sand => 'o',
                };
                print!("{}", char);
            }
            println!();
        }
        // print!(" ");
        for _ in self.x_min..=self.x_max {
            print!("#");
        }
        println!();
    }
    fn add_sand(&mut self, c: Coordinate) -> bool {
        if self.get(&c) != CaveItem::Air {
            return false;
        }

        if c.y + 1 < self.y_max + 2 {
            for x_offset in [0, -1, 1] {
                let c2 = Coordinate {
                    x: c.x + x_offset,
                    y: c.y + 1,
                };
                if self.get(&c2) == CaveItem::Air {
                    return self.add_sand(c2);
                }
            }
        }

        self.set(c, CaveItem::Sand);
        true
    }
}

#[derive(PartialEq)]
enum CaveItem {
    Rock,
    Air,
    Sand,
}

fn parse_coordinate(input: &mut parse::Cursor) -> Option<Coordinate> {
    let x = parse::int(input)?;
    parse::expect(input, &",")?;
    let y = parse::int(input)?;
    Some(Coordinate {
        x: x.try_into().unwrap(),
        y: y.try_into().unwrap(),
    })
}

fn parse_input(input: &str) -> Cave {
    let mut cursor = input.chars().peekable();
    let paths = parse::list(&mut cursor, |c| {
        let c1 = parse_coordinate(c)?;
        let mut rest = parse::list(c, |c| {
            parse::expect(c, &" -> ")?;
            parse_coordinate(c)
        });
        parse::expect(c, &"\n")?;
        rest.insert(0, c1);

        Some(rest)
    });

    let mut cave = Cave {
        items: HashMap::new(),
        y_max: isize::MIN,
        x_min: isize::MAX,
        x_max: isize::MIN,
    };

    dbg!(&paths);

    for path in paths {
        let mut path = path.iter();
        let mut prev = path.next().unwrap();
        for next in path {
            println!("prev: {:#?}, next: {:#?}", &prev, &next);
            cave.set(prev.clone(), CaveItem::Rock);
            if prev.x == next.x {
                let (start, end) = if prev.y > next.y {
                    (next.y, prev.y - 1)
                } else {
                    (prev.y + 1, next.y)
                };
                for y in start..=end {
                    cave.set(Coordinate { x: prev.x, y }, CaveItem::Rock);
                }
            } else {
                let (start, end) = if prev.x > next.x {
                    (next.x, prev.x - 1)
                } else {
                    (prev.x + 1, next.x)
                };
                for x in start..=end {
                    cave.set(Coordinate { x, y: prev.y }, CaveItem::Rock);
                }
            }
            prev = next;
        }
        cave.set(prev.clone(), CaveItem::Rock);
    }

    cave
}

fn solve(input: &str) -> usize {
    let mut cave = parse_input(input);
    println!(" ===Initial State===");
    cave.print();
    println!();
    while cave.add_sand(Coordinate { x: 500, y: 0 }) {}
    cave.print();
    cave.items
        .iter()
        .filter(|(_, item)| **item == InnerCaveItem::Sand)
        .count()
}

#[test]
fn example() {
    assert_eq!(solve(include_str!("../example.txt")), 93);
}

fn main() {
    let result = solve(include_str!("../input.txt"));
    println!("result: {result}");
}
