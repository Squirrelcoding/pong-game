use std::{thread, time::Duration};

use crate::lib::{
    component::{ball::Ball, player::Player, Component, Direction},
    Point,
};

use super::screen::Screen;

pub struct Runtime {
    screen: Screen,
    ball: Ball,
    p1: Player,
    p2: Player,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            screen: Screen::new(),
            ball: Ball::new(Point::new(0, 0)),
            p1: Player::new(Point::new(-40, 0)),
            p2: Player::new(Point::new(40, 0)),
        }
    }

    /// Initiates the runtime
    pub fn init(&mut self) {
        loop {
            // Check if the ball is at the edge of the screen and if so then increment the score of the other player
            // and redraw the ball in the center of the screen.
            if self.ball.pos.x == 50 {
                self.p1.inc_score();
                self.ball.clear(&mut self.screen);
                self.ball.pos.reset();
            } else if self.ball.pos.x == -50 {
                self.p2.inc_score();
                self.ball.clear(&mut self.screen);
                self.ball.pos.reset();
            }

            // Check if the ball is near the players
            if self.ball.center().is_near(&self.p1.center())
                || self.ball.center().is_near(&self.p2.center())
            {
                self.ball.bounce_back();
            }

            self.screen.update_screen();
            println!("P1: {}\tP2: {}", self.p1.get_score(), self.p2.get_score());

            self.p1.action(Direction::Down, &mut self.screen);
            self.p2.action(Direction::Up, &mut self.screen);

            self.ball.move_self(&mut self.screen);

            thread::sleep(Duration::from_millis(10));
        }
    }
}
