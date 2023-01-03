use super::game::screen::Screen;

pub mod ball;
pub mod player;
pub mod shape;

pub trait Component {
    /// Takes in a given direction and moves the component one unit in said direction. The screen is also updated.
    fn action(&mut self, direction: Direction, screen: &mut Screen);

    /// Draws this component onto a screen.
    fn draw(&self, screen: &mut Screen);

    /// Handles the position of the component when it collides with the edge of the screen.
    fn handle_collide(&mut self, edge: Edge, screen: &mut Screen);

    /// Checks if the component is colliding with the edge of the screen
    fn is_at_edge(&self) -> Option<Edge>;

    /// Clears the ball off the screen
    fn clear(&mut self, screen: &mut Screen);
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
    Left,
    Right,
}
