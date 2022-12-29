use crate::lib::component::Component;

use super::{
    component::{ball::Ball, player::Player},
    shape::Shape,
    Point,
};

pub const SCREEN_WIDTH: i32 = 100;
pub const SCREEN_HEIGHT: i32 = 20;

pub struct Screen {
    p1: Player,
    p2: Player,
    ball: Ball,
    pub buf: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
}

impl Screen {
    pub fn new() -> Screen {
        let p1_pos = Point::new(-40, 0);
        let p2_pos = Point::new(SCREEN_WIDTH - 10, 0);
        let ball_pos = Point::new(0, 0);

        Screen {
            p1: Player::new(p1_pos),
            p2: Player::new(p2_pos),
            ball: Ball::new(ball_pos),
            buf: [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    pub fn write_pixel(&mut self, p: Point, val: u8) {
        // Map the 2D coordinate to an index in the buffer
        let i = ((SCREEN_WIDTH / 2) + p.x) + SCREEN_WIDTH * ((SCREEN_HEIGHT / 2) - p.y);

        self.buf[i as usize] = val;
    }

    /// Clears and updates the screen
    pub fn update_screen(&self) {
        // Clear the screen
        print!("{}[2J", 27 as char);

        println!("####################################################################################################");
        // Iterate over each line by splitting the buffer into groups of SCREEN_WIDTH bytes
        self.buf.chunks(SCREEN_WIDTH as usize).for_each(|chunk| {
            // Render each byte
            chunk.iter().for_each(|byte| match byte {
                1 => {
                    print!("#");
                }
                0 => {
                    print!(" ");
                }
                _ => panic!(),
            });
            println!();
        });
        println!("####################################################################################################");
    }
}
