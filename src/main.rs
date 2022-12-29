mod lib;

use std::{thread, time::Duration};

use lib::{
    component::{
        ball::{Ball},
        Component, Direction,
    },
    screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
    Point,
};

fn main() {
    let mut screen = Screen::new();

    // screen.write_pixel(Point::new(-50, -9), 1);
    // screen.update_screen();

    let mut ball = Ball::new(Point::new(40, 0));

    loop {
        screen.update_screen();
        ball.move_self(&mut screen);
        thread::sleep(Duration::from_millis(20));
    }
}
