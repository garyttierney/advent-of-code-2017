use std::fs::File;
use std::io::Read;
use std::io::BufReader;

fn main() {
    let input_file = File::open("data/day1.txt").expect("failed to open input file");

    let mut input_reader = BufReader::new(input_file);
    let mut input = String::new();

    input_reader
        .read_to_string(&mut input)
        .expect("failed to read input");

    let values: Vec<u32> = input
        .chars()
        .map(|ch| {
            ch.to_digit(10).expect("unable to parse number from input")
        })
        .collect();

    let num_values = values.len();
    let step_size = num_values / 2;

    let mut sum = 0;

    for (pos, value) in values.iter().enumerate() {
        let next_value_pos = pos + step_size;
        let next_value = if next_value_pos >= values.len() {
            values[next_value_pos - num_values]
        } else {
            values[next_value_pos]
        };

        if *value == next_value {
            sum = sum + value;
        }
    }

    println!("number is: {}", sum);
}
