extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag, TextAlignment};

fn main() {
    let mut root = RootConsole::initializer().size(80, 50).title("Displaying text").init();

    root.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left,
                  b"Ascii text aligned to left.\xf8");
    root.print_ex(78, 1, BackgroundFlag::None, TextAlignment::Right,
                  "Unicode text aligned to right.\u{f8}");
    root.print_ex(40, 15, BackgroundFlag::None, TextAlignment::Center,
                  "And this bit of text is centered.");
    root.print_ex(40, 19, BackgroundFlag::None, TextAlignment::Center,
                  "Press any key to quit.");

    let wrapped_text = "This text is wrapped to form X lines: https://xkcd.com/688/";
    let lines = root.get_height_rect(10, 25, 22, 10, wrapped_text);
    root.print_rect(10, 25, 22, 10, wrapped_text.replace("X", &lines.to_string()));
    root.flush();
    root.wait_for_keypress(true);
}
