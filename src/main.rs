mod lib;


use lib::{
    game::{rt::Runtime},
};

fn main() {
    let mut rt = Runtime::new();

    rt.init();
}
