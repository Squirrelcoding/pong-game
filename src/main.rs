mod lib;

use std::{thread, time::Duration};

use lib::{
    component::{player::Player, Component, Direction},
    game::{screen::Screen},
    Point,
};

fn main() {
    let mut screen = Screen::new();

    // screen.write_pixel(Point::new(-50, -9), 1);
    // screen.update_screen();

    let mut p1 = Player::new(Point::new(-40, 5));
    let mut p2 = Player::new(Point::new(40, -9));

    loop {
        screen.update_screen();
        p1.action(Direction::Down, &mut screen);
        p2.action(Direction::Up, &mut screen);

        thread::sleep(Duration::from_millis(10));
    }
}
