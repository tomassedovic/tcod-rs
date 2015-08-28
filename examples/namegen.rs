 extern crate tcod;

use tcod::namegen::Namegen;
use tcod::random::{Rng, Algo};

fn setup_namegen() -> Namegen {
    let rng = Rng::new(Algo::MT);
    let mut namegen = Namegen::new().unwrap();
    namegen.parse_with_rng("data/namegen/mingos_standard.cfg", &rng);
    namegen
}

fn main() {
    let namegen = setup_namegen();
    println!("{}", namegen.generate("male").unwrap());
}
