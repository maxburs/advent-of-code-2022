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

fn solve(input: &str, target_row: isize) -> usize {
    let sensors = parse_input(input);
    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;

    for s in sensors.iter() {
        let min = s.coordinate.x - s.beacon_distance;
        if min < x_min {
            x_min = min;
        }
        let max = s.coordinate.x + s.beacon_distance;
        if max > x_max {
            x_max = max
        }
    }
    dbg!(&sensors);

    dbg!(x_min, x_max);

    let mut count = 0;

    for x in x_min..=x_max {
        let c = Coordinate { x, y: target_row };
        if sensors
            .iter()
            .any(|s| c.distance(&s.coordinate) <= s.coordinate.distance(&s.closest_beacon))
            && sensors
                .iter()
                .all(|s| c != s.coordinate && c != s.closest_beacon)
        {
            count += 1;
            // dbg!(&c, count);
        }
    }

    count
}

// too high: 5068328
5040643
#[test]
fn example() {
    assert_eq!(solve(include_str!("../example.txt"), 20), 26);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = solve(include_str!("../input.txt"), 2000000);
    println!("result: {result}");

    Ok(())
}
