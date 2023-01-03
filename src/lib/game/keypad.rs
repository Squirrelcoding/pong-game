pub struct Controls(char, char);

impl Controls {
    pub fn new(up: char, down: char) -> Controls {
        Controls(up, down)
    }
}
