extern crate tcod;
extern crate tcod_sys as ffi;
extern crate rand;

use tcod::console::*;
use tcod::input::*;
use tcod::system;
use tcod::colors;
use tcod::colors::Color;
use tcod::chars;
use tcod::pathfinding::{Dijkstra, AStar};
use tcod::map::{Map, FovAlgorithm};
use tcod::image;
use tcod::namegen::Namegen;
use rand::Rng;
use rand::ThreadRng;
use std::char::from_u32;
use std::fs::read_dir;

const SAMPLE_SCREEN_WIDTH : i32 = 46;
const SAMPLE_SCREEN_HEIGHT : i32 = 20;

const SAMPLE_SCREEN_X : i32 = 20;
const SAMPLE_SCREEN_Y : i32 = 10;

trait Render {
    fn render(&mut self,
              console: &mut Offscreen,
              root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> ();
}

struct ColorsSample {
    cols: [Color; 4],
    dirr: [i8; 4],
    dirg: [i8; 4],
    dirb: [i8; 4],
    rng: ThreadRng,
}

impl ColorsSample {
    fn new() -> Self {
        ColorsSample {
            cols: [Color::new(50,  40, 150),
                    Color::new(240, 85, 5),
                    Color::new(50,  35, 240),
                    Color::new(10, 200, 130)],
            dirr: [1, -1, 1, 1],
            dirg: [1, -1, -1, 1],
            dirb: [1, 1, 1, -1],
            rng: rand::thread_rng(),
        }
    }

    fn cycle_color_component(&self, color_component: u8, dir: i8) -> (u8, i8) {
        let delta : i16 = (5 * dir) as i16;
        let new_component = (color_component as i16 + delta) as u8;

        let new_dir = if new_component == 255 { -1 }
                      else if new_component == 0 { 1 }
                      else { dir };
        (new_component, new_dir)
    }

    fn cycle_colors(&mut self) -> () {
        for c in 0..4 {
            let component = self.rng.gen_range(0, 3);
            match component {
                0 => {
                    let (n_c, n_d) = self.cycle_color_component(self.cols[c].r, self.dirr[c]);
                    self.cols[c].r = n_c;
                    self.dirr[c] = n_d;
                },
                1 => {
                    let (n_c, n_d) = self.cycle_color_component(self.cols[c].g, self.dirg[c]);
                    self.cols[c].g = n_c;
                    self.dirg[c] = n_d;
                },
                2 => {
                    let (n_c, n_d) = self.cycle_color_component(self.cols[c].b, self.dirb[c]);
                    self.cols[c].b = n_c;
                    self.dirb[c] = n_d;
                },
                _ => panic!("Random number generator is broken!")
            }
        }

    }

    fn set_colors(&self, console: &mut Console) {
        enum Dir {
            TopLeft = 0,
            TopRight,
            BottomLeft,
            BottomRight,
        };

        // ==== scan the whole screen, interpolating corner colors ====
        for x in 0..SAMPLE_SCREEN_WIDTH {
            let xcoef = (x as f32) / ((SAMPLE_SCREEN_WIDTH-1) as f32);

            // get the current column top and bottom colors
            let top = colors::lerp(self.cols[Dir::TopLeft as usize],
                                   self.cols[Dir::TopRight as usize],
                                   xcoef);
            let bottom = colors::lerp(self.cols[Dir::BottomLeft as usize],
                                      self.cols[Dir::BottomRight as usize],
                                      xcoef);
            for y in 0..SAMPLE_SCREEN_HEIGHT {
                let ycoef = (y as f32) / ((SAMPLE_SCREEN_HEIGHT-1) as f32);

                // get the current cell color
                let cur_color = colors::lerp(top, bottom, ycoef);
                console.set_char_background(x, y, cur_color, BackgroundFlag::Set);
            }
        }
    }

    fn print_random_chars(&mut self, console: &mut Console) -> colors::Color {
        // ==== print the text with a random color ====
        // get the background color at the text position
        let mut text_color = console.get_char_background(SAMPLE_SCREEN_WIDTH/2, 5);
        // and invert it
        text_color.r = 255 - text_color.r;
        text_color.g = 255 - text_color.g;
        text_color.b = 255 - text_color.b;
        // put random text (for performance tests)
        for x in 0..SAMPLE_SCREEN_WIDTH {
            for y in 0..SAMPLE_SCREEN_HEIGHT {
                let mut c;
                let mut col = console.get_char_background(x, y);
                col = colors::lerp(col, colors::BLACK, 0.5);
                // use colored character 255 on first and last lines
                if y == 0 || y == SAMPLE_SCREEN_HEIGHT-1 {
                    c = std::char::from_u32(0x00ff).unwrap();
                } else {
                    let r = self.rng.gen_range('a' as u32, 'z' as u32);
                    c = from_u32(r).unwrap();
                }

                console.set_default_foreground(col);
                console.put_char(x, y, c, BackgroundFlag::None);
            }
        }

        text_color
    }
}

impl Render for ColorsSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              _event: Option<(EventFlags, Event)>) -> () {

        if first {
            system::set_fps(0);
            console.clear()
        }

        self.cycle_colors();
        self.set_colors(console);
        let text_color = self.print_random_chars(console);

        console.set_default_foreground(text_color);
        // the background behind the text is slightly darkened using the Multiply flag
        console.set_default_background(colors::GREY);
        console.print_rect_ex(SAMPLE_SCREEN_WIDTH/2, 5, SAMPLE_SCREEN_WIDTH-2, SAMPLE_SCREEN_HEIGHT-1,
                              BackgroundFlag::Multiply, TextAlignment::Center,
                              "The Doryen library uses 24 bits colors, for both background and foreground.");
    }
}

struct OffscreenSample {
    secondary : Offscreen,
    screenshot : Offscreen,
    init : bool,
    counter : i32,
    x : i32,
    y : i32,
    xdir : i32,
    ydir : i32
}

impl OffscreenSample {
    fn new() -> OffscreenSample {
        OffscreenSample {
            secondary : Offscreen::new(SAMPLE_SCREEN_WIDTH/2, SAMPLE_SCREEN_HEIGHT/2),
            screenshot : Offscreen::new(SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
            init : false,
            counter : 0,
            x : 0,
            y : 0,
            xdir : 1,
            ydir : 1
        }
    }
}

impl Render for OffscreenSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              _event: Option<(EventFlags, Event)>) -> () {
        if !self.init {
            self.init = true;
            self.secondary.print_frame(0, 0, SAMPLE_SCREEN_WIDTH/2, SAMPLE_SCREEN_HEIGHT/2,
                                        false, BackgroundFlag::Set, Some("Offscreen console"));
            self.secondary.print_rect_ex(SAMPLE_SCREEN_WIDTH/4, 2, SAMPLE_SCREEN_WIDTH/2-2,
                                          SAMPLE_SCREEN_HEIGHT/2, BackgroundFlag::None, TextAlignment::Center,
                                          "You can render to an offscreen console and blit in on another one, simulating alpha transparency.");
        }

        if first {
            system::set_fps(30);
            blit(console, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
                 &mut self.screenshot, (0, 0), 1.0, 1.0);
        }

        self.counter += 1;
        if self.counter % 20 == 0 {
            self.x += self.xdir;
            self.y += self.ydir;
            if self.x == (SAMPLE_SCREEN_WIDTH/2 + 5) { self.xdir = -1 }
            else if self.x == -5 { self.xdir = 1 }
            if self.y == (SAMPLE_SCREEN_HEIGHT/2 + 5) { self.ydir = -1 }
            else if self.y == -5 { self.ydir = 1 }
        }

        blit(&self.screenshot, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
             console, (0, 0), 1.0, 1.0);
        blit(&self.secondary, (0, 0), (SAMPLE_SCREEN_WIDTH/2, SAMPLE_SCREEN_HEIGHT/2),
             console, (self.x, self.y), 1.0, 0.75);
    }
}

/*
fn render_lines(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
fn render_noise(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
*/

struct FovSample {
    px: i32,
    py: i32,
    recompute_fov: bool,
    torch: bool,
    map: Map,
    dark_wall: colors::Color,
    light_wall: colors::Color,
    dark_ground: colors::Color,
    light_ground: colors::Color,
    // noise: Noise,
    light_walls: bool,
    algorithm: FovAlgorithm,
    _torch_x: f32,
}

fn clamp(a: f32, b: f32, x: f32) -> f32 {
    if x < a { a } else if x > b { b } else { x }
}

impl FovSample {
    fn new() -> Self {
        FovSample {
            px: 20, py: 10,
            recompute_fov: true,
            torch: false,
            map: create_map(),
            dark_wall: colors::Color::new(0, 0, 100),
            light_wall: colors::Color::new(130, 110, 50),
            dark_ground: colors::Color::new(50, 50, 150),
            light_ground: colors::Color::new(200, 180, 50),
            // noise: ,
            light_walls: true,
            algorithm: FovAlgorithm::Basic,
            _torch_x: 0.0,
        }
    }

    fn init(&mut self, console: &mut Offscreen) -> () {
        system::set_fps(30);
        console.clear();
        self.display_help(console);
        console.put_char(self.px, self.py, '@', BackgroundFlag::None);
        iterate_map(&mut |x, y, c| {
            if c == '=' {
                console.put_char(x, y, chars::DHLINE, BackgroundFlag::None)
            }
        });
    }

    fn display_help(&self, console: &mut Offscreen) -> () {
        console.set_default_foreground(colors::WHITE);
        console.print(1, 0,
                      format!("IJKL : move around\nT : torch fx {}\nW : light walls {}\n+-: algo {:11?}",
                              if self.torch { "on " } else { "off" },
                              if self.light_walls { "on "  } else { "off" },
                              self.algorithm));
        console.set_default_foreground(colors::BLACK);
    }

    fn display_map(&mut self, console: &mut Offscreen, dx: f32, dy: f32, di: f32) -> () {
        iterate_map(&mut |x, y, c| {
            let visible = self.map.is_in_fov(x, y);
            let is_wall = c == '#';
            if !visible {
                let color = if is_wall { self.dark_wall } else { self.dark_ground };
                console.set_char_background(x, y, color, BackgroundFlag::Set);
            } else {
                let mut light: colors::Color;
                if !self.torch {
                    light = if is_wall { self.light_wall } else { self.light_ground };
                } else {
                    let mut base = if is_wall { self.dark_wall } else { self.dark_ground };
                    light = if is_wall { self.light_wall } else { self.light_ground };
                    let r = (x as f32 - self.px as f32 + dx) *
                        (x as f32 - self.px as f32 + dx) +
                        (y as f32 - self.py as f32 + dy) *
                        (y as f32 - self.py as f32 + dy);
                    if r < SQUARED_TORCH_RADIUS {
                        let mut l = (SQUARED_TORCH_RADIUS - r) / SQUARED_TORCH_RADIUS + di;
                        l = clamp(0.0, 1.0, l);
                        base = colors::lerp(base, light, l);
                    }
                    light = base;
                }
                console.set_char_background(x, y, light, BackgroundFlag::Set);
            }
        });
    }

    fn handle_event<F>(&mut self, console: &mut Offscreen, clo: &mut F)
        where F: Fn(&mut FovSample) -> ()
    {
        console.put_char(self.px, self.py, ' ', BackgroundFlag::None);
        clo(self);
        console.put_char(self.px, self.py, '@', BackgroundFlag::None);
        self.recompute_fov = true;
    }

    fn next_algorithm(&mut self) -> () {
        match self.algorithm {
            FovAlgorithm::Restrictive => return,
            _ => self.algorithm = unsafe { std::mem::transmute(self.algorithm as i32 + 1) }
        };
    }

    fn previous_algorithm(&mut self) -> () {
        match self.algorithm {
            FovAlgorithm::Basic => return,
            _ => self.algorithm = unsafe { std::mem::transmute(self.algorithm as i32 - 1) }
        };
    }
}

impl Render for FovSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> () {
        if first { self.init(console) }
        if self.recompute_fov {
            self.recompute_fov = false;
            let radius = if self.torch { TORCH_RADIUS as i32 } else { 0 };
            self.map.compute_fov(self.px, self.py, radius, self.light_walls, self.algorithm);
        }

        let dx = 0.0;
        let dy = 0.0;
        let di = 0.0;
        if self.torch {
            // TODO implemenent when noise in wrapped in Rust API
        }

        self.display_map(console, dx, dy, di);

        if let Some((_, Event::Key(state))) = event {
            match state {
                Key { printable: 'i', .. } | Key { printable: 'I', .. }
                    if self.map.is_walkable(self.px, self.py - 1) =>
                    self.handle_event(console, &mut |s| s.py -= 1),
                Key { printable: 'k', .. } | Key { printable: 'K', .. }
                    if self.map.is_walkable(self.px, self.py + 1) =>
                    self.handle_event(console, &mut |s| s.py += 1),
                Key { printable: 'j', .. } | Key { printable: 'J', .. }
                    if self.map.is_walkable(self.px - 1, self.py) =>
                    self.handle_event(console, &mut |s| s.px -= 1),
                Key { printable: 'l', .. } | Key { printable: 'L', .. }
                    if self.map.is_walkable(self.px + 1, self.py) =>
                    self.handle_event(console, &mut |s| s.px += 1),
                Key { printable: 't', .. } | Key { printable: 'T', .. } => {
                    self.torch = !self.torch;
                    self.display_help(console);
                },
                Key { printable: 'w', .. } | Key { printable: 'W', .. } => {
                    self.light_walls = !self.light_walls;
                    self.display_help(console);
                    self.recompute_fov = true;
                },
                Key { printable: '+', .. } => {
                    self.next_algorithm();
                    self.display_help(console);
                    self.recompute_fov = true;
                },
                Key { printable: '-', .. } => {
                    self.previous_algorithm();
                    self.display_help(console);
                    self.recompute_fov = true;
                },
                _ => {}
            }
        }
    }
}


struct PathSample<'a> {
    px: i32,
    py: i32,
    dx: i32,
    dy: i32,
    dark_wall: colors::Color,
    dark_ground: colors::Color,
    light_ground: colors::Color,
    using_astar: bool,
    dijkstra_dist: f32,
    dijkstra: Dijkstra<'a>,
    astar: AStar<'a>,
    recalculate_path: bool,
    busy: f32,
    old_char: char,
}

const TORCH_RADIUS : f32 = 10.0;
const SQUARED_TORCH_RADIUS : f32 = (TORCH_RADIUS*TORCH_RADIUS);


static SMAP : [&'static str; 20] = [
    "##############################################",
    "#######################      #################",
    "#####################    #     ###############",
    "######################  ###        ###########",
    "##################      #####             ####",
    "################       ########    ###### ####",
    "###############      #################### ####",
    "################    ######                  ##",
    "########   #######  ######   #     #     #  ##",
    "########   ######      ###                  ##",
    "########                                    ##",
    "####       ######      ###   #     #     #  ##",
    "#### ###   ########## ####                  ##",
    "#### ###   ##########   ###########=##########",
    "#### ##################   #####          #####",
    "#### ###             #### #####          #####",
    "####           #     ####                #####",
    "########       #     #### #####          #####",
    "########       #####      ####################",
    "##############################################",
    ];

fn iterate_map<F>(closure: &mut F) -> ()
    where F: FnMut(i32, i32, char) -> ()
{
    for (y, line) in SMAP.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            closure(x as i32, y as i32, c)
        }
    }
}

fn create_map() -> Map {
    let mut map = Map::new(SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT);
    iterate_map(&mut |x, y, c| {
        if c == ' ' { map.set(x, y, true, true) } // ground
        else if c == '=' { map.set(x, y, true, false) } // window
    });
    map
}



impl<'a> PathSample<'a> {

    fn new() -> Self {
        PathSample {
            px: 20, py: 10,
            dx: 24, dy: 1,
            dark_wall: colors::Color::new(0, 0, 100),
            dark_ground: colors::Color::new(50, 50, 150),
            light_ground: colors::Color::new(200, 180, 50),
            using_astar: true,
            dijkstra_dist: 0.0,
            dijkstra: Dijkstra::new_from_map(create_map(), 1.41f32),
            astar: AStar::new_from_map(create_map(), 1.41f32),
            recalculate_path: false,
            busy: 0.0,
            old_char: ' ',
        }
    }

    fn init(&mut self, console: &mut Offscreen) {
        system::set_fps(30);
        console.clear();
        // we draw the foreground only the first time.
        // during the player movement, only the @ is redrawn.
        // the rest impacts only the background color
        // draw the help text & player @
        console.set_default_foreground(colors::WHITE);
        console.put_char(self.dx, self.dy, '+', BackgroundFlag::None);
        console.put_char(self.px, self.py, '@', BackgroundFlag::None);
        console.print(1, 1, "IJKL / mouse :\nmove destination\nTAB : A*/dijkstra");
        console.print(1, 4, "Using : A*");

        // draw windows
        iterate_map(&mut |x, y, c| {
            if c == '=' {
                console.put_char(x, y, chars::DHLINE, BackgroundFlag::None)
            }
        });
        self.recalculate_path = true;
    }

    fn display_map(&mut self, console: &mut Offscreen) -> () {
        iterate_map(&mut |x, y, c| {
            let wall = c == '#';
            let color = if wall { self.dark_wall} else { self.dark_ground };
            console.set_char_background(x, y, color, BackgroundFlag::Set);
        });
    }

    fn recalculate(&mut self) -> () {
        if self.using_astar {
            self.astar.find((self.px, self.py), (self.dx, self.dy));
        } else {
            self.dijkstra_dist = 0.0;
            self.dijkstra.compute_grid((self.px, self.py));
            iterate_map(&mut |x, y, _c| {
                let d = self.dijkstra.distance_from_root((x, y));
                match d {
                    Some(d) if d > self.dijkstra_dist => self.dijkstra_dist = d,
                    _ => {},
                }
            });
            self.dijkstra.find((self.dx, self.dy));
        }
        self.recalculate_path = false;
        self.busy = 0.2;
    }

    fn draw_path(&mut self, console: &mut Offscreen) -> () {
        if self.using_astar {
            for i in 0..self.astar.len() {
                let (x, y) = self.astar.get(i).unwrap();
                console.set_char_background(x, y, self.light_ground, BackgroundFlag::Set);
            }
        } else {
            iterate_map(&mut |x, y, c| {
                let wall = c == '#';
                if !wall {
                    let d = self.dijkstra.distance_from_root((x, y));
                    if let Some(d) = d {
                        let color = colors::lerp(self.light_ground,
                                                 self.dark_ground,
                                                 0.9 * d / self.dijkstra_dist);
                        console.set_char_background(x, y, color, BackgroundFlag::Set);
                    }
                }
            });
            for i in 0..self.dijkstra.len() {
                let (x, y) = self.dijkstra.get(i).unwrap();
                console.set_char_background(x, y, self.light_ground, BackgroundFlag::Set);
            }
        }
    }

    fn move_creature(&mut self, console: &mut Offscreen) -> () {
        self.busy = 0.2;
        if self.using_astar {
            if ! self.astar.is_empty() {
                console.put_char(self.px, self.py, ' ', BackgroundFlag::None);
                let (x, y) = self.astar.walk_one_step(true).unwrap();
                self.px = x;
                self.py = y;
                console.put_char(self.px, self.py, '@', BackgroundFlag::None);
            }
        } else {
            if !self.dijkstra.is_empty() {
                console.put_char(self.px, self.py, ' ', BackgroundFlag::None);
                let (x, y) = self.dijkstra.walk_one_step().unwrap();
                self.px = x;
                self.py = y;
                console.put_char(self.px, self.py, '@', BackgroundFlag::None);
                self.recalculate_path = true;
            }
        }
    }

    fn handle_event<F>(&mut self, console: &mut Offscreen, clo: &mut F)
        where F: Fn(&mut PathSample) -> ()
    {
        console.put_char(self.dx, self.dy, self.old_char, BackgroundFlag::None);
        clo(self);
        self.old_char = console.get_char(self.dx, self.dy);
        console.put_char(self.dx, self.dy, '+', BackgroundFlag::None);
        if SMAP[self.dy as usize].chars().nth(self.dx as usize).unwrap() == ' ' {
            self.recalculate_path = true;
        }
    }
}

impl<'a> Render for PathSample<'a> {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> () {
        if first { self.init(console) }
        if self.recalculate_path { self.recalculate() }

        self.display_map(console);
        self.draw_path(console);

        self.busy -= system::get_last_frame_length();
        if self.busy < 0.0 {
            self.move_creature(console);
        }

        if let Some((_, Event::Key(state))) = event {
            match state {
                Key { printable: 'i', .. } | Key { printable: 'I', .. } if self.dy > 0 =>
                    self.handle_event(console, &mut |s| s.dy -= 1),
                Key { printable: 'k', .. } | Key { printable: 'K', .. } if self.dy < SAMPLE_SCREEN_HEIGHT-1 =>
                    self.handle_event(console, &mut |s| s.dy += 1),
                Key { printable: 'j', .. } | Key { printable: 'J', .. } if self.dx > 0 =>
                    self.handle_event(console, &mut |s| s.dx -= 1),
                Key { printable: 'l', .. } | Key { printable: 'L', .. } if self.dx < SAMPLE_SCREEN_WIDTH-1 =>
                    self.handle_event(console, &mut |s| s.dx += 1),
                Key { code: KeyCode::Tab, .. } => {
                    self.using_astar = ! self.using_astar;
                    if self.using_astar {
                        console.print(1, 4, "Using : A*      ");
                    } else {
                        console.print(1, 4, "Using : Dijkstra");
                    }
                    self.recalculate_path = true;
                }
                _ => {}
            }
        }
        if let Some((_, Event::Mouse(state))) = event {
            let mx: i32 = state.cx as i32 - SAMPLE_SCREEN_X;
            let my: i32 = state.cy as i32 - SAMPLE_SCREEN_Y;
            if  mx >= 0 && mx < SAMPLE_SCREEN_WIDTH &&
                my >= 0 && my < SAMPLE_SCREEN_HEIGHT &&
                (self.dx != mx || self.dy != my)
            {
                self.handle_event(console, &mut |s| {
                    s.dx = mx;
                    s.dy = my;
                });
            }
        }
    }
}

/*
fn render_bsp(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
 */

struct ImageSample {
    img: image::Image,
    circle: image::Image,
    blue: colors::Color,
    green: colors::Color,
}

impl ImageSample {
    fn new() -> Self {
        let mut i = ImageSample {
            img: image::Image::from_file("data/img/skull.png")
                .ok()
                .expect("Could not load data/img/skull.png"),
            circle: image::Image::from_file("data/img/circle.png")
                .ok()
                .expect("Could not load data/img/circle.png"),
            blue: colors::Color::new(0, 0, 255),
            green: colors::Color::new(0, 255, 0),
        };
        i.img.set_key_color(colors::BLACK);
        i
    }
}

impl Render for ImageSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              _event: Option<(EventFlags, Event)>) -> () {
        if first {
            system::set_fps(30)
        }

        console.set_default_background(colors::BLACK);
        console.clear();

        let elapsed_seconds: f32 = (system::get_elapsed_time().num_milliseconds() as f32) / 1000.0;
        let x = (SAMPLE_SCREEN_WIDTH/2) as f32 + (elapsed_seconds as f32).cos() * 10.0;
        let y = ( SAMPLE_SCREEN_HEIGHT/2 ) as f32;
        let scale_x = 0.2 + 1.8 * (1.0 + (elapsed_seconds / 2.0).cos()) / 2.0;
        let scale_y = scale_x;
        let angle = elapsed_seconds;
        let elapsed = system::get_elapsed_time().num_milliseconds() / 2000;

        if (elapsed & 1) != 0 {
            // split the color channels of circle.png
            // the red channel
            console.set_default_background(colors::RED);
            console.rect(0, 3, 15, 15, false, BackgroundFlag::Set);
            image::blit_rect(&self.circle, (-1, -1), console, (0, 3), BackgroundFlag::Multiply);
            // the green channel
            console.set_default_background(self.green);
            console.rect(15, 3, 15, 15, false, BackgroundFlag::Set);
            image::blit_rect(&self.circle, (-1, -1), console, (15, 3), BackgroundFlag::Multiply);
            // the blue channel
            console.set_default_background(self.blue);
            console.rect(30, 3, 15, 15, false, BackgroundFlag::Set);
            image::blit_rect(&self.circle, (-1, -1), console, (30, 3), BackgroundFlag::Multiply);
        } else {
            image::blit_rect(&self.circle, (-1, -1), console, ( 0, 3), BackgroundFlag::Set);
            image::blit_rect(&self.circle, (-1, -1), console, (15, 3), BackgroundFlag::Set);
            image::blit_rect(&self.circle, (-1, -1), console, (30, 3), BackgroundFlag::Set);
        }

        image::blit(&self.img, (scale_x, scale_y), angle, console, (x, y), BackgroundFlag::Set);
    }
}


struct MouseSample {
    left_button:   bool,
    middle_button: bool,
    right_button:  bool,
    mouse_state: Option<MouseState>,
}

impl MouseSample {
    fn new() -> Self {
        MouseSample { left_button: false, middle_button: false, right_button: false,
                      mouse_state: None,
        }
    }

    fn format(&self, mouse: &MouseState, root: &Root) -> String {
        format!("{}\n \
                 Mouse position : {:4}x{:4} {}\n \
                 Mouse cell     : {:4}x{:4}\n \
                 Mouse movement : {:4}x{:4}\n \
                 Left button    : {} (toggle {})\n \
                 Right button   : {} (toggle {})\n \
                 Middle button  : {} (toggle {})\n \
                 Wheel          : {}\n",
                if root.is_active() {""} else {"APPLICATION INACTIVE"},
                mouse.x, mouse.y,
                if root.has_focus() {""} else {"OUT OF FOCUS"},
                mouse.cx, mouse.cy,
                mouse.dx, mouse.dy,
                if mouse.lbutton { " ON" } else { "OFF" },
                if self.left_button { " ON" } else { "OFF" },
                if mouse.rbutton { " ON" } else { "OFF" },
                if self.right_button { " ON" } else { "OFF" },
                if mouse.mbutton { " ON" } else { "OFF" },
                if self.middle_button { " ON" } else { "OFF" },
                if mouse.wheel_up { "UP" } else if mouse.wheel_down { "DOWN" } else { "" })
    }

    fn init(&mut self, console: &mut Console) -> () {
        system::set_fps(30);
        console.set_default_background(colors::GREY);
        console.set_default_foreground(colors::LIGHT_YELLOW);
        move_cursor(320, 200);
        show_cursor(true)
    }
}

impl Render for MouseSample {
    fn render(&mut self,
              console: &mut Offscreen,
              root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> () {
        if first {
            self.init(console)
        }

        console.clear();
        match event {
            Some((_, Event::Mouse(mouse))) => {
                if mouse.lbutton_pressed {self.left_button = !self.left_button;}
                if mouse.mbutton_pressed {self.middle_button = !self.middle_button;}
                if mouse.rbutton_pressed {self.right_button = !self.right_button;}

                self.mouse_state = Some(mouse);
            },
            Some((_, Event::Key(Key {code: KeyCode::Number1, ..}))) => {
                show_cursor(false);
            },
            Some((_, Event::Key(Key {code: KeyCode::Number2, ..}))) => {
                show_cursor(true)
            },
            _ => {} // Ignore other events
        }

        if let Some(mouse) = self.mouse_state {
            console.print(1, 1, self.format(&mouse, root));
        }

        console.print(1, 10, "1 : Hide cursor\n2 : Show cursor");
    }
}

struct NameSample {
    sets: Vec<String>,
    cur_set: usize,
    delay: f32,
    names: Vec<String>,
    name_gen: Namegen,
}

impl NameSample {
    fn new() -> Self {
        let mut n = Namegen::new().unwrap();

        let entries = read_dir("data/namegen").ok().expect("Could not read data/namegen");
        for entry in entries {
            match entry {
                Ok(e) => {
                    let path = e.path();
                    let extension = path.extension().unwrap().to_str().unwrap();
                    if extension.ends_with("cfg") {
                        n.parse(&path);
                    }
                },
                _ => {}
            }
        };
        NameSample { sets: n.get_sets(),
                     cur_set: 0,
                     delay: 0.0,
                     names: vec![],
                     name_gen: n,
        }
    }

    fn display_names(&self, console: &mut Offscreen) {
        console.set_default_background(colors::LIGHT_BLUE);
        console.clear();
        console.set_default_foreground(colors::WHITE);
        console.print(1, 1, format!("{}\n\n+ : next generator\n- : prev generator",
                                    self.sets[self.cur_set]));
        for (i, name) in self.names.iter().enumerate() {
            if (name.len() as i32) < SAMPLE_SCREEN_WIDTH {
                console.print_ex(SAMPLE_SCREEN_WIDTH - 2, 2 + i as i32,
                                 BackgroundFlag::None, TextAlignment::Right, name)
            }
        }
    }

    fn limit_names(&mut self) -> () {
        while self.names.len() >= 15 {
            self.names.remove(0);
        }
    }
}

impl Render for NameSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> () {
        if first {
            system::set_fps(30);
        }

        self.limit_names();
        self.display_names(console);

        self.delay += system::get_last_frame_length();
        if self.delay >= 0.5 {
            let name = &self.sets[self.cur_set];

            self.delay -= 0.5;
            self.names.push(self.name_gen.generate(name).unwrap())
        }

        match event {
            Some((_, Event::Key(state))) => {
                match state {
                    Key { printable: '+', .. } => {
                        self.cur_set += 1;
                        if self.cur_set == self.sets.len() { self.cur_set = 0 }
                        self.names.push("======".to_string());
                    },
                    Key { printable: '-', .. } => {
                        if self.cur_set == 0 {
                            self.cur_set = self.sets.len() - 1
                        } else {
                            self.cur_set -= 1;
                        }
                        self.names.push("======".to_string());
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

/*
fn render_sdl(_console: &mut Offscreen, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
*/

struct MenuItem<'a> {
    name: String,
    render: &'a mut Render
}

impl<'a> MenuItem<'a> {
    fn new(name: &str, render: &'a mut Render) -> Self {
        MenuItem { name: name.to_string(), render: render }
    }
}

struct Options {
    fullscreen_width: i32,
    fullscreen_height: i32,
    font: String,
    font_type: FontType,
    font_layout: FontLayout,
    nb_char_horiz: i32,
    nb_char_vertic: i32,
    fullscreen: bool,
    renderer: Renderer,
}

impl Options {
    fn new() -> Self {
        Options {fullscreen_width: 0,
                 fullscreen_height: 0,
                 font: "consolas10x10_gs_tc.png".to_string(),
                 font_type: FontType::Greyscale,
                 font_layout: FontLayout::Tcod,
                 nb_char_horiz: 0,
                 nb_char_vertic: 0,
                 fullscreen: false,
                 renderer: Renderer::SDL,
        }
    }
}

fn main() {
    let mut colors = ColorsSample::new();
    let mut offscreen = OffscreenSample::new();
    let mut mouse = MouseSample::new();
    let mut path_sample = PathSample::new();
    let mut fov = FovSample::new();
    let mut image_sample = ImageSample::new();
    let mut names = NameSample::new();
    let mut samples = vec![MenuItem::new("  True colors      ", &mut colors),
                           MenuItem::new("  Offscreen console", &mut offscreen),
                           // MenuItem::new("  Line drawing     ", &mut ),
                           // MenuItem::new("  Noise            ", &mut ),
                           MenuItem::new("  Field of view    ", &mut fov),
                           MenuItem::new("  Path finding     ", &mut path_sample),
                           // MenuItem::new("  Bsp toolkit      ", &mut ),
                           MenuItem::new("  Image toolkit    ", &mut image_sample),
                           MenuItem::new("  Mouse support    ", &mut mouse),
                           MenuItem::new("  Name generator   ", &mut names),
                           // MenuItem::new("  SDL callback     ", &mut ),
                           ];
    let mut cur_sample = 0;
    let mut options = Options::new();
    let mut first = true;
    let mut console = Offscreen::new(SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT);

    parse_args(&mut options);

    if options.fullscreen_width > 0 {
        system::force_fullscreen_resolution(options.fullscreen_width,
                                            options.fullscreen_height);
    }
    let mut root = Root::initializer()
        .size(80, 50)
        .title("libtcod Rust sample")
        .fullscreen(options.fullscreen)
        .renderer(options.renderer)
        .font(options.font, options.font_layout)
        .font_type(options.font_type)
        .font_dimensions(options.nb_char_horiz, options.nb_char_vertic)
        .init();
    let mut credits_end = false;

    while !root.window_closed() {
        if !credits_end {
            credits_end = root.render_credits(60, 43, false);
        }

        print_samples(&mut root, cur_sample, &samples);
        print_help_message(&mut root);

        let event = check_for_event(KEY_PRESS | MOUSE);

        {
            // Scope to limit mutable borrow
            let mut r = &mut samples[cur_sample].render;
            r.render(&mut console, &root, first, event);
        }

        first = false;
        blit(&console, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
             &mut root, (SAMPLE_SCREEN_X, SAMPLE_SCREEN_Y), 1.0, 1.0);

        // erase the renderer in debug mode (needed because the root
        // console is not cleared each frame)
        root.print(1, 1, "        ");

        root.flush();
        match event {
            None => {continue;}
            Some((_flag, Event::Key(state))) => {
                match state {
                    Key { code: KeyCode::Down, .. } => {
                        cur_sample = (cur_sample + 1) % samples.len();
                        first = true
                    }
                    Key { code: KeyCode::Up, .. } => {
                        if cur_sample == 0 { cur_sample = samples.len() - 1; }
                        else { cur_sample -= 1; }
                        first = true
                    }
                    Key { code: KeyCode::Enter, .. } if state.left_alt => {
                        let fullscreen = root.is_fullscreen();
                        root.set_fullscreen(!fullscreen)
                    }
                    Key { code: KeyCode::PrintScreen, .. } => {
                        // TODO
                    }
                    Key { code: KeyCode::Escape, .. } => { break }
                    _ => {continue;}
                }
            }
            _ => {continue;}
        }
    }
}

fn print_samples(root: &mut Root, cur_sample: usize, samples: &Vec<MenuItem>) -> () {
    for i in 0..samples.len() {
        if i == cur_sample {
            root.set_default_foreground(colors::WHITE);
            root.set_default_background(colors::LIGHT_BLUE);
        } else {
            root.set_default_foreground(colors::GREY);
            root.set_default_background(colors::BLACK);
        }
        let y : i32 = 46 - (samples.len() as i32 - i as i32);
        let fun = &samples[i].name; //.name;
        root.print_ex(2, y, BackgroundFlag::Set, TextAlignment::Left, fun);
    }

}

fn print_help_message(root: &mut Root) -> () {
    root.set_default_foreground(colors::GREY);
    root.print_ex(79, 46, BackgroundFlag::None, TextAlignment::Right,
                  format!("last frame : {:3.0} ms ({:3} fps)",
                          system::get_last_frame_length() * 1000.0,
                          system::get_fps()));
    let time = system::get_elapsed_time();
    root.print_ex(79, 47, BackgroundFlag::None, TextAlignment::Right,
                  format!("elapsed {:8}ms {:4.2}s",
                          time.num_milliseconds(),
                          time.num_milliseconds() as f32/ 1000.0));
    root.print(2, 47, format!("{}{} : select a sample",
                              chars::ARROW_N, chars::ARROW_S));
    let fullscreen_text = if root.is_fullscreen() {"windowed mode"}
    else {"fullscren_mode"};
    root.print(2, 48, format!("ALT-ENTER : switch to {}", fullscreen_text));
}

fn parse_args(options: &mut Options) {
    let mut args = std::env::args();

    loop {
        use std::i32;
        use std::str::FromStr;

        match args.next() {
            None => break,
            Some(opt) => match opt.as_ref() {
                "-font" => {
                    let n = args.next();
                    if n.is_some() {
                        options.font = n.unwrap()
                    }
                },
                "-font-nb-char" => {
                    let horiz = args.next();
                    let vertic = args.next();
                    if horiz.is_some() {
                        options.nb_char_horiz = i32::from_str(horiz.unwrap().as_ref()).unwrap()
                    }
                    if vertic.is_some() {
                        options.nb_char_vertic = i32::from_str(vertic.unwrap().as_ref()).unwrap()
                    }
                }
                "-fullscreen-resolution" => {
                    let width  = args.next();
                    let height = args.next();
                    if width.is_some() {
                        options.fullscreen_width = i32::from_str(width.unwrap().as_ref()).unwrap()
                    }
                    if height.is_some() {
                        options.fullscreen_height = i32::from_str(height.unwrap().as_ref()).unwrap()
                    }
                }
                "-fullscreen" => options.fullscreen = true,
                "-font-in-row" => options.font_layout = FontLayout::AsciiInRow,
                "-font-greyscale" => options.font_type = FontType::Greyscale,
                "-font-tcod" => options.font_layout = FontLayout::Tcod,
                "-renderer" => {
                    let r = args.next();
                    if r.is_some() {
                        match i32::from_str(r.unwrap().as_ref()).unwrap() {
                            0 => options.renderer = Renderer::GLSL,
                            1 => options.renderer = Renderer::OpenGL,
                            2 => options.renderer = Renderer::SDL,
                            _ => {
                                println!("Invalid renderer");
                                std::process::exit(1)
                            }
                        }
                    }
                }
                "-help" | "-?" => {
                    println!("options :");
                    println!("-font <filename> : use a custom font");
                    println!("-font-nb-char <nb_char_horiz> <nb_char_vertic> : number of characters in the font");
                    println!("-font-in-row : the font layout is in row instead of columns");
                    println!("-font-tcod : the font uses TCOD layout instead of ASCII");
                    println!("-font-greyscale : antialiased font using greyscale bitmap");
                    println!("-fullscreen : start in fullscreen");
                    println!("-fullscreen-resolution <screen_width> <screen_height> : force fullscreen resolution");
                    println!("-renderer <num> : set renderer. 0 : GLSL 1 : OPENGL 2 : SDL");
                    std::process::exit(0)
                }
                _ => continue
            }
        }
    }

}
