use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split(",").map(str::parse::<i32>).flatten().collect()
}

#[aoc(day7, part1)]
pub fn part1(horizontal_positions: &Vec<i32>) -> i32 {
    let horizontal_positions = {
        let mut positions = horizontal_positions.clone();
        positions.sort();
        positions
    };

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

    cost
}

#[aoc(day7, part2)]
pub fn part2(horizontal_positions: &Vec<i32>) -> Option<i32> {
    let horizontal_positions = {
        let mut positions = horizontal_positions.clone();
        positions.sort();
        positions
    };

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

    costs.min()
}
