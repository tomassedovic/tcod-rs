extern crate tcod;

use tcod::{Console, FontLayout, FontType, RootConsole};

const FLOOR_TILE: char = 1 as char;
const WALL_TILE: char = 2 as char;
const HERO_TILE: char = 3 as char;
const CHEST_TILE: char = 4 as char;
const ENEMY_1_TILE: char = 5 as char;
const ENEMY_2_TILE: char = 6 as char;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

const ROOM_WIDTH: i32 = 20;
const ROOM_HEIGHT: i32 = 15;

fn main() {
    let mut root = RootConsole::initializer()
        .font("angband16x16.bmp", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .font_dimensions(32, 60) // angband16x16.bmp has 32 columns & 60 rows of
        // characters
        .size(MAP_WIDTH, MAP_HEIGHT)
        .title("Using custom character mapping with libtcod")
        .init();

    // map ASCII_CODE to character in font at position X, Y
    root.map_ascii_code_to_font(FLOOR_TILE as i32, 4, 0);
    root.map_ascii_code_to_font(WALL_TILE as i32, 13, 0);
    root.map_ascii_code_to_font(HERO_TILE as i32, 1, 18);
    root.map_ascii_code_to_font(CHEST_TILE as i32, 26, 4);
    root.map_ascii_code_to_font(ENEMY_1_TILE as i32, 15, 33);
    root.map_ascii_code_to_font(ENEMY_2_TILE as i32, 0, 48);

    root.clear();

    // fill map with wall tiles
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            root.set_char(x, y, WALL_TILE);
        }
    }

    let (starting_x, starting_y) = (30, 10);
    let (middle_x, middle_y) = (
        starting_x + (ROOM_WIDTH / 2),
        starting_y + (ROOM_HEIGHT / 2),
    );

    // draw room
    for x in starting_x..starting_x + ROOM_WIDTH {
        for y in starting_y..starting_y + ROOM_HEIGHT {
            root.set_char(x, y, FLOOR_TILE);
        }
    }

    // place special tiles
    root.set_char(middle_x, middle_y + 2, HERO_TILE);
    root.set_char(middle_x, middle_y - 3, ENEMY_1_TILE);
    root.set_char(middle_x - 3, middle_y - 3, ENEMY_1_TILE);
    root.set_char(middle_x + 3, middle_y - 3, ENEMY_1_TILE);
    root.set_char(middle_x - 2, middle_y - 5, ENEMY_2_TILE);
    root.set_char(middle_x + 2, middle_y - 5, ENEMY_2_TILE);
    root.set_char(middle_x, middle_y - 7, CHEST_TILE);

    root.flush();
    root.wait_for_keypress(true);
}
