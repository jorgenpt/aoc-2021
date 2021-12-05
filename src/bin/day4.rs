use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum State {
    Uncalled(i32),
    Called(i32),
}

struct Board {
    numbers: [[State; 5]; 5],
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut called_numbers = String::new();
    reader.read_line(&mut called_numbers).unwrap();
    let called_numbers = called_numbers.split(",").map(|n| n.parse::<i32>().unwrap());

    let line_reader = reader.lines();
    let board_reader = line_reader.chunks(6);

    let mut boards = board_reader
        .into_iter()
        .map(|board_lines| {
            let board_lines = board_lines.skip(1).map(Result::unwrap);
            let mut board_rows = board_lines.map(|line| {
                let mut numbers = line
                    .split(' ')
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .map(State::Uncalled);
                [
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                ]
            });

            Board {
                numbers: [
                    board_rows.next().unwrap(),
                    board_rows.next().unwrap(),
                    board_rows.next().unwrap(),
                    board_rows.next().unwrap(),
                    board_rows.next().unwrap(),
                ],
            }
        })
        .collect::<Vec<_>>();

    for called_number in called_numbers {
        for board in &mut boards {
            for row in &mut board.numbers {
                for number in row {
                    match number {
                        State::Uncalled(called_number) => {}
                        _ => {}
                    }
                }
            }
        }
    }
}
