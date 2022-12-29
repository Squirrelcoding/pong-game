use crate::lib::{
    screen::{self, Screen, SCREEN_WIDTH},
    Point,
};

use super::{Component, Direction, Edge};

pub struct Ball {
    pub pos: Point,
    width: i32,
    height: i32,
    direction: Direction,
}

impl Ball {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            width: 3,
            height: 4,
            direction: Direction::Right,
        }
    }

    /// Clears the ball off the screen
    fn clear(&mut self, screen: &mut Screen) {
        for i in 0..self.width {
            for j in 0..self.height {
                screen.write_pixel(Point::new(self.pos.x() + i, self.pos.y + j), 0);
            }
        }
    }

    pub fn move_self(&mut self, screen: &mut Screen) {
        self.action(self.direction.clone(), screen);
    }
}

impl Component for Ball {
    fn action(&mut self, direction: Direction, screen: &mut Screen) {
        if let Some(edge) = self.is_at_edge() {
            self.handle_collide(edge, screen);
        }

        match direction {
            Direction::Up => {
                self.pos.y += 1;

                // Calculate the y coordinate of the old bottom row
                let old_bottom_row_y = self.pos.y - 1;

                // Clear all the old pixels in the old bottom row
                for i in 0..self.width {
                    screen.write_pixel(Point::new(self.pos.x() + i, old_bottom_row_y), 0);
                }
            }
            Direction::Down => {
                self.pos.y -= 1;

                // Calculate the y coordinate of the old top row
                let old_top_row_y = self.pos.y + self.height;

                // Clear all the old pixels in the old top row
                for i in 0..self.width {
                    screen.write_pixel(Point::new(self.pos.x() + i, old_top_row_y), 0);
                }
            }
            Direction::Left => {
                self.pos.x -= 1;

                // Calculate the x coordinate of the old last column
                let old_last_column_x = self.pos.x() + self.width;

                // Clear all the old pixels in the old last column
                for i in 0..self.height {
                    screen.write_pixel(Point::new(old_last_column_x, self.pos.y + i), 0);
                }
            }
            Direction::Right => {
                self.pos.x += 1;

                // Calculate the x coordinate of the old last column
                let old_first_last_column = self.pos.x() - 1;

                // Clear all the old pixels in the old last column
                for i in 0..self.height {
                    screen.write_pixel(Point::new(old_first_last_column, self.pos.y + i), 0);
                }
            }
            Direction::Upright => {
                self.action(Direction::Right, screen);
                self.action(Direction::Up, screen);
            }
            Direction::Downright => {
                self.action(Direction::Right, screen);
                self.action(Direction::Down, screen);
            }
            Direction::Upleft => {
                self.action(Direction::Left, screen);
                self.action(Direction::Up, screen);
            }
            Direction::Downleft => {
                self.action(Direction::Left, screen);
                self.action(Direction::Down, screen);
            }
        }

        self.draw(screen);
    }

    fn draw(&self, screen: &mut Screen) {
        for i in 0..self.width {
            for j in 0..self.height {
                screen.write_pixel(Point::new(self.pos.x() + i, self.pos.y + j), 1);
            }
        }
    }

    fn handle_collide(&mut self, edge: Edge, screen: &mut Screen) {
        self.clear(screen);

        // self.pos.y += match self.pos.y.is_positive() {
        // true => -self.height,
        // false => self.height,
        // };

        match edge {
            Edge::Top => self.pos.y -= self.height,
            Edge::Bottom => self.pos.y += self.height,
            Edge::Left => self.pos.x += self.width,
            Edge::Right => self.pos.x -= self.width + 1,
        }

        self.direction = match self.direction {
            Direction::Upright => Direction::Downright,
            Direction::Downright => Direction::Upright,
            Direction::Upleft => Direction::Downleft,
            Direction::Downleft => Direction::Upleft,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }

    fn is_at_edge(&self) -> Option<Edge> {
        let center = Point::new(
            self.pos.x() + (self.width / 2),
            self.pos.y() + (self.height / 2),
        );

        if ((SCREEN_WIDTH / 2) - center.x.abs()) <= (self.width / 2) + 1 {
            if self.pos.x.is_positive() {
                return Some(Edge::Right);
            } else {
                return Some(Edge::Left);
            }
        }

        None
    }
}
