use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let lines = lines.map(|l| l.unwrap());
    let binary_numbers = lines.map(|l| {
        l.chars()
            .map(|c| match c {
                '0' => 0u32,
                '1' => 1u32,
                _ => panic!("Invalid integer {}", c),
            })
            .collect::<Vec<_>>()
    });

    let (columns, column_sums) =
        binary_numbers.fold((0, vec![0; 0]), |(count, mut accum), mut row| {
            let accum = if accum.len() < row.len() {
                let mut vec = Vec::new();
                vec.resize(row.len() - accum.len(), 0);
                vec.append(&mut accum);
                vec
            } else {
                accum
            };

            let row = if row.len() < accum.len() {
                let mut vec = Vec::new();
                vec.resize(accum.len() - row.len(), 0);
                vec.append(&mut row);
                vec
            } else {
                row
            };

            return (
                count + 1,
                accum
                    .iter()
                    .zip(row)
                    .map(|(a, r)| a + r)
                    .collect::<Vec<_>>(),
            );
        });

    println!("{:?}", column_sums);

    let bitlength = column_sums.len();
    let epsilon_rate = column_sums.iter().fold(0u32, |accum, count| {
        (accum << 1) | ((*count > columns / 2) as u32)
    });

    println!("Columns: {},  mask: {}", columns, (1 << bitlength) - 1);
    let gamma_rate = !epsilon_rate & ((1 << (bitlength)) - 1);
    println!("{}", epsilon_rate * gamma_rate);

    Ok(())
}
