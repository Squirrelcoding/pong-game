use rand::{thread_rng, Rng};

use crate::lib::{
    game::screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
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
            direction: Direction::Upright,
        }
    }

    pub fn move_self(&mut self, screen: &mut Screen) {
        self.action(self.direction.clone(), screen);
    }

    /// Returns a random direction for the ball to spin
    // Writen by ChatGPT.
    fn bounce_back(&mut self) {
        let mut rng = thread_rng();
        let choice = rng.gen_range(0..2);

        let new_direction = match self.direction {
            Direction::Up => {
                if choice == 0 {
                    Direction::Downleft
                } else {
                    Direction::Downright
                }
            }
            Direction::Down => {
                if choice == 0 {
                    Direction::Upleft
                } else {
                    Direction::Upright
                }
            }
            Direction::Upright => {
                if choice == 0 {
                    Direction::Downright
                } else {
                    Direction::Downleft
                }
            }
            Direction::Upleft => {
                if choice == 0 {
                    Direction::Upright
                } else {
                    Direction::Downright
                }
            }
            Direction::Downright => {
                if choice == 0 {
                    Direction::Upright
                } else {
                    Direction::Upleft
                }
            }
            Direction::Downleft => {
                if choice == 0 {
                    Direction::Upright
                } else {
                    Direction::Upleft
                }
            }
            _ => unreachable!(),
        };
        self.direction = new_direction;
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
        }

        self.draw(screen);
    }

    /// Clears the ball off the screen
    fn clear(&mut self, screen: &mut Screen) {
        for i in 0..self.width {
            for j in 0..self.height {
                screen.write_pixel(Point::new(self.pos.x() + i, self.pos.y + j), 0);
            }
        }
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

        match edge {
            Edge::Top => self.pos.y -= self.height + 1,
            Edge::Bottom => self.pos.y += self.height + 1,
            Edge::Left => todo!("aaa"),
            Edge::Right => todo!("aaa"),
        }

        self.bounce_back();
    }

    // Code written by me and refactored by ChatGPT
    fn is_at_edge(&self) -> Option<Edge> {
        let center = Point::new(
            self.pos.x() + (self.width / 2),
            self.pos.y() + (self.height / 2),
        );

        // Calculate the distance between the center of the box and the edges of the screen
        let x_distance_from_edge = (SCREEN_WIDTH / 2) - center.x.abs();
        let y_distance_from_edge = (SCREEN_HEIGHT / 2) - center.y.abs();

        // Check if the distance between the center of the box and the edge of the screen is close
        // and if so, check the sign of x and return the appropriate edge
        if x_distance_from_edge <= (self.width / 2) + 1 {
            if self.pos.x.is_positive() {
                return Some(Edge::Right);
            } else {
                return Some(Edge::Left);
            }
        }

        // Check if the distance between the center of the box and the top or bottom edge of the screen
        // is close and if so, return the appropriate edge
        if y_distance_from_edge <= (self.height / 2) - 1 && self.pos.y.is_positive() {
            return Some(Edge::Top);
        } else if y_distance_from_edge < self.height {
            return Some(Edge::Bottom);
        }

        None
    }
}
