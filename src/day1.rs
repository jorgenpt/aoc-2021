use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(depths: &[u32]) -> u32 {
    let count_increases = |(count, previous), value| {
        if value > previous {
            (count + 1, value)
        } else {
            (count, value)
        }
    };

    let initial_value = (0, &depths[0]);
    let (increases, _) = depths.iter().skip(1).fold(initial_value, count_increases);
    increases
}

#[aoc(day1, part2)]
pub fn part2(depths: &[u32]) -> u32 {
    let count_increases = |(count, previous), value| {
        if value > previous {
            (count + 1, value)
        } else {
            (count, value)
        }
    };

    let sliding_windows = (0..depths.len() - 2)
        .map(|index| depths[index] + depths[index + 1] + depths[index + 2])
        .collect::<Vec<_>>();

    let initial_value = (0, &sliding_windows[0]);
    let (increases, _) = sliding_windows
        .iter()
        .skip(1)
        .fold(initial_value, count_increases);
    increases
}
