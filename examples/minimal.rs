extern crate tcod;

use tcod::RootConsole;

fn main() {
    let mut root = RootConsole::initializer().size(80, 50).title("Minimal libtcod loop").init();

    while !root.window_closed() {
        root.flush();
        let key = root.wait_for_keypress(true);
        println!("Pressed key: {:?}", key);
    }
}
