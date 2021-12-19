use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Map {
    width: u8,
    height: u8,
    heights: Vec<u8>,
}

impl Map {
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
                    let lower_neighbors =
                        [(1, 0), (-1, 0), (0, 1), (0, -1)]
                            .into_iter()
                            .any(|neighbor| {
                                if let Some(height) = map.get_height(x + neighbor.0, y + neighbor.1)
                                {
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
}
