use itertools::Itertools;


fn start_of_marker(stream: &[u8], length: usize) -> usize {
    length + stream
        .windows(length)
        .position(|window| window
            .iter()
            .tuple_combinations::<(&u8, &u8)>()
            .all(|(a, b)| a != b))
            .unwrap()
}


fn main() {
    let raw_input = include_bytes!("../../inputs/day06.in");

    let start_of_packet_marker = start_of_marker(raw_input, 4);

    let start_of_message_marker = start_of_marker(raw_input, 14);

    println!("PT1: {}\nPT2: {}", start_of_packet_marker, start_of_message_marker);
}