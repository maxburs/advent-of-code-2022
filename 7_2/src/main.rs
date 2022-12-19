use std::vec;

#[derive(Debug)]
struct Directory {
    name: String,
    content: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug)]
enum Item {
    Directory(Directory),
    File { name: String, size: usize },
}

struct FileSystem {
    items: Vec<Item>,
}

impl FileSystem {
    fn get_dir_mut(&mut self, id: usize) -> &mut Directory {
        match &mut self.items[id] {
            Item::Directory(d) => d,
            Item::File { .. } => panic!("Invalid dir id {id}"),
        }
    }
    fn get_dir(&self, id: usize) -> &Directory {
        match &self.items[id] {
            Item::Directory(d) => d,
            Item::File { .. } => panic!("Invalid dir id {id}"),
        }
    }

    fn add_dir(&mut self, parent_dir_id: usize, name: String) -> usize {
        let item = Item::Directory(Directory {
            name,
            content: vec![],
            parent: Some(parent_dir_id),
        });
        self.items.push(item);
        let id = self.items.len() - 1;
        let parent = self.get_dir_mut(parent_dir_id);
        parent.content.push(id);
        id
    }

    fn add_file(&mut self, parent_dir_id: usize, name: String, size: usize) -> usize {
        self.items.push(Item::File { name, size });

        let id = self.items.len() - 1;
        let parent = self.get_dir_mut(parent_dir_id);

        parent.content.push(id);
        id
    }

    fn parent(&self, dir_id: usize) -> usize {
        self.get_dir(dir_id).parent.unwrap()
    }

    fn print(&self) {
        println!(" - / (dir)");
        self.print_dir(0, 0);
    }
    fn print_dir(&self, depth: usize, dir_id: usize) {
        for item_id in self.get_dir(dir_id).content.as_slice() {
            for _ in 0..(depth + 1) {
                print!("  ");
            }
            let item: &Item = &self.items[*item_id];
            match item {
                Item::File { name, size } => println!(" - {name} (file, size={size})"),
                Item::Directory(Directory { name, .. }) => {
                    println!(" - {name} (dir)");
                    self.print_dir(depth + 1, *item_id);
                }
            }
        }
    }
}

fn build_filesystem(input: &str) -> FileSystem {
    let mut file_system = FileSystem {
        items: vec![Item::Directory(Directory {
            name: "/".to_string(),
            content: vec![],
            parent: None,
        })],
    };

    let mut position = 0usize;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line.as_slice() {
            ["$", "cd", ".."] => position = file_system.parent(position),
            ["$", "cd", "/"] => position = 0,
            ["$", "cd", target_name] => {
                let dir: &Directory = file_system.get_dir(position);
                position = *dir
                    .content
                    .iter()
                    .find(|item_id| {
                        if let Item::Directory(Directory { name, .. }) =
                            &file_system.items[**item_id]
                        {
                            name == target_name
                        } else {
                            false
                        }
                    })
                    .unwrap_or_else(|| {
                        panic!("Unable to find directory {} in {}", dir.name, target_name)
                    })
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                file_system.add_dir(position, name.to_string());
            }
            [size, name] => {
                let size: usize = size.parse().unwrap();
                file_system.add_file(position, name.to_string(), size);
            }
            line => panic!("Unexpected line: {:?}", line),
        }
    }

    file_system
}

fn total_sizes(
    file_system: &FileSystem,
    dir_id: usize,
    target_size: usize,
) -> (usize, Option<usize>) {
    let mut dir_size = 0usize;
    let mut smallest_found: Option<usize> = None;

    for item_id in file_system.get_dir(dir_id).content.iter() {
        match file_system.items[*item_id] {
            Item::File { size, .. } => dir_size += size,
            Item::Directory(_) => {
                let sizes = total_sizes(file_system, *item_id, target_size);
                dir_size += sizes.0;
                let replace = match (smallest_found, sizes.1) {
                    (Some(a), Some(b)) => a > b,
                    (None, Some(_)) => true,
                    _ => false,
                };
                if replace {
                    smallest_found = sizes.1;
                }
            }
        }
        dbg!(&file_system.items[*item_id], dir_size);
    }

    if smallest_found.is_none() && dir_size >= target_size {
        smallest_found = Some(dir_size);
    }

    dbg!(dir_size, smallest_found);

    (dir_size, smallest_found)
}

fn run(input: &str) -> usize {
    let file_system = build_filesystem(input);

    file_system.print();

    let total_size = total_sizes(&file_system, 0, 0).0;

    let target_space_used = 70000000 - 30000000;

    total_sizes(&file_system, 0, total_size - target_space_used)
        .1
        .unwrap()
}

#[test]
fn example_total_size() {
    assert_eq!(run(include_str!("../example.txt")), 24933642);
}

fn main() {
    let result = run(include_str!("../input.txt"));
    println!("result: {result}");
}
