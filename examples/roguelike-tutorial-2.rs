//! Port of the Complete Roguelike Tutorial for Python + libtcod to Rust
//!
//! This is the complete code of the part two of the tutorial: The object and the map.
//! http://www.roguebasin.com/index.php?title=Complete_Roguelike_Tutorial,_using_python%2Blibtcod,_part_2
//!

extern crate tcod;

use tcod::console::{Root, Offscreen, Console, FontLayout, FontType, BackgroundFlag};
use tcod::colors::{self, Color};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 45;

const LIMIT_FPS: i32 = 20;  // 20 frames-per-second maximum

const COLOR_DARK_WALL: Color = Color{r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color{r: 50, g: 50, b: 150};

type Map = Vec<Vec<Tile>>;

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    /// Move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

#[derive(Copy, Clone)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

fn make_map() -> Map {
    let mut map = vec![];

    // fill map with "unblocked" tiles
    for _ in 0..MAP_WIDTH {
        let column = vec![Tile{blocked: false, block_sight: false}; MAP_HEIGHT];
        map.push(column);
    }

    // place two pillars to test the map
    map[30][22].blocked = true;
    map[30][22].block_sight = true;
    map[50][22].blocked = true;
    map[50][22].block_sight = true;

    map
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    for object in objects {
        object.draw(con);
    }

    // go through all tiles, and set their background color
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let wall = map[x][y].block_sight;
            let (x, y) = (x as i32, y as i32);
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
    // blit the contents of "con" to the root console
    tcod::console::blit(con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
                        root, (0, 0),
                        1.0, 1.0);
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    // if let Some(keypress) = root.check_for_keypress()  // real-time
    let keypress = root.wait_for_keypress(true);  // turn-based
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
        Special(Up) => player.move_by(0, -1, map),
        Special(Down) => player.move_by(0, 1, map),
        Special(Left) => player.move_by(-1, 0, map),
        Special(Right) => player.move_by(1, 0, map),
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

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    let map = make_map();

    while !root.window_closed() {
        // render the screen
        render_all(&mut root, &mut con, &objects, &map);

        root.flush();

        // erase all objects at their old location, before they move
        for object in &objects {
            object.clear(&mut con);
        }

        // handle keys and exit game if needed
        let exit = handle_keys(&mut root, &mut objects[0], &map);
        if exit {
            break
        }
    }
}
