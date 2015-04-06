extern crate tcod;

use tcod::RootConsole;

fn main() {
    let mut root = RootConsole::init(80, 50, "Minimal libtcod loop", false);
    while !root.window_closed() {
        root.flush();
        let key = root.wait_for_keypress(true);
        println!("Pressed key: {:?}", key);
    }
}
