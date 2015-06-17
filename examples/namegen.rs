 extern crate tcod;

use tcod::namegen::Namegen;
use tcod::random::Rng;

fn main() {
    Namegen::parse("examples/names.txt", Rng::get_instance());
    println!("{}", Namegen::generate("king").unwrap());
    Namegen::reset();
}
