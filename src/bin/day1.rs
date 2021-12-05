use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depths = reader
        .lines()
        .filter_map(|line| line.map(|s| s.parse::<u32>().unwrap()).ok());

    let initial_value = (0, depths.next().unwrap());

    let (increases, _) = depths.fold(initial_value, |(count, previous), value| {
        if value > previous {
            (count + 1, value)
        } else {
            (count, value)
        }
    });

    println!("Depth increases: {}", increases);
}
