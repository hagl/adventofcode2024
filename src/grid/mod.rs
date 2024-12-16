#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const VALUES: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    pub fn from_char(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => todo!(),
        }
    }

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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Grid {
    array: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

struct GridIterator {
    width: usize,
    height: usize,
    current: Point,
}

impl Iterator for GridIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        let Point { x, y } = self.current;
        if x < self.width - 1 {
            self.current = Point { x: x + 1, y };
            Some(self.current)
        } else {
            if y < self.height - 1 {
                self.current = Point { x: 0, y: y + 1 };
                Some(self.current)
            } else {
                None
            }
        }
    }
}

impl Grid {
    pub fn iter(self: &Grid) -> impl Iterator<Item = Point> {
        GridIterator {
            width: self.width,
            height: self.height,
            current: Point { x: 0, y: 0 },
        }
    }

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
        self.get_point(Point { x, y })
    }

    pub fn get_point(self: &Grid, Point { x, y }: Point) -> Option<char> {
        let line = self.array.get(y)?;
        line.get(usize::try_from(x).ok()?).copied()
    }

    pub fn set_point(self: &mut Grid, Point { x, y }: Point, c: char) {
        let line = &mut self.array[y];
        line[x] = c;
    }

    pub fn point_in_direction(self: &Grid, Point { x, y }: Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::Up => {
                if y > 0 {
                    Some(Point { x, y: y - 1 })
                } else {
                    None
                }
            }
            Direction::Right => {
                if x + 1 < self.width {
                    Some(Point { x: x + 1, y })
                } else {
                    None
                }
            }
            Direction::Down => {
                if y + 1 < self.height {
                    Some(Point { x, y: y + 1 })
                } else {
                    None
                }
            }
            Direction::Left => {
                if x > 0 {
                    Some(Point { x: x - 1, y })
                } else {
                    None
                }
            }
        }
    }

    pub fn pos_in_direction(
        self: &Grid,
        (x, y): (usize, usize),
        dir: Direction,
    ) -> Option<(usize, usize)> {
        self.point_in_direction(Point { x, y }, dir)
            .map(|p| (p.x, p.y))
    }

    pub fn to_string(self: &Grid) -> String {
        let mut result = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(self.get((x, y)).unwrap());
            }
            result.push('\n');
        }
        result
    }

    pub fn to_string2(self: &Grid, Point { x: x0, y: y0 }: Point, c: char) -> String {
        let mut result = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                if x == x0 && y == y0 {
                    result.push(c);
                } else {
                    result.push(self.get((x, y)).unwrap());
                }
            }
            result.push('\n');
        }
        result
    }
}
