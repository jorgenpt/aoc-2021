use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(PartialEq)]
pub enum ChunkAction {
    Open,
    Close,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChunkType {
    Soff,
    Square,
    Squiggly,
    Angle,
}

impl ChunkType {
    fn get_score(&self) -> usize {
        match self {
            ChunkType::Soff => 3,
            ChunkType::Square => 57,
            ChunkType::Squiggly => 1197,
            ChunkType::Angle => 25137,
        }
    }
    fn get_remainder_score(&self) -> usize {
        match self {
            ChunkType::Soff => 1,
            ChunkType::Square => 2,
            ChunkType::Squiggly => 3,
            ChunkType::Angle => 4,
        }
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<(ChunkAction, ChunkType)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    '(' => Some((ChunkAction::Open, ChunkType::Soff)),
                    ')' => Some((ChunkAction::Close, ChunkType::Soff)),
                    '[' => Some((ChunkAction::Open, ChunkType::Square)),
                    ']' => Some((ChunkAction::Close, ChunkType::Square)),
                    '{' => Some((ChunkAction::Open, ChunkType::Squiggly)),
                    '}' => Some((ChunkAction::Close, ChunkType::Squiggly)),
                    '<' => Some((ChunkAction::Open, ChunkType::Angle)),
                    '>' => Some((ChunkAction::Close, ChunkType::Angle)),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(lines: &Vec<Vec<(ChunkAction, ChunkType)>>) -> usize {
    lines
        .into_iter()
        .map(|line| {
            let mut chunk_stack = Vec::<ChunkType>::new();
            for (chunk_action, chunk_type) in line {
                if *chunk_action == ChunkAction::Open {
                    chunk_stack.push(*chunk_type);
                } else {
                    if let Some(last_chunk) = chunk_stack.last() {
                        if last_chunk != chunk_type {
                            return chunk_type.get_score();
                        } else {
                            chunk_stack.pop();
                        }
                    } else {
                        return chunk_type.get_score();
                    }
                }
            }

            0
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(lines: &Vec<Vec<(ChunkAction, ChunkType)>>) -> usize {
    let scores = lines
        .into_iter()
        .filter_map(|line| {
            let mut chunk_stack = Vec::<ChunkType>::new();
            for (chunk_action, chunk_type) in line {
                if *chunk_action == ChunkAction::Open {
                    chunk_stack.push(*chunk_type);
                } else {
                    if let Some(last_chunk) = chunk_stack.last() {
                        if last_chunk != chunk_type {
                            return None;
                        } else {
                            chunk_stack.pop();
                        }
                    } else {
                        return None;
                    }
                }
            }

            Some(chunk_stack)
        })
        .map(|remaining_stack| {
            remaining_stack
                .into_iter()
                .rev()
                .map(|t| t.get_remainder_score())
                .fold(0, |accum, score| 5 * accum + score)
        })
        .sorted()
        .collect_vec();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>\n\
                         [(()[<>])]({[<{<<[]>>(\n\
                         {([(<{}[<>[]}>{[]{[(<()>\n\
                         (((({<>}<{<{<>}{[]{[]{}\n\
                         [[<[([]))<([[{}[[()]]]\n\
                         [{[{({}]{}}([{[{{{}}([]\n\
                         {<[[]]>}<{[{[{[]{()[[[]\n\
                         [<(<(<(<{}))><([]([]()\n\
                         <{([([[(<>()){}]>(<<{{\n\
                         <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(INPUT)), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(INPUT)), 288957);
    }
}
