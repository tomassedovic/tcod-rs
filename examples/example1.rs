extern crate tcod;

use tcod::{Console, background_flag, key_code, Special};
use tcod::colors;

fn main() {
	let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    con.set_default_foreground(colors::lighter_azure);
	let mut exit = false;
	while !(Console::window_closed() || exit) {
		con.clear();
		con.put_char(40, 25, '@', background_flag::Set);
		Console::flush();
		let keypress = Console::wait_for_keypress(true);
		match keypress.key {
			Special(key_code::Escape) => exit = true,
			_ => {}
		}
	}
}
