extern crate tcod;

use tcod::RootConsole;

fn main() {
    RootConsole::init(80, 50, "Minimal libtcod loop", false);
    while !RootConsole::window_closed() {
        RootConsole::flush();
        let key = RootConsole::wait_for_keypress(true);
        println!("Pressed key: {:?}", key);
    }
}
