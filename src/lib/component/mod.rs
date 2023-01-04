use super::{game::screen::{Screen, SCREEN_HEIGHT}, Point};

pub mod ball;
pub mod player;

pub trait Component {
    /// Takes in a given direction and moves the component one unit in said direction. The screen is also updated.
    fn action(&mut self, direction: Direction, screen: &mut Screen);

    /// Draws this component onto a screen.
    fn draw(&self, screen: &mut Screen) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                screen.write_pixel(Point::new(self.get_pos().x() + i, self.get_pos().y + j), 1);
            }
        }
    }

    /// Handles the position of the component when it collides with the edge of the screen.
    fn handle_collide(&mut self, edge: Edge, screen: &mut Screen);

    /// Checks if the component is colliding with the edge of the screen
    fn is_at_y_edge(&self) -> Option<Edge> {
        let center_y = self.center().y;

        // This is the magic sign. It's used to conditionally negate (SCREEN_HEIGHT / 2) in y_distance_from_edge on
        // line 90. For example if the y coordinate is negative then it will negate (SCREEN_HEIGHT / 2) such that
        // it corresponds with the *bottom* edge and not the top.
        let sign = match self.get_pos().y.is_positive() {
            true => 1,
            false => -1,
        };

        // Calculate the distance between the center of the box and the edges of the screen
        let y_distance_from_edge = sign * (SCREEN_HEIGHT / 2) - center_y;

        // Check if the distance between the center of the box and the top or bottom edge of the screen
        // is close and if so, return the appropriate edge
        if y_distance_from_edge.abs() <= (self.get_height() / 2) - sign + 1{
            if self.get_pos().y.is_positive() {
                return Some(Edge::Top);
            }
            return Some(Edge::Bottom);
        }
        None
    }

    /// Clears the ball off the screen
    fn clear(&mut self, screen: &mut Screen) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                screen.write_pixel(
                    Point::new(self.get_pos().x() + i, self.get_pos().y() + j),
                    0,
                );
            }
        }
    }

    /// Returns the coordinates of the center of the component
    fn center(&self) -> Point {
        Point::new(self.get_pos().x + self.get_width(), self.get_pos().y + self.get_height())
    }

    /// Returns the position of the component
    fn get_pos(&self) -> Point;

    /// Returns the width of the component
    fn get_width(&self) -> i32;

    /// Returns the height of the component
    fn get_height(&self) -> i32;
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// An enum representing the different directions
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    // Diagnol directions
    Upright,
    Downright,
    Upleft,
    Downleft,
}

pub enum Edge {
    Top,
    Bottom,
}
