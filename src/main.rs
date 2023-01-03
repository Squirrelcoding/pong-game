mod lib;

use std::{thread, time::Duration};

use lib::{
    component::{player::Player, Component, Direction},
    game::{screen::Screen, rt::Runtime},
    Point,
};

fn main() {
    let mut rt = Runtime::new();

    rt.init();
}
