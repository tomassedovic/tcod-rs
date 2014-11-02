extern crate tcod;

use tcod::Console;

fn main() {
    Console::init_root(80, 50, "Minimal libtcod loop", false);
    while !Console::window_closed() {
        Console::flush();
        let key = Console::wait_for_keypress(true);
        println!("Pressed key: {}", key);
    }
}
