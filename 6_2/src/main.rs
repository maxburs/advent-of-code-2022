fn start_of_packet_distance(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

     'outer: for i in 0..(chars.len() - 15) {
        for j in 0..13 {
            for k in (j + 1)..14 {
                if chars[i + j] == chars[i + k] {
                    continue 'outer;
                }
            }
        }
        return i + 14;
    }

    panic!()
}

#[test]
fn example() {
    assert_eq!(start_of_packet_distance("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(start_of_packet_distance("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(
        start_of_packet_distance("nppdvjthqldpwncqszvftbrmjlhg"),
        23,
    );
    assert_eq!(
        start_of_packet_distance("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        29,
    );
    assert_eq!(
        start_of_packet_distance("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        26,
    );
}

fn main() {
    let length = start_of_packet_distance(include_str!("../input.txt"));
    println!("length: {length}");
}
