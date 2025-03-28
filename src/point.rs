#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub struct Coordinate {
//     pub x: usize,
//     pub y: usize,
// }

// impl Coordinate {
//     pub fn offset_by(&self, dx: isize, dy: isize) -> Coordinate {
//         let x = (self.x as isize + dx) as usize;
//         let y = (self.y as isize + dy) as usize;

//         Coordinate { x, y }
//     }
// }

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum Direction {
//     North,
//     NorthEast,
//     East,
//     SouthEast,
//     South,
//     SouthWest,
//     West,
//     NorthWest,
// }

// impl Direction {
//     /// Returns the vector of the direction
//     /// The vector is a tuple of (dx, dy)
//     ///
//     /// - Negative dx moves west
//     /// - Negative dy moves north
//     /// - Positive dx moves east
//     /// - Positive dy moves south
//     pub fn delta(&self) -> (isize, isize) {
//         match self {
//             Direction::North => (0, -1),
//             Direction::NorthEast => (1, -1),
//             Direction::East => (1, 0),
//             Direction::SouthEast => (1, 1),
//             Direction::South => (0, 1),
//             Direction::SouthWest => (-1, 1),
//             Direction::West => (-1, 0),
//             Direction::NorthWest => (-1, -1),
//         }
//     }

//     pub fn out_of_bounds(&self, coor: &Coordinate, width: usize, height: usize) -> bool {
//         let (dx, dy) = self.delta();

//         if coor.x as isize + dx < 0 || coor.x as isize + dx >= width as isize {
//             return true;
//         }

//         if coor.y as isize + dy < 0 || coor.y as isize + dy > height as isize {
//             return true;
//         }

//         false
//     }

//     pub fn offset_by(&self, coor: &Coordinate) -> Coordinate {
//         let (dx, dy) = self.delta();
//         coor.offset_by(dx, dy)
//     }
// }
