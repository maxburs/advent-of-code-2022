static CELL_WIDTH: usize = 2usize;

fn print_map<T>(map: &Vec<Vec<T>>, print_cell: fn(cell: &T) -> String) {
    for row in map {
        println!(
            "|{}",
            row.iter()
                .map(|c| {
                    {
                        let c = print_cell(c);
                        let pad = " ".repeat(CELL_WIDTH - c.len());
                        c + &pad + "|"
                    }
                })
                .collect::<String>()
        );
    }
}

#[derive(Debug)]
struct Input {
    // [y][x]
    height_map: Vec<Vec<usize>>,
    // x, y
    start: (usize, usize),
    end: (usize, usize),
}

fn read_input(input: &str) -> Input {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    let mut map: Vec<Vec<usize>> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<usize> = vec![];
        for (x, letter) in line.chars().enumerate() {
            let height: usize = match letter {
                'S' => {
                    start = Some((x, y));
                    0
                }
                'E' => {
                    end = Some((x, y));
                    25
                }
                l @ 'a'..='z' => (l as usize) - ('a' as usize),
                l => panic!("Unexpected input {l}"),
            };
            row.push(height);
        }
        map.push(row);
    }

    Input {
        height_map: map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn calc_path_to(
    map: &Input,
    distances: &mut Vec<Vec<Option<usize>>>,
    pos: (usize, usize),
    prev_height: usize,
    distance: usize,
) {
    // println!("({}, {})", pos.0, pos.1);

    let new_height = map.height_map[pos.1][pos.0];
    // let mut distance = distance;

    // if prev_height == new_height + 1 {
    //     // distance += 1;
    // } else if prev_height > new_height {
    //     println!("   unable to ascend ({new_height}->{prev_height}");
    //     return;
    // }

    if prev_height > new_height + 1 {
        // println!("   unable to ascend ({new_height}->{prev_height}");
        return;
    }

    if match distances[pos.1][pos.0] {
        Some(d) => d <= distance,
        None => false,
    } {
        // println!("   already visited");
        return;
    }

    distances[pos.1][pos.0] = Some(distance);

    let mut adjacent_positions: Vec<(usize, usize)> = vec![];

    if pos.0 != 0 {
        adjacent_positions.push((pos.0 - 1, pos.1));
    }
    if map.height_map[0].len() - 1 != pos.0 {
        adjacent_positions.push((pos.0 + 1, pos.1));
    }
    if pos.1 != 0 {
        adjacent_positions.push((pos.0, pos.1 - 1));
    }
    if map.height_map.len() - 1 != pos.1 {
        adjacent_positions.push((pos.0, pos.1 + 1));
    }
    for pos in adjacent_positions {
        calc_path_to(map, distances, pos, new_height, distance + 1)
    }
}

fn calc_distances(input: &Input) -> Vec<Vec<Option<usize>>> {
    let mut distances: Vec<Vec<Option<usize>>> = std::iter::repeat_with(|| {
        std::iter::repeat_with(|| None)
            .take(input.height_map[0].len())
            .collect()
    })
    .take(input.height_map.len())
    .collect();

    calc_path_to(input, &mut distances, input.end, 25, 0);

    distances
}

fn start_of_packet_distance(input: &str) -> usize {
    let input = read_input(&input);

    dbg!(input.start);
    dbg!(input.end);

    println!(" === Heights === ");
    print_map::<usize>(&input.height_map, |c| c.to_string());
    println!();

    let distances = calc_distances(&input);

    println!(" === Distances === ");
    // print_map(&distances, |d| match d {
    //     Some(d) => d.to_string(),
    //     None => " ".to_string(),
    // });
    println!();

    distances.iter().enumerate().flat_map(|(y, row)| {
        let height_map = &input.height_map;
        row.iter().enumerate().filter_map(move|(x, d)| {
        if height_map[y][x] != 0 {
            None
        } else {
           *d
        }
    })}).min().unwrap()
}

#[test]
fn example() {
    assert_eq!(start_of_packet_distance(include_str!("../example.txt")), 29);
}

fn main() {
    let result = start_of_packet_distance(include_str!("../input.txt"));
    println!("result: {result}");
}
