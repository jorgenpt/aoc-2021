use std::{ffi::OsStr, fs::File, io::Read, path::Path};

fn main() {
    let initial_stages = {
        let file_name = Path::new(file!())
            .file_stem()
            .map(OsStr::to_str)
            .flatten()
            .unwrap();
        let mut file = File::open(format!("{}.txt", file_name)).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
            .split(",")
            .map(str::parse::<usize>)
            .flatten()
            .collect::<Vec<_>>()
    };

    // This is where newly hatched lanternfish live until they're on to the "normal" mature lifecycle
    let mut immature = [0u32; 2];
    // Circular buffer style counter of number of laternfish at each "age"
    let mut mature = [0u32; 7];

    // Count up the fish from the input data into their appropriate buckets
    initial_stages.into_iter().for_each(|stage| {
        mature[stage] += 1;
    });

    for current_day in 0..80usize {
        // Treat `immature` as a circular buffer for who is ready to graduate into the mature bucket
        let current_immature_bucket = current_day % 2;
        let graduating_immature = immature[current_immature_bucket];

        let current_stage = current_day % 7;
        // "Spawn" immature lanternfish from today's generation
        immature[current_immature_bucket] = mature[current_stage];
        // Graduate the previously counted immature into the mature array
        mature[current_stage] += graduating_immature;
    }

    println!(
        "Total alive: {}",
        mature.iter().sum::<u32>() + immature.iter().sum::<u32>()
    )
}
