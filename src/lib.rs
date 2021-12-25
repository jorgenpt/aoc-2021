use std::fmt::Debug;

use itertools::iproduct;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
}

impl Offset {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct AocMap<T> {
    pub size: Point,
    pub values: Vec<T>,
}

impl<T> Debug for AocMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.values.chunks(self.size.x) {
            for elem in row {
                write!(f, "{:?}", elem)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl<T> AocMap<T>
where
    T: Copy,
{
    // Neighbor offsets in a + shape around the coordinate
    pub const PLUS_NEIGHBORS: [Offset; 4] = [
        Offset::new(0, -1),
        Offset::new(-1, 0),
        Offset::new(1, 0),
        Offset::new(0, 1),
    ];
    // Neighbor offsets including diagonal
    pub const ALL_NEIGHBORS: [Offset; 8] = [
        Offset::new(-1, -1),
        Offset::new(0, -1),
        Offset::new(1, -1),
        Offset::new(1, 0),
        Offset::new(-1, 0),
        Offset::new(-1, 1),
        Offset::new(0, 1),
        Offset::new(1, 1),
    ];

    pub fn set(&mut self, p: Point, v: T) {
        self.values[p.x + p.y * self.size.x] = v;
    }

    pub fn get(&self, p: Point) -> T {
        self.values[p.x + p.y * self.size.x]
    }

    // Add offset to usize, returning None if this would put the results outside of [0, exclusive_upper_bound)
    pub fn bounded_add(base: usize, offset: isize, exclusive_upper_bound: usize) -> Option<usize> {
        if offset < 0 {
            let negative_offset = -offset as usize;
            if base < negative_offset {
                None
            } else {
                Some(base - negative_offset)
            }
        } else {
            let offset = offset as usize;
            if base + offset >= exclusive_upper_bound {
                None
            } else {
                Some(base + offset)
            }
        }
    }

    pub fn get_relative(&self, point: Point, offset: Offset) -> Option<Point> {
        if let (Some(modified_0), Some(modified_1)) = (
            Self::bounded_add(point.x, offset.x, self.size.x),
            Self::bounded_add(point.y, offset.y, self.size.y),
        ) {
            Some(Point::new(modified_0, modified_1))
        } else {
            None
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Point> {
        iproduct!(0..self.size.y, 0..self.size.x).map(|(y, x)| Point::new(x, y))
    }

    pub fn from_render<F>(input: &str, f: F) -> Self
    where
        F: FnMut(char) -> Option<T> + Copy,
    {
        let mut width = None;
        let mut map = vec![];
        input
            .lines()
            .map(|line| line.chars().filter_map(f).collect::<Vec<_>>())
            .for_each(|mut line| {
                if let Some(width) = width {
                    assert_eq!(width, line.len());
                } else {
                    width = Some(line.len());
                }

                map.append(&mut line);
            });

        let width = width.unwrap();
        Self {
            size: Point::new(width, map.len() / width),
            values: map,
        }
    }
}
