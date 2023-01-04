use crate::lib::{
    game::screen::{Screen},
    Point,
};

use super::{Component, Direction, Edge};

pub struct Player {
    pub pos: Point,
    width: i32,
    height: i32,
    score: u8,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            width: 2,
            height: 10,
            score: 0
        }
    }

    pub fn inc_score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> u8 {
        self.score
    }
}

impl Component for Player {
    fn action(&mut self, direction: Direction, screen: &mut Screen) {
        if let Some(edge) = self.is_at_y_edge() {
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
            _ => panic!("Player can only move vertically!"),
        }

        self.draw(screen);
    }

    fn handle_collide(&mut self, edge: Edge, screen: &mut Screen) {
        self.clear(screen);

        match edge {
            Edge::Top => self.pos.y -= 1,
            Edge::Bottom => self.pos.y += 1,
        }
    }

    fn get_pos(&self) -> Point {
       self.pos.clone()
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

}
