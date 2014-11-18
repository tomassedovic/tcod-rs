extern crate tcod;

use tcod::{Console, BackgroundFlag, Key, KeyCode};

fn main() {
	let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
	let mut exit = false;
	while !(Console::window_closed() || exit) {
		con.clear();
		con.put_char(40, 25, '@', BackgroundFlag::Set);
		Console::flush();
		let keypress = Console::wait_for_keypress(true);
		match keypress.key {
			Key::Special(KeyCode::Escape) => exit = true,
			_ => {}
		}
	}
}
