use aoc_runner_derive::{aoc, aoc_generator};

use itertools::{iproduct, Itertools};

#[derive(Debug)]
pub struct Map {
    width: u8,
    height: u8,
    heights: Vec<u8>,
}

impl Map {
    const NEIGHBORS: [(i16, i16); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn get_height(&self, x: i16, y: i16) -> Option<u8> {
        let (width, height) = (self.width as i16, self.height as i16);
        if x < 0 || x >= width {
            None
        } else if y < 0 || y >= height {
            None
        } else {
            Some(self.heights[(x + y * width) as usize])
        }
    }

    fn get_num_neighbors(&self, x: i16, y: i16) -> usize {
        let x_border = x == 0 || x == (self.width as i16) - 1;
        let y_border = y == 0 || y == (self.height as i16) - 1;
        Self::NEIGHBORS.len() - x_border as usize - y_border as usize
    }

    fn get_higher_neighbors(&self, x: i16, y: i16) -> Vec<(i16, i16)> {
        let point_height = self.get_height(x, y);
        if let Some(point_height) = point_height {
            Self::NEIGHBORS
                .into_iter()
                .filter_map(|neighbor| {
                    let (neighbor_x, neighbor_y) = (x + neighbor.0, y + neighbor.1);
                    if let Some(height) = self.get_height(neighbor_x, neighbor_y) {
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
pub fn generator(input: &str) -> Map {
    let mut width = None;
    let mut map = vec![];
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    c if c >= '0' && c <= '9' => Some(c as u8 - '0' as u8),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .for_each(|mut line| {
            if let Some(width) = width {
                assert_eq!(width, line.len());
            } else {
                width = Some(line.len());
            }

            map.append(&mut line);
        });

    let width = width.unwrap();
    Map {
        width: width as u8,
        height: (map.len() / width) as u8,
        heights: map,
    }
}

#[aoc(day9, part1)]
pub fn part1(map: &Map) -> usize {
    (0..map.height)
        .map(|y| {
            let y = y as i16;

            (0..map.width)
                .map(|x| {
                    let x = x as i16;

                    let point_height = map.get_height(x, y).unwrap();
                    let lower_neighbors = Map::NEIGHBORS.into_iter().any(|neighbor| {
                        if let Some(height) = map.get_height(x + neighbor.0, y + neighbor.1) {
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
pub fn part2(map: &Map) -> usize {
    let basins = iproduct!(0..map.height, 0..map.width)
        .map(|(y, x)| {
            let (x, y) = (x as i16, y as i16);
            let mut higher_neighbors = map.get_higher_neighbors(x, y);
            if higher_neighbors.len() != map.get_num_neighbors(x, y) {
                0
            } else {
                let mut basin = vec![(x, y)];
                while let Some(neighbor) = higher_neighbors.pop() {
                    if basin.contains(&neighbor) {
                        continue;
                    }

                    if map.get_height(neighbor.0, neighbor.1).unwrap() == 9 {
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
