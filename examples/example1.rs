extern crate tcod;

use tcod::{Console, BackgroundFlag};
use tcod::Key::Special;
use tcod::KeyCode::{Up, Down, Left, Right, Escape};

fn main() {
    let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    let mut x = 40;
    let mut y = 25;
    while !Console::window_closed() {
        con.clear();
        con.put_char(x, y, '@', BackgroundFlag::Set);
        Console::flush();
        let keypress = Console::wait_for_keypress(true);
        // libtcod 1.5.1 has a bug where `wait_for_keypress` emits two events:
        // one for key down and one for key up. So we ignore the "key up" ones.
        if keypress.pressed {
            match keypress.key {
                Special(Escape) => break,
                Special(Up) => y -= 1,
                Special(Down) => y += 1,
                Special(Left) => x -= 1,
                Special(Right) => x += 1,
                _ => {}
            }
        }
    }
}
