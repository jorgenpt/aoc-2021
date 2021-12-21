#[derive(Debug)]
pub struct AocMap<T> {
    pub width: u8,
    pub height: u8,
    pub values: Vec<T>,
}

impl<T> AocMap<T>
where
    T: Copy,
{
    pub const NEIGHBORS: [(i16, i16); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn get_value(&self, x: i16, y: i16) -> Option<T> {
        let (width, height) = (self.width as i16, self.height as i16);
        if x < 0 || x >= width {
            None
        } else if y < 0 || y >= height {
            None
        } else {
            Some(self.values[(x + y * width) as usize])
        }
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
