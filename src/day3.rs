use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    // Convert each line to a vector, each digit of the number is one u8
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => 0u8,
                    '1' => 1u8,
                    _ => panic!("Invalid integer {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(binary_numbers: &[Vec<u8>]) -> u32 {
    // Sum each line (and count the number of lines) into one big Vec<u32>
    let (num_rows, column_sums) =
        binary_numbers
            .iter()
            .fold((0u32, vec![0u32; 0]), |(count, accum), row| {
                // Length of the longest previous number in the sequence
                let accumulated_number_bitlength = accum.len();
                // Make sure that they are the same size -- left pad accum if it's not as long as this row
                let accum = std::iter::repeat(0)
                    .take(row.len().saturating_sub(accumulated_number_bitlength))
                    .chain(accum);

                // Make sure that they are the same size -- left pad row if it's not as long as the previous rows
                let row = std::iter::repeat(&0u8)
                    .take(accumulated_number_bitlength.saturating_sub(row.len()))
                    .chain(row);

                // Join the two iterators into tuples ala ((it1[0], it2[0]), (it1[1], it2[1]), ...) and then add
                // them elementwise (it1[0] + it2[0], it1[1] + it2[1], ..).
                let accum = accum
                    .zip(row)
                    .map(|(a, r)| a + (*r as u32))
                    .collect::<Vec<_>>();

                return (count + 1, accum);
            });

    let binary_number_length = column_sums.len();
    // This is a mask whose bits is 1 for the length of our binary numbers
    let number_mask = (1 << binary_number_length) - 1;
    // Calculate epsilon by creating a number that is 1 for each column whose value is greater than half the number of rows
    let epsilon_rate = column_sums.iter().fold(0u32, |accum, count| {
        (accum << 1) | ((*count > num_rows / 2) as u32)
    });

    // Since gamma is the opposite of epsilon, just use bitwise NOT on the epsilon number (but mask it
    // so that we only "not" the bits in our input numbers)
    let gamma_rate = !epsilon_rate & number_mask;
    return epsilon_rate * gamma_rate;
}

#[aoc(day3, part2)]
pub fn part2(binary_numbers: &[Vec<u8>]) -> u32 {
    // Sum each line (and count the number of lines) into one big Vec<u32>
    let (_, column_sums) =
        binary_numbers
            .iter()
            .fold((0u32, vec![0u32; 0]), |(count, accum), row| {
                // Length of the longest previous number in the sequence
                let accumulated_number_bitlength = accum.len();
                // Make sure that they are the same size -- left pad accum if it's not as long as this row
                let accum = std::iter::repeat(0)
                    .take(row.len().saturating_sub(accumulated_number_bitlength))
                    .chain(accum);

                // Make sure that they are the same size -- left pad row if it's not as long as the previous rows
                let row = std::iter::repeat(&0u8)
                    .take(accumulated_number_bitlength.saturating_sub(row.len()))
                    .chain(row);

                // Join the two iterators into tuples ala ((it1[0], it2[0]), (it1[1], it2[1]), ...) and then add
                // them elementwise (it1[0] + it2[0], it1[1] + it2[1], ..).
                let accum = accum
                    .zip(row)
                    .map(|(a, r)| a + (*r as u32))
                    .collect::<Vec<_>>();

                return (count + 1, accum);
            });
    let binary_number_length = column_sums.len();

    let numbers = binary_numbers
        .into_iter()
        .map(|n| {
            n.into_iter()
                .fold(0u32, |accum, bit| (accum << 1) | (*bit as u32))
        })
        .collect::<Vec<_>>();

    let mut oxygen_rating_candidates = numbers.clone();
    for bit_index in 1..=binary_number_length {
        let bit_num = binary_number_length - bit_index;
        let num_set_bits = oxygen_rating_candidates
            .iter()
            .map(|n| (n >> bit_num) & 1)
            .sum::<u32>();

        let retain_bit = if num_set_bits >= oxygen_rating_candidates.len() as u32 - num_set_bits {
            1u32
        } else {
            0u32
        };

        oxygen_rating_candidates.retain(|n| (n >> bit_num) & 1 == retain_bit);
        if oxygen_rating_candidates.len() == 1 {
            break;
        }
    }

    let mut co2_rating_candidates = numbers.clone();
    for bit_index in 1..=binary_number_length {
        let bit_num = binary_number_length - bit_index;
        let num_set_bits = co2_rating_candidates
            .iter()
            .map(|n| (n >> bit_num) & 1)
            .sum::<u32>();

        let retain_bit = if num_set_bits < co2_rating_candidates.len() as u32 - num_set_bits {
            1u32
        } else {
            0u32
        };

        co2_rating_candidates.retain(|n| (n >> bit_num) & 1 == retain_bit);
        if co2_rating_candidates.len() == 1 {
            break;
        }
    }

    // Life support rating
    return oxygen_rating_candidates[0] * co2_rating_candidates[0];
}
