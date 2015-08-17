extern crate tcod;
use tcod::console::{Root, Console, BackgroundFlag, Offscreen, blit, Renderer};
use tcod::input::{Key, KeyCode, KEY_RELEASED};
use tcod::system::set_fps;
use tcod::colors;

fn main() {
    let renderer = Renderer::SDL;
    let mut root = Root::initializer()
        .size(80, 50)
        .title("libtcod Rust sample")
        .fullscreen(false)
        .renderer(renderer)
        .init();
    while !root.window_closed() {
        root.flush();
        root.check_for_keypress(KEY_RELEASED);
    }
}
