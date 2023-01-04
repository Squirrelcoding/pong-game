pub mod component;
pub mod game;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn is_near(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 5 && (self.y - other.y).abs() < 5
    }
}
