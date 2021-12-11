use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(str::parse::<usize>)
        .flatten()
        .collect()
}

fn simulate_and_count(initial_stages: &[usize], num_days: usize) -> u64 {
    // This is where newly hatched lanternfish live until they're on to the "normal" mature lifecycle
    let mut immature = [0u64; 2];
    // Circular buffer style counter of number of laternfish at each "age"
    let mut mature = [0u64; 7];

    // Count up the fish from the input data into their appropriate buckets
    initial_stages.into_iter().for_each(|stage| {
        mature[*stage] += 1;
    });

    for current_day in 0..num_days {
        // Treat `immature` as a circular buffer for who is ready to graduate into the mature bucket
        let current_immature_bucket = current_day % 2;
        let graduating_immature = immature[current_immature_bucket];

        let current_stage = current_day % 7;
        // "Spawn" immature lanternfish from today's generation
        immature[current_immature_bucket] = mature[current_stage];
        // Graduate the previously counted immature into the mature array
        mature[current_stage] += graduating_immature;
    }

    mature.iter().sum::<u64>() + immature.iter().sum::<u64>()
}

#[aoc(day6, part1)]
pub fn part1(initial_stages: &[usize]) -> u64 {
    simulate_and_count(initial_stages, 80)
}

#[aoc(day6, part2)]
pub fn part2(initial_stages: &[usize]) -> u64 {
    simulate_and_count(initial_stages, 256)
}
