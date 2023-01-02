mod parse;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl std::fmt::Debug for Coordinate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "({},{})", self.x, self.y)
    }
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    coordinate: Coordinate,
    closest_beacon: Coordinate,
    beacon_distance: isize,
}

fn parse_coordinate(cursor: &mut parse::Cursor) -> parse::ParserResult<Coordinate> {
    parse::expect(cursor, &"x=")?;
    let x = parse::singed_int(cursor)?;
    parse::expect(cursor, &", y=")?;
    let y = parse::singed_int(cursor)?;
    Ok(Coordinate { x, y })
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_sensor(cursor: &mut parse::Cursor) -> parse::ParserResult<Sensor> {
    parse::expect(cursor, &"Sensor at ")?;
    let coordinate = parse_coordinate(cursor)?;
    parse::expect(cursor, &": closest beacon is at ")?;
    let closest_beacon = parse_coordinate(cursor)?;
    Ok(Sensor {
        beacon_distance: coordinate.distance(&closest_beacon),
        coordinate,
        closest_beacon,
    })
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .try_fold(vec![], |mut acc, line| {
            acc.push(parse_sensor(&mut line.chars().peekable())?);
            Ok::<Vec<Sensor>, Box<dyn std::error::Error>>(acc)
        })
        .unwrap()
}

fn solve(input: &str, search_size: isize) -> isize {
    let sensors = parse_input(input);

    dbg!(&sensors);

    let mut y = 0isize;

    while y <= search_size {
        let mut x = 0isize;
        while x <= search_size {
            let c = Coordinate { x, y };
            // dbg!(&c);
            let sensor = sensors.iter().find(|s| {
                if c.distance(&s.coordinate) <= s.beacon_distance {
                    true
                } else {
                    // dbg!(s);
                    false
                }
            });
            if let Some(sensor) = sensor {
                x = sensor.beacon_distance - (sensor.coordinate.y - y).abs() + sensor.coordinate.x + 1;
                // dbg!(x, sensor);
                // println!();
            } else {
                return c.x * 4000000 + c.y;
            }
        }
        y += 1;
    }

    panic!()
}

#[test]
fn example() {
    assert_eq!(solve(include_str!("../example.txt"), 20), 56000011);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = solve(include_str!("../input.txt"), 4000000);
    println!("result: {result}");

    Ok(())
}
