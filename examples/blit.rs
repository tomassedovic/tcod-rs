extern crate tcod;

use tcod::console;
use tcod::{Console, RootConsole, OffscreenConsole};
use tcod::colors;


fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 50)
        .title("Using blit with libtcod")
        .init();

    let mut direct: OffscreenConsole = OffscreenConsole::new(20, 20);
    let mut boxed_direct: Box<OffscreenConsole> = Box::new(OffscreenConsole::new(20, 20));
    let mut trait_object: &Console = &OffscreenConsole::new(20, 20);
    let mut boxed_trait: Box<Console> = Box::new(OffscreenConsole::new(20, 20));


    root.set_default_background(colors::DARKEST_GREEN);

    direct.set_default_background(colors::RED);
    boxed_direct.set_default_background(colors::WHITE);
    trait_object.set_default_background(colors::BLACK);
    boxed_trait.set_default_background(colors::BLUE);

    root.clear();

    direct.clear();
    boxed_direct.clear();
    trait_object.clear();
    boxed_trait.clear();


    console::blit(&direct, (0, 0), (20, 20), &mut root, (0, 0), 1.0, 1.0);
    console::blit(&boxed_direct, (0, 0), (20, 20), &mut root, (20, 0), 1.0, 1.0);
    console::blit(&trait_object, (0, 0), (20, 20), &mut root, (0, 20), 1.0, 1.0);
    console::blit(&boxed_trait, (0, 0), (20, 20), &mut root, (20, 20), 1.0, 1.0);

    root.flush();
    root.wait_for_keypress(true);
}
