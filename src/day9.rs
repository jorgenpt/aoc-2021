use aoc_2021::AocMap;
use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

trait HeightMap {
    fn get_num_neighbors(&self, x: i16, y: i16) -> usize;
    fn get_higher_neighbors(&self, x: i16, y: i16) -> Vec<(i16, i16)>;
}

impl HeightMap for AocMap<u8> {
    fn get_num_neighbors(&self, x: i16, y: i16) -> usize {
        let x_border = x == 0 || x == (self.width as i16) - 1;
        let y_border = y == 0 || y == (self.height as i16) - 1;
        Self::PLUS_NEIGHBORS.len() - x_border as usize - y_border as usize
    }

    fn get_higher_neighbors(&self, x: i16, y: i16) -> Vec<(i16, i16)> {
        let point_height = self.get_value(x, y);
        if let Some(point_height) = point_height {
            Self::PLUS_NEIGHBORS
                .into_iter()
                .filter_map(|neighbor| {
                    let (neighbor_x, neighbor_y) = (x + neighbor.0, y + neighbor.1);
                    if let Some(height) = self.get_value(neighbor_x, neighbor_y) {
                        if height > point_height {
                            Some((neighbor_x, neighbor_y))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> AocMap<u8> {
    AocMap::<u8>::generator(input, |c| match c {
        c if c >= '0' && c <= '9' => Some(c as u8 - '0' as u8),
        _ => None,
    })
}

#[aoc(day9, part1)]
pub fn part1(map: &AocMap<u8>) -> usize {
    (0..map.height)
        .map(|y| {
            let y = y as i16;

            (0..map.width)
                .map(|x| {
                    let x = x as i16;

                    let point_height = map.get_value(x, y).unwrap();
                    let lower_neighbors =
                        AocMap::<u8>::PLUS_NEIGHBORS.into_iter().any(|neighbor| {
                            if let Some(height) = map.get_value(x + neighbor.0, y + neighbor.1) {
                                height <= point_height
                            } else {
                                false
                            }
                        });
                    if lower_neighbors {
                        0
                    } else {
                        1 + point_height as usize
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(map: &AocMap<u8>) -> usize {
    let basins = map
        .coordinates()
        .map(|(y, x)| {
            let mut higher_neighbors = map.get_higher_neighbors(x, y);
            if higher_neighbors.len() != map.get_num_neighbors(x, y) {
                0
            } else {
                let mut basin = vec![(x, y)];
                while let Some(neighbor) = higher_neighbors.pop() {
                    if basin.contains(&neighbor) {
                        continue;
                    }

                    if map.get_value(neighbor.0, neighbor.1).unwrap() == 9 {
                        continue;
                    }

                    basin.push(neighbor);
                    let mut neighbors_higher_neighbors =
                        map.get_higher_neighbors(neighbor.0, neighbor.1);
                    higher_neighbors.append(&mut neighbors_higher_neighbors);
                }

                basin.len()
            }
        })
        .sorted_unstable()
        .rev();
    let basins = basins.collect_vec();
    let basins = basins.into_iter();

    let top_three_basins = basins.take(3);

    top_three_basins
        .reduce(|product, elem| product * elem)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210\n\
                         3987894921\n\
                         9856789892\n\
                         8767896789\n\
                         9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(INPUT)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(INPUT)), 1134);
    }
}
