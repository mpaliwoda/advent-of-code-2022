use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day6/input.txt").expect("caveman brain forgot");

    let input = input.trim();
    let chars: Vec<char> = input.chars().collect();

    println!("Part 1: {}", find_marker(&chars, 4));
    println!("Part 1: {}", find_marker(&chars, 14));
}

fn find_marker(buffer: &Vec<char>, marker_len: usize) -> usize {
    let mut marker_ix: usize = marker_len;

    let mut start: usize = 0;
    let mut end: usize = 1;

    let mut seq: Vec<&char> = Vec::new();

    seq.push(&buffer.iter().next().unwrap());

    while start < buffer.len() - marker_len && end < buffer.len() {
        if seq.len() == marker_len {
            marker_ix = end;
            break;
        }

        let current = &buffer[end];

        if seq.contains(&current) {
            seq.clear();
            start = start + 1;
            end = start + 1;
            seq.push(&buffer[start]);
            continue;
        }

        end += 1;
        seq.push(&current);
    }

    return marker_ix;
}
