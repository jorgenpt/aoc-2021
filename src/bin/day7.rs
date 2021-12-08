use std::{ffi::OsStr, fs::File, io::Read, path::Path};

use itertools::Itertools;

fn main() {
    let mut horizontal_positions = {
        let file_name = Path::new(file!())
            .file_stem()
            .map(OsStr::to_str)
            .flatten()
            .unwrap();
        let mut file = File::open(format!("{}.txt", file_name)).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
            .split(",")
            .map(str::parse::<i32>)
            .flatten()
            .collect::<Vec<_>>()
    };

    horizontal_positions.sort();
    let horizontal_positions = horizontal_positions;

    let median = if horizontal_positions.len() % 2 != 0 {
        (horizontal_positions[horizontal_positions.len() / 2]
            + horizontal_positions[horizontal_positions.len() / 2 + 1])
            / 2
    } else {
        horizontal_positions[horizontal_positions.len() / 2]
    };

    let cost = horizontal_positions
        .iter()
        .map(|x| (x - median).abs())
        .sum::<i32>();

    println!("Cost to align to {}: {}", median, cost);

    let brute_force_options =
        horizontal_positions[0]..=horizontal_positions[horizontal_positions.len() - 1];
    let costs = brute_force_options.map(|position| {
        horizontal_positions
            .iter()
            .map(|x| {
                let num_moves = (x - position).abs();
                (num_moves * (num_moves + 1)) / 2
            })
            .sum::<i32>()
    });

    let min_cost = costs.min().unwrap();

    println!("Cost to align (part 2): {}", min_cost);
}
