use crate::lib::{
    game::screen::{Screen, SCREEN_HEIGHT},
    Point,
};

use super::{Component, Direction, Edge};

pub struct Player {
    pos: Point,
    width: i32,
    height: i32,
    score: u8,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            width: 2,
            height: 6,
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
            _ => panic!("Player can only move vertically!"),
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

        match edge {
            Edge::Top => self.pos.y -= 1,
            Edge::Bottom => self.pos.y += 1,
            _ => panic!("Player cannot be at edge of screen!"),
        }
    }

    // Code written by me and refactored by ChatGPT
    fn is_at_edge(&self) -> Option<Edge> {
        // Calculate the center y-coordinate of the player component
        let center_y = self.pos.y() + (self.height / 2);

        // This is the magic sign. It's used to conditionally negate (SCREEN_HEIGHT / 2) in y_distance_from_edge on
        // line 90. For example if the y coordinate is negative then it will negate (SCREEN_HEIGHT / 2) such that
        // it corresponds with the *bottom* edge and not the top.
        let sign = match self.pos.y.is_positive() {
            true => 1,
            false => -1,
        };

        // Calculate the distance between the center of the box and the edges of the screen
        let y_distance_from_edge = sign * (SCREEN_HEIGHT / 2) - center_y;

        // Check if the distance between the center of the box and the top or bottom edge of the screen
        // is close and if so, return the appropriate edge
        if y_distance_from_edge.abs() <= (self.height / 2) - sign {
            if self.pos.y.is_positive() {
                return Some(Edge::Top);
            } 
            return Some(Edge::Bottom);
        }
        None
    }

    fn clear(&mut self, screen: &mut Screen) {
        for i in 0..self.width {
            for j in 0..self.height {
                screen.write_pixel(Point::new(self.pos.x() + i, self.pos.y() + j), 0);
            }
        }
    }
}
