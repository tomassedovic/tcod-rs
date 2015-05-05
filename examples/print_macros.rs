#[macro_use]
extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag, TextAlignment};

fn main() {
    let mut root = RootConsole::initializer().size(80, 50).title("Displaying text").init();

    // No optional parameters
    tcod_print!(root, At(1, 1), "Any text params may be {}", "formatted with println! formatting");

    // One optional parameter
    tcod_print!(root, At(1, 3), Wrap(6, 2), "Simple wrap");
    tcod_print!(root, At(1, 6), Bg(BackgroundFlag::None), "No background flag");
    tcod_print!(root, At(75, 8), Align(TextAlignment::Right), "Right align");


    // Two optional parameters. The optional parameters may be given in any order
    tcod_print!(root, At(1, 10), Wrap(6, 2), Bg(BackgroundFlag::None), "Bg and wrap");
    tcod_print!(root, At(70, 12), Align(TextAlignment::Right), Wrap(6, 3), "Align and wrap");

    // Three optional parameters
    tcod_print!(root, At(40, 25), Wrap(10, 10), Bg(BackgroundFlag::None), Align(TextAlignment::Center),
                "This text is printed with every optional parameter, format: {} {}", "string", 1);

    root.flush();
    root.wait_for_keypress(true);
}
