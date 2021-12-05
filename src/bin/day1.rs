use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("day1.txt").unwrap();
    let reader = BufReader::new(file);

    let depths = reader
        .lines()
        .filter_map(|line| line.map(|s| s.parse::<u32>().unwrap()).ok())
        .collect::<Vec<_>>();

    let count_increases = |(count, previous), value| {
        if value > previous {
            (count + 1, value)
        } else {
            (count, value)
        }
    };

    let initial_value = (0, &depths[0]);
    let (increases, _) = depths.iter().skip(1).fold(initial_value, count_increases);
    println!("Depth increases: {}", increases);

    let sliding_windows = (0..depths.len() - 2)
        .map(|index| depths[index] + depths[index + 1] + depths[index + 2])
        .collect::<Vec<_>>();

    let initial_value = (0, &sliding_windows[0]);
    let (increases, _) = sliding_windows
        .iter()
        .skip(1)
        .fold(initial_value, count_increases);
    println!("Depth increases (sliding window): {}", increases);
}
