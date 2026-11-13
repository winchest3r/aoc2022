/// Split pattern for distinct data with empty space between.
pub const SPLIT_PATTERN: &'static str = cfg_select! {
    windows => "\r\n\r\n",
    _ => "\n\n",
};

/// Typical 2D point type with signed integer coordinates.
/// Expects that +Y is DOWN and +X is RIGHT.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const ZERO: Point = Point::new(0, 0);
    pub const UP: Point = Point::new(0, -1);
    pub const RIGHT: Point = Point::new(1, 0);
    pub const DOWN: Point = Point::new(0, 1);
    pub const LEFT: Point = Point::new(-1, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn distance_squared(self, other: Self) -> u64 {
        (self.x - other.x).pow(2) as u64 + (self.y - other.y).pow(2) as u64
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<u64> for Point {
    type Output = Point;
    fn mul(self, rhs: u64) -> Self::Output {
        Point {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl From<(u64, u64)> for Point {
    fn from(value: (u64, u64)) -> Self {
        Point::new(value.0 as i64, value.1 as i64)
    }
}
