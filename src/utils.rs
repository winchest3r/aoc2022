/// Split pattern for distinct data with empty space between.
pub const SPLIT_PATTERN: &'static str = cfg_select! {
    windows => "\n\n",
    _ => "\n\n",
};

/// Typical 2D point type with signed integer coordinates.
/// Expects that +Y is DOWN and +X is RIGHT.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn magnitude(self) -> f64 {
        (self.distance_squared(Point::ZERO) as f64).sqrt()
    }

    pub fn manhattan_distance(self, other: Self) -> u64 {
        (self.x - other.x).abs() as u64 + (self.y - other.y).abs() as u64
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

impl std::ops::Div<i64> for Point {
    type Output = Point;
    fn div(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
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

/// Same as Point but with Z dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    pub const UP: Point3 = Point3::new(0, 1, 0);
    pub const DOWN: Point3 = Point3::new(0, -1, 0);
    pub const RIGHT: Point3 = Point3::new(1, 0, 0);
    pub const LEFT: Point3 = Point3::new(-1, 0, 0);
    pub const FRONT: Point3 = Point3::new(0, 0, -1);
    pub const BACK: Point3 = Point3::new(0, 0, 1);
    /// Direction from unit vectors. Front is -Z.
    pub const DIRECTIONS: [Point3; 6] = [
        Point3::UP,
        Point3::DOWN,
        Point3::RIGHT,
        Point3::LEFT,
        Point3::FRONT,
        Point3::BACK,
    ];

    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub const fn negate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub const fn dot(self, rhs: Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Add for Point3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Greatest commod divider
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Least common multiplier
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
