use crate::lib::{
    screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
    Point,
};

use super::{Component, Direction, Edge};

pub struct Player {
    pos: Point,
    width: i32,
    height: i32,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            width: 2,
            height: 6,
        }
    }
}

impl Component for Player {
    fn action(&mut self, direction: super::Direction, screen: &mut Screen) {
        // if self.is_at_edge() {
        // self.handle_collide(screen);
        // }

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
            _ => panic!("Player cannot move left or right!"),
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
        todo!()
    }

    fn is_at_edge(&self) -> Option<Edge> {
        if i32::abs(SCREEN_WIDTH - self.pos.x()) < self.width {
            todo!()
        }

        if i32::abs(SCREEN_HEIGHT - self.pos.y) < self.height {
            todo!()
        }

        todo!()
    }
}
