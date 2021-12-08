use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn parse_wire(c: char) -> Option<u8> {
    match c {
        'a' => Some(0b0000001),
        'b' => Some(0b0000010),
        'c' => Some(0b0000100),
        'd' => Some(0b0001000),
        'e' => Some(0b0010000),
        'f' => Some(0b0100000),
        'g' => Some(0b1000000),
        _ => None,
    }
}

trait SevenSegment {
    fn find_and_remove_digit<T>(&mut self, predicate: T) -> u8
    where
        T: FnMut(u8) -> bool;
}

impl SevenSegment for Vec<u8> {
    fn find_and_remove_digit<T>(&mut self, mut predicate: T) -> u8
    where
        T: FnMut(u8) -> bool,
    {
        let position = self.iter().position(|d| predicate(*d)).unwrap();
        let value = self[position];
        self.remove(position);
        value
    }
}

fn main() {
    let notes = {
        let file_name = Path::new(file!())
            .file_stem()
            .map(OsStr::to_str)
            .flatten()
            .unwrap();
        let reader = BufReader::new(File::open(format!("{}.txt", file_name)).unwrap());

        reader.lines().flatten().map(|line| {
            let make_wires = |s: &str| {
                s.chars()
                    .map(parse_wire)
                    .flatten()
                    .reduce(|mask, value| mask | value)
                    .unwrap()
            };

            let (digits, output) = line.split(" | ").collect_tuple().unwrap();
            (
                digits
                    .split_whitespace()
                    .map(&make_wires)
                    .collect::<Vec<_>>(),
                output
                    .split_whitespace()
                    .map(&make_wires)
                    .collect::<Vec<_>>(),
            )
        })
    };

    let sum_of_all_numbers = notes
        .map(|(mut digits, output)| {
            let one = digits.find_and_remove_digit(|d| d.count_ones() == 2);
            let seven = digits.find_and_remove_digit(|d| d.count_ones() == 3);
            let four = digits.find_and_remove_digit(|d| d.count_ones() == 4);
            let eight = digits.find_and_remove_digit(|d| d.count_ones() == 7);

            let top_right_and_bottom_right = one;
            let top_wire = seven & !top_right_and_bottom_right;

            let bottom_left_and_bottom = !(four | top_wire) & 0b1111111;

            let nine = digits.find_and_remove_digit(|d| {
                d.count_ones() == 6 && (d & bottom_left_and_bottom).count_ones() == 1
            });
            let bottom = nine & bottom_left_and_bottom;
            let bottom_left = !bottom & bottom_left_and_bottom;

            let six = digits.find_and_remove_digit(|d| {
                d.count_ones() == 6 && (d & top_right_and_bottom_right).count_ones() == 1
            });
            let bottom_right = six & top_right_and_bottom_right;
            let top_right = !bottom_right & top_right_and_bottom_right;

            let zero = digits.find_and_remove_digit(|d| d.count_ones() == 6);

            let two = digits
                .find_and_remove_digit(|d| d.count_ones() == 5 && (d & bottom_left) == bottom_left);
            let three = digits
                .find_and_remove_digit(|d| d.count_ones() == 5 && (d & top_right) == top_right);
            let five = digits.find_and_remove_digit(|d| d.count_ones() == 5);

            let values = [zero, one, two, three, four, five, six, seven, eight, nine];

            output
                .into_iter()
                .map(|number| values.iter().position(|d| *d == number).unwrap())
                .reduce(|number, digit| number * 10 + digit)
                .unwrap()
        })
        .sum::<usize>();

    println!("Sum of all numbers: {}", sum_of_all_numbers);
}
