use std::{fmt::Debug, ops::Range};

use itertools::{iproduct, Product};

#[derive(Clone)]
pub struct AocMap<T> {
    pub width: u8,
    pub height: u8,
    pub values: Vec<T>,
}

impl<T> Debug for AocMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.values.chunks(self.width as usize) {
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
    pub const PLUS_NEIGHBORS: [(i16, i16); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    // Neighbor offsets including diagonal
    pub const ALL_NEIGHBORS: [(i16, i16); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn is_valid(&self, x: i16, y: i16) -> bool {
        let (width, height) = (self.width as i16, self.height as i16);
        if x < 0 || x >= width {
            false
        } else if y < 0 || y >= height {
            false
        } else {
            true
        }
    }

    pub fn set_value(&mut self, x: i16, y: i16, v: T) {
        let width = self.width as i16;
        self.values[(x + y * width) as usize] = v;
    }

    pub fn get_value(&self, x: i16, y: i16) -> Option<T> {
        let width = self.width as i16;
        if self.is_valid(x, y) {
            Some(self.values[(x + y * width) as usize])
        } else {
            None
        }
    }

    pub fn coordinates(&self) -> Product<Range<i16>, Range<i16>> {
        iproduct!(0..(self.height as i16), 0..(self.width as i16))
    }

    pub fn generator<F>(input: &str, f: F) -> Self
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
            width: width as u8,
            height: (map.len() / width) as u8,
            values: map,
        }
    }
}
