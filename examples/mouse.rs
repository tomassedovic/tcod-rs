extern crate tcod;

use tcod::input as input;
use tcod::{Console, RootConsole, BackgroundFlag};

fn main() {
    let mut con = RootConsole::initializer()
        .size(80, 50)
        .title("Move the cursor inside the window")
        .init();

    let mut x = 40;
    let mut y = 25;

    while !con.window_closed() {

        loop {
            match input::check_for_event(input::KEY | input::MOUSE) {
                None => {
                    break;
                }
                Some((_, event)) => {
                    match event {
                        input::Event::Key(ref key_state) => {
                            println!("{:?}", key_state);
                        },
                        input::Event::Mouse(ref mouse_state) => {
                            x = mouse_state.cx as i32;
                            y = mouse_state.cy as i32;
                            println!("{:?}", mouse_state);
                        }
                    }
                }
            }
        }

        con.clear();
        con.put_char(x, y, '@', BackgroundFlag::Set);
        con.flush();
    }
}
