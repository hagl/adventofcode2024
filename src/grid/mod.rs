#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const VALUES: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    pub fn right(self: &Direction) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn left(self: &Direction) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

pub struct Grid {
    array: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn from_str(s: &str) -> Grid {
        let array: Vec<Vec<char>> = s
            .split("\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect())
            .collect();

        let width = array[0].len();
        let height = array.len();

        Grid {
            array,
            width,
            height,
        }
    }

    pub fn get(self: &Grid, (x, y): (usize, usize)) -> Option<char> {
        let line = self.array.get(usize::try_from(y).ok()?)?;
        line.get(usize::try_from(x).ok()?).copied()
    }

    pub fn pos_in_direction(
        self: &Grid,
        (x, y): (usize, usize),
        dir: &Direction,
    ) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x + 1 < self.width {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y + 1 < self.height {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        }
    }
}
