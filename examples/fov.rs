extern crate rand;
extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag, Map};
use tcod::map::FovAlgorithm;

// We'll use a basic structure to define our tiles.
#[derive(Copy, Clone)]
pub struct Tile {
    ch: char,
    x: i32,
    y: i32,
}

fn main() {
    let mut root = RootConsole::initializer().size(40, 40) .title("FOV example").init();

    let mut map = Map::new(40,40);
    let mut tiles = Vec::new();

    root.clear();

    // Set the map.
    for x in 0..40 {
        for y in 0..40 {
            // Place some walls randomly.
            if rand::random() {
                tiles.push(Tile{x:x, y:y, ch: '#' });
                // Mark this place as non transparent, and non walkable.
                map.set(x,y,false,false);
            } else {
                tiles.push(Tile{x:x, y:y, ch: '.'});
                // Mark this place as transparent and walkable.
                map.set(x,y,true,true);
            }
        }
    }

    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.
    map.compute_fov(20,20, 10, true, FovAlgorithm::Basic);

    for tile in tiles.iter() {
        if map.is_in_fov(tile.x, tile.y) {
            root.put_char(tile.x,tile.y,tile.ch, BackgroundFlag::Set);
        }
    }

    root.put_char(20,20, '@', BackgroundFlag::Set);

    root.flush();
    //Press any key to exit.
    root.wait_for_keypress(true);
}
