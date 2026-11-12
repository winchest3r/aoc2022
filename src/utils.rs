/// Split pattern for distinct data with empty space between.
pub const SPLIT_PATTERN: &'static str = cfg_select! {
    windows => "\r\n\r\n",
    _ => "\n\n",
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
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
