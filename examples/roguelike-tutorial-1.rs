//! Port of the Complete Roguelike Tutorial for Python + libtcod to Rust
//!
//! This is the complete code of the part one of the tutorial: Graphics:
//! http://www.roguebasin.com/index.php?title=Complete_Roguelike_Tutorial,_using_python%2Blibtcod,_part_1
//!

extern crate tcod;

use tcod::console::{Root, Console, FontLayout, FontType, BackgroundFlag};
use tcod::colors;
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;  // 20 frames-per-second maximum

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    // if let Some(keypress) = root.check_for_keypress()  // real-time
    let keypress = root.wait_for_keypress(true);

    match keypress.key {
        // Alt+Enter: toggle fullscreen
        Special(Enter) if keypress.left_alt => {
            let fullscreen = !root.is_fullscreen();
            root.set_fullscreen(fullscreen);
        }
        Special(Escape) => {
            return true  // exit game
        }
        // movement keys
        Special(Up) => *player_y -= 1,
        Special(Down) => *player_y += 1,
        Special(Left) => *player_x -= 1,
        Special(Right) => *player_x += 1,
        _ => {}
    }
    return false;
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);

        root.flush();

        root.put_char(player_x, player_y, ' ', BackgroundFlag::None);

        // handle keys and exit game if needed
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break
        }
    }
}
