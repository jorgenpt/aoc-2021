use aoc_2021::{AocMap, Point};
use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

trait HeightMap {
    fn get_num_neighbors(&self, p: Point) -> usize;
    fn get_higher_neighbors(&self, p: Point) -> Vec<Point>;
}

impl HeightMap for AocMap<u8> {
    fn get_num_neighbors(&self, p: Point) -> usize {
        let x_border = p.x == 0 || p.x == self.size.x - 1;
        let y_border = p.y == 0 || p.y == self.size.y - 1;
        Self::PLUS_NEIGHBORS.len() - x_border as usize - y_border as usize
    }

    fn get_higher_neighbors(&self, p: Point) -> Vec<Point> {
        let point_height = self.get(p);
        Self::PLUS_NEIGHBORS
            .into_iter()
            .filter_map(|neighbor_offset| {
                if let Some(neighbor) = self.get_relative(p, neighbor_offset) {
                    let height = self.get(neighbor);
                    if height > point_height {
                        Some(neighbor)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> AocMap<u8> {
    AocMap::<u8>::from_render(input, |c| match c {
        c if c >= '0' && c <= '9' => Some(c as u8 - '0' as u8),
        _ => None,
    })
}

#[aoc(day9, part1)]
pub fn part1(map: &AocMap<u8>) -> usize {
    map.coordinates()
        .map(|p| {
            let point_height = map.get(p);
            let lower_neighbors = AocMap::<u8>::PLUS_NEIGHBORS
                .into_iter()
                .any(|neighbor_offset| {
                    if let Some(neighbor) = map.get_relative(p, neighbor_offset) {
                        map.get(neighbor) <= point_height
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
}

#[aoc(day9, part2)]
pub fn part2(map: &AocMap<u8>) -> usize {
    let basins = map
        .coordinates()
        .map(|p| {
            let mut higher_neighbors = map.get_higher_neighbors(p);
            if higher_neighbors.len() != map.get_num_neighbors(p) {
                0
            } else {
                let mut basin = vec![p];
                while let Some(neighbor) = higher_neighbors.pop() {
                    if basin.contains(&neighbor) {
                        continue;
                    }

                    if map.get(neighbor) == 9 {
                        continue;
                    }

                    basin.push(neighbor);
                    let mut neighbors_higher_neighbors = map.get_higher_neighbors(neighbor);
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
