#[derive(Debug)]
struct Tree {
    height: usize,
    score: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn sight_lines(
    forest: &Vec<Vec<Tree>>,
) -> Vec<(Direction, Box<dyn Iterator<Item = (usize, usize)>>)> {
    let mut sight_lines: Vec<(Direction, Box<dyn Iterator<Item = (usize, usize)>>)> = vec![];

    for y in 0..forest.len() {
        println!("y: {y}");
        sight_lines.push((
            Direction::Right,
            Box::new((0..forest.len()).map(move |x| (x, y))),
        ));
        sight_lines.push((
            Direction::Left,
            Box::new((0..forest.len()).rev().map(move |x| (x, y))),
        ));
    }

    for x in 0..forest[0].len() {
        println!("x: {x}");
        sight_lines.push((
            Direction::Down,
            Box::new((0..forest[0].len()).map(move |y| (x, y))),
        ));
        sight_lines.push((
            Direction::Up,
            Box::new((0..forest[0].len()).rev().map(move |y| (x, y))),
        ));
    }

    sight_lines
}

#[derive(Debug)]
struct VisibleTree {
    x: usize,
    y: usize,
    height: usize,
}

fn total_visible(input: &str) -> usize {
    // [y][x]
    let mut forest: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree {
                    height: c.to_string().parse().unwrap(),
                    score: 1,
                })
                .collect()
        })
        .collect();

    let width = forest[0].len();
    let height = forest.len();

    for (direction, line) in sight_lines(&forest) {
        let mut visible_trees: Vec<VisibleTree> = vec![];
        for (x, y) in line {
            let tree = &mut forest[y][x];
            println!("({x},{y})");
            let blocking_tree = visible_trees
                .iter()
                .enumerate()
                .rev()
                .find(|t| t.1.height >= tree.height);
            // dbg!(&direction, &tree, &blocking_tree);
            let view_distance = match blocking_tree {
                Some(t2) => {
                    let view_distance = match direction {
                        Direction::Up => t2.1.y - y,
                        Direction::Down => y - t2.1.y,
                        Direction::Left => t2.1.x - x,
                        Direction::Right => x - t2.1.x,
                    };
                    if t2.1.height == tree.height {
                        // println!("clear1");
                        visible_trees.clear();
                    } else {
                        // dbg!(t2.0);
                        visible_trees.truncate(dbg!(t2.0 + 1));
                    }
                    view_distance
                }
                None => {
                    println!("clear2");
                    visible_trees.clear();
                    match direction {
                        Direction::Up => (height - 1) - y,
                        Direction::Down => y,
                        Direction::Left => (width - 1) - x,
                        Direction::Right => x,
                    }
                }
            };
            visible_trees.push(VisibleTree {
                height: tree.height,
                x,
                y,
            });
            tree.score *= view_distance;
            println!("view_distance: {view_distance}, score: {:#?}, visible: {:#?}", tree.score, &visible_trees);
            println!();
        }
    }

    println!("{:#?}", forest);

    let best = forest
        .iter()
        .enumerate()
        .map(|(y, column)| column.iter().enumerate().map(move |(x, tree)| (x, y, tree)))
        .flatten()
        .max_by_key(|tree| tree.2.score)
        .unwrap();

    println!("best: {:#?}", best);

    best.2.score
}

#[test]
fn example() {
    assert_eq!(total_visible(include_str!("../example.txt")), 8);
}

fn main() {
    let result = total_visible(include_str!("../input.txt"));
    println!("result: {result}");
}
