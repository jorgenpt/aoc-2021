use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let notes = {
        let file_name = Path::new(file!())
            .file_stem()
            .map(OsStr::to_str)
            .flatten()
            .unwrap();
        let reader = BufReader::new(File::open(format!("{}.txt", file_name)).unwrap());

        reader.lines().flatten().map(|line| {
            let sort_letters = |s: &str| s.chars().sorted().collect::<String>();

            let (digits, output) = line.split(" | ").collect_tuple().unwrap();
            (
                digits
                    .split_whitespace()
                    .map(&sort_letters)
                    .collect::<Vec<_>>(),
                output
                    .split_whitespace()
                    .map(&sort_letters)
                    .collect::<Vec<_>>(),
            )
        })
    };

    let num_1478_appearances = notes
        .map(|(digits, output)| {
            let lookup = digits
                .into_iter()
                .map(|digit| match digit.len() {
                    2 => Some((digit, 1)),
                    3 => Some((digit, 7)),
                    4 => Some((digit, 4)),
                    7 => Some((digit, 8)),
                    _ => None,
                })
                .flatten()
                .collect::<HashMap<_, _>>();
            output
                .into_iter()
                .map(|number| lookup.get(&number))
                .flatten()
                .count()
        })
        .sum::<usize>();

    println!("1, 4, 7, and 8 appeared {} times", num_1478_appearances);
}
