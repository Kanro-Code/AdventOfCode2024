#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn offset(&self, dx: isize, dy: isize) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn translate_offset(&self, other: Point) -> Point {
        let (dx, dy) = self.delta(other);
        self.offset(dx, dy)
    }

    pub fn delta(&self, other: Point) -> (isize, isize) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn translate_direction(&self, direction: Direction) -> Point {
        let (dx, dy) = direction.delta();
        self.offset(dx, dy)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset() {
        let point = Point { x: 1, y: 1 };
        assert_eq!(point.offset(1, 1), Point { x: 2, y: 2 });

        let point_a = Point { x: 3, y: 3 };
        let point_b = Point { x: 5, y: 5 };

        assert_eq!(point_a.translate_offset(point_b), Point { x: 1, y: 1 });
        assert_eq!(point_b.translate_offset(point_a), Point { x: 7, y: 7 });
    }
}
