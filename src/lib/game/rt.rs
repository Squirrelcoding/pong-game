use crate::lib::component::{ball::Ball, player::Player};

use super::screen::Screen;

pub struct Runtime {
    screen: Screen,
    ball: Ball,
    p1: Player,
    p2: Player,
}
