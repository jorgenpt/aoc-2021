use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub enum State {
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

#[derive(Copy, Clone, Debug)]
pub struct Board {
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
#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut line_reader = input.lines();

    let called_numbers = line_reader.next().unwrap();
    let called_numbers = called_numbers
        .trim_end()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap());

    let board_reader = line_reader.chunks(6);
    let boards = board_reader
        .into_iter()
        .map(|board_lines| {
            let board_lines = board_lines.skip(1);
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
        .filter_map(|x| x);

    (called_numbers.collect(), boards.collect())
}

#[aoc(day4, part1)]
pub fn part1((called_numbers, boards): &(Vec<i32>, Vec<Board>)) -> Option<i32> {
    let mut boards = boards.clone();
    for called_number in called_numbers {
        let called_number = *called_number;
        for board_index in 0..boards.len() {
            let board = &mut boards[board_index];
            'row_loop: for row in 0..board.numbers.len() {
                for column in 0..board.numbers[row].len() {
                    match board.numbers[row][column] {
                        State::Uncalled(number) if number == called_number => {
                            board.numbers[row][column] = State::Called(number);
                            if board.has_won(row, column) {
                                return Some(board.get_score(row, column));
                            }
                            break 'row_loop;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    None
}

#[aoc(day4, part2)]
pub fn part2((called_numbers, boards): &(Vec<i32>, Vec<Board>)) -> Option<i32> {
    let mut boards = boards.clone();
    let mut last_win = None;
    for called_number in called_numbers {
        let called_number = *called_number;
        let mut finished_boards = Vec::new();
        let mut last_board_score = None;
        for board_index in 0..boards.len() {
            let board = &mut boards[board_index];
            'row_loop: for row in 0..board.numbers.len() {
                for column in 0..board.numbers[row].len() {
                    match board.numbers[row][column] {
                        State::Uncalled(number) if number == called_number => {
                            board.numbers[row][column] = State::Called(number);
                            if board.has_won(row, column) {
                                let score = board.get_score(row, column);
                                finished_boards.push(board_index);
                                last_board_score = Some(score);
                            }
                            break 'row_loop;
                        }
                        _ => {}
                    }
                }
            }
        }

        finished_boards.reverse();
        for board_index in finished_boards {
            boards.swap_remove(board_index);
        }

        if boards.len() == 0 {
            last_win = last_board_score;
            break;
        }
    }

    last_win
}
