fn start_of_packet_distance(input: &str) -> usize {
    unimplemented!()
}

#[test]
fn example() {
    assert_eq!(start_of_packet_distance(include_str!("../example.txt")), 19);
}

fn main() {
    let result = start_of_packet_distance(include_str!("../input.txt"));
    println!("result: {result}");
}
