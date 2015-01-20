extern crate tcod;

use std::rand;

use tcod::{Console, BackgroundFlag, FovAlgorithm, Map};

// We'll use a basic structure to define our tiles.
pub struct Tile {
    ch: char,
    x: i32,
    y: i32,
}

fn main() {
    let mut root = Console::init_root(40,40, "FOV example", false);
    let mut map = Map::new(40,40);

    let mut tiles = Vec::new();

    root.clear();

    // Set the map.
    for x in range(0,40) {
        for y in range(0,40) {
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

    Console::flush();
    //Press any key to exit.
    Console::wait_for_keypress(true);
}
