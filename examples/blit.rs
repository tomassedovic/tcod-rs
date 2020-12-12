extern crate tcod;

use tcod::console;
use tcod::error::Result;
use tcod::{Color, Console, OffscreenConsole, RootConsole};

fn main() -> Result<()> {
    let mut root = RootConsole::initializer()
        .size(80, 50)
        .title("Using blit with libtcod")
        .init()?;

    let mut direct: OffscreenConsole = OffscreenConsole::new(20, 20);
    let mut boxed_direct: Box<OffscreenConsole> = Box::new(OffscreenConsole::new(20, 20));
    let mut trait_object: &dyn Console = &OffscreenConsole::new(20, 20);
    let mut boxed_trait: Box<dyn Console> = Box::new(OffscreenConsole::new(20, 20));

    root.set_default_background(Color::DARKEST_GREEN);

    direct.set_default_background(Color::RED);
    boxed_direct.set_default_background(Color::WHITE);
    trait_object.set_default_background(Color::BLACK);
    boxed_trait.set_default_background(Color::BLUE);

    root.clear();

    direct.clear();
    boxed_direct.clear();
    trait_object.clear();
    boxed_trait.clear();

    console::blit(&direct, (0, 0), (20, 20), &mut root, (0, 0), 1.0, 1.0);
    console::blit(
        &boxed_direct,
        (0, 0),
        (20, 20),
        &mut root,
        (20, 0),
        1.0,
        1.0,
    );
    console::blit(
        &trait_object,
        (0, 0),
        (20, 20),
        &mut root,
        (0, 20),
        1.0,
        1.0,
    );
    console::blit(
        &boxed_trait,
        (0, 0),
        (20, 20),
        &mut root,
        (20, 20),
        1.0,
        1.0,
    );

    root.flush()?;
    root.wait_for_keypress(true);

    Ok(())
}
