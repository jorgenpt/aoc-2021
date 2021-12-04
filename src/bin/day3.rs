use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Grab an iterator over the lines
    let lines = reader.lines().map(|l| l.unwrap());
    // Convert each line to a vector, each digit of the number is one u8
    let binary_numbers = lines.map(|l| {
        l.chars()
            .map(|c| match c {
                '0' => 0u8,
                '1' => 1u8,
                _ => panic!("Invalid integer {}", c),
            })
            .collect::<Vec<_>>()
    });

    // Sum each line (and count the number of lines) into one big Vec<u32>
    let (num_rows, column_sums) =
        binary_numbers.fold((0u32, vec![0u32; 0]), |(count, accum), row| {
            // Length of the longest previous number in the sequence
            let accumulated_number_bitlength = accum.len();
            // Make sure that they are the same size -- left pad accum if it's not as long as this row
            let accum = std::iter::repeat(0)
                .take(row.len().saturating_sub(accumulated_number_bitlength))
                .chain(accum);

            // Make sure that they are the same size -- left pad row if it's not as long as the previous rows
            let row = std::iter::repeat(0)
                .take(accumulated_number_bitlength.saturating_sub(row.len()))
                .chain(row);

            // Join the two iterators into tuples ala ((it1[0], it2[0]), (it1[1], it2[1]), ...) and then add
            // them elementwise (it1[0] + it2[0], it1[1] + it2[1], ..).
            let accum = accum
                .zip(row)
                .map(|(a, r)| a + (r as u32))
                .collect::<Vec<_>>();

            return (count + 1, accum);
        });

    // This is a mask whose bits is 1 for the length of our binary numbers
    let number_mask = (1 << column_sums.len()) - 1;
    // Calculate epsilon by creating a number that is 1 for each column whose value is greater than half the number of rows
    let epsilon_rate = column_sums.iter().fold(0u32, |accum, count| {
        (accum << 1) | ((*count > num_rows / 2) as u32)
    });

    // Since gamma is the opposite of epsilon, just use bitwise NOT on the epsilon number (but mask it
    // so that we only "not" the bits in our input numbers)
    let gamma_rate = !epsilon_rate & number_mask;
    println!("{}", epsilon_rate * gamma_rate);
}
