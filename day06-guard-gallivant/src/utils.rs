#[derive(Hash, Eq, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    pub fn step(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
}
