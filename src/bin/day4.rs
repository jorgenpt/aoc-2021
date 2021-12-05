use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum State {
    Uncalled(i32),
    Called(i32),
}

impl State {
    fn is_called(&self) -> bool {
        match self {
            State::Called(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Board {
    numbers: [[State; 5]; 5],
}

impl Board {
    fn has_won(&self, called_row: usize, called_column: usize) -> bool {
        let row_complete = self.numbers[called_row].iter().all(State::is_called);
        if row_complete {
            return true;
        }

        let column_complete = self
            .numbers
            .iter()
            .map(|row| row.get(called_column).unwrap())
            .all(State::is_called);
        return column_complete;
    }

    fn get_score(&self, winning_row: usize, winning_column: usize) -> i32 {
        let uncalled_sum = self
            .numbers
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|v| match v {
                        State::Uncalled(n) => Some(n),
                        _ => None,
                    })
                    .sum::<i32>()
            })
            .sum::<i32>();
        let winning_cell = &self.numbers[winning_row][winning_column];
        match winning_cell {
            State::Called(number) => uncalled_sum * number,
            _ => panic!("What"),
        }
    }
}

fn main() {
    let file = File::open("day4.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut called_numbers = String::new();
    reader.read_line(&mut called_numbers).unwrap();
    let called_numbers = called_numbers
        .trim_end()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap());

    let line_reader = reader.lines();
    let board_reader = line_reader.chunks(6);

    let mut boards = board_reader
        .into_iter()
        .map(|board_lines| {
            let board_lines = board_lines.skip(1).map(Result::unwrap);
            let mut board_rows = board_lines.map(|line| {
                let mut numbers = line
                    .split_whitespace()
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

            let numbers = [
                board_rows.next(),
                board_rows.next(),
                board_rows.next(),
                board_rows.next(),
                board_rows.next(),
            ];
            if numbers.last().is_none() {
                None
            } else {
                Some(Board {
                    numbers: numbers.map(Option::unwrap),
                })
            }
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    'calls: for called_number in called_numbers {
        for board in &mut boards {
            'row_loop: for row in 0..board.numbers.len() {
                for column in 0..board.numbers[row].len() {
                    match board.numbers[row][column] {
                        State::Uncalled(number) if number == called_number => {
                            board.numbers[row][column] = State::Called(number);
                            if board.has_won(row, column) {
                                println!("winning score: {}", board.get_score(row, column));
                                break 'calls;
                            }
                            break 'row_loop;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
