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
use tcod::line::Line;
use tcod::noise::{Noise, NoiseType, DEFAULT_HURST, DEFAULT_LACUNARITY, MAX_OCTAVES};
use tcod::image::{Image, blit_2x};
use tcod::bsp::{Bsp, TraverseOrder};
use rand::Rng;
use rand::ThreadRng;
use std::char::from_u32;
use std::fs::read_dir;
use std::cmp::{min, max};
use std::time::Duration;

const SAMPLE_SCREEN_WIDTH : i32 = 46;
const SAMPLE_SCREEN_HEIGHT : i32 = 20;

const SAMPLE_SCREEN_X : i32 = 20;
const SAMPLE_SCREEN_Y : i32 = 10;

trait Render {
    fn initialize(&mut self, console: &mut Offscreen);

    fn render(&mut self, console: &mut Offscreen, root: &Root,
              event: Option<(EventFlags, Event)>);
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

    fn cycle_colors(&mut self) {
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
                let mut col = console.get_char_background(x, y);
                col = colors::lerp(col, colors::BLACK, 0.5);
                // use colored character 255 on first and last lines
                let c = if y == 0 || y == SAMPLE_SCREEN_HEIGHT-1 {
                    '\u{00ff}'
                } else {
                    let r = self.rng.gen_range('a' as u32, 'z' as u32);
                    from_u32(r).unwrap()
                };

                console.set_default_foreground(col);
                console.put_char(x, y, c, BackgroundFlag::None);
            }
        }

        text_color
    }
}

impl Render for ColorsSample {
    fn initialize(&mut self, console: &mut Offscreen) {
        system::set_fps(0);
        console.clear()
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              _event: Option<(EventFlags, Event)>) {
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
    fn initialize(&mut self, console: &mut Offscreen) {
        if !self.init {
            self.init = true;
            self.secondary.print_frame(0, 0, SAMPLE_SCREEN_WIDTH/2, SAMPLE_SCREEN_HEIGHT/2,
                                       false, BackgroundFlag::Set, Some("Offscreen console"));
            self.secondary.print_rect_ex(SAMPLE_SCREEN_WIDTH/4, 2, SAMPLE_SCREEN_WIDTH/2-2,
                                         SAMPLE_SCREEN_HEIGHT/2, BackgroundFlag::None, TextAlignment::Center,
                                         "You can render to an offscreen console\
                                          and blit in on another one, simulating\
                                          alpha transparency.");
        }

        system::set_fps(30);
        blit(console, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
             &mut self.screenshot, (0, 0), 1.0, 1.0);
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              _event: Option<(EventFlags, Event)>) {
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

struct LineSample {
    bk_flag: BackgroundFlag,
    bk: Offscreen,
}

impl LineSample {
    fn new() -> Self {
        let mut line = LineSample {
            bk_flag: BackgroundFlag::Set,
            bk: Offscreen::new(SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
        };

        for x in 0..SAMPLE_SCREEN_WIDTH {
            for y in 0..SAMPLE_SCREEN_HEIGHT {
                let col = colors::Color::new(
                    (x * 255 / (SAMPLE_SCREEN_WIDTH - 1)) as u8,
                    ((x+y) * 255 / (SAMPLE_SCREEN_WIDTH + SAMPLE_SCREEN_HEIGHT - 2)) as u8,
                    (y * 255 / (SAMPLE_SCREEN_HEIGHT - 1)) as u8);
                line.bk.set_char_background(x, y, col, BackgroundFlag::Set);
            }
        };
        line
    }

    fn next_flag(&mut self, flag_byte: i32) {
        let max = BackgroundFlag::Default as i32;
        if flag_byte >= max - 1 {
            self.bk_flag = BackgroundFlag::None;
        } else {
            self.bk_flag = unsafe {
                std::mem::transmute(flag_byte + 1)
            }
        }
    }

    fn set_alpha(&mut self, elapsed_seconds: f32, flag: BackgroundFlag) {
        let alpha = (1.0 + (elapsed_seconds*2.0).cos()) / 2.0;
        self.bk_flag = unsafe {
            let alpha_value = ((alpha*255.0) as u32) <<8;
            let new_flag = flag as u32 | alpha_value;
            std::mem::transmute(new_flag)
        };
    }
}

fn seconds_from_duration(duration: Duration) -> f32 {
    duration.as_secs() as f32 + (duration.subsec_nanos() as f32 / 1_000_000_000.0)
}

impl Render for LineSample {
    fn initialize(&mut self, console: &mut Offscreen) {
        system::set_fps(30);
        console.set_default_foreground(colors::WHITE);
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        let elapsed_seconds = seconds_from_duration(system::get_elapsed_time());
        let flag_byte = self.bk_flag as i32 & 0xff;
        if flag_byte == BackgroundFlag::Alph as i32 {
            self.set_alpha(elapsed_seconds, BackgroundFlag::Alph);
        }
        if flag_byte == BackgroundFlag::AddA as i32 {
            self.set_alpha(elapsed_seconds, BackgroundFlag::AddA);
        }

        blit(&self.bk, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
             console, (0, 0), 1.0, 1.0);
        let rect_y = ((SAMPLE_SCREEN_HEIGHT - 2) as f32 * ((1.0 + elapsed_seconds.cos()) / 2.0)) as i32;
        for x in 0..SAMPLE_SCREEN_WIDTH {
            let component = (x * 255 / SAMPLE_SCREEN_WIDTH) as u8;
            let col = colors::Color::new(component, component, component);
            console.set_char_background(x, rect_y,   col, self.bk_flag);
            console.set_char_background(x, rect_y+1, col, self.bk_flag);
            console.set_char_background(x, rect_y+2, col, self.bk_flag);
        }

        let angle = elapsed_seconds * 2.0;
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let xo = ((SAMPLE_SCREEN_WIDTH / 2) as f32 * (1.0 + cos_angle)) as i32;
        let yo = ((SAMPLE_SCREEN_HEIGHT / 2) as f32 +
                  sin_angle * (SAMPLE_SCREEN_WIDTH / 2) as f32) as i32;
        let xd = ((SAMPLE_SCREEN_WIDTH / 2) as f32 * (1.0 - cos_angle)) as i32;
        let yd = ((SAMPLE_SCREEN_HEIGHT / 2) as f32 -
                  sin_angle * (SAMPLE_SCREEN_WIDTH / 2) as f32) as i32;

        let line = Line::new((xo, yo), (xd, yd));
        for (x, y) in line {
            if x >= 0 && y >=0 && x < SAMPLE_SCREEN_WIDTH && y < SAMPLE_SCREEN_HEIGHT {
                console.set_char_background(x, y, colors::LIGHT_BLUE, self.bk_flag);
            }
        }

        let display_flag : BackgroundFlag = unsafe {
            std::mem::transmute(flag_byte)
        };
        console.print(2, 2, format!("{:?} (ENTER to change)", display_flag));

        if let Some((_, Event::Key(key))) = event {
            match key.code {
                KeyCode::Enter => self.next_flag(flag_byte),
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NoiseFunction {
    Perlin = 0,
    Simplex,
    Wavelet,
    FbmPerlin,
    TurbulencePerlin,
    FbmSimplex,
    TurbulenceSimplex,
    FbmWavelet,
    TurbulenceWavelet,
}

static VALUES: &'static [NoiseFunction] = &[
    NoiseFunction::Perlin,
    NoiseFunction::Simplex,
    NoiseFunction::Wavelet,
    NoiseFunction::FbmPerlin,
    NoiseFunction::TurbulencePerlin,
    NoiseFunction::FbmSimplex,
    NoiseFunction::TurbulenceSimplex,
    NoiseFunction::FbmWavelet,
    NoiseFunction::TurbulenceWavelet];

struct NoiseFunctionIterator {
    val: usize
}

impl NoiseFunction {
    fn iter() -> NoiseFunctionIterator {
        NoiseFunctionIterator { val: 0 }
    }

    fn from_value(val: u8) -> Self {
        match val as usize {
            x if x < VALUES.len() => VALUES[x],
            _ => panic!("Wrong value to convert to NoiseFunction")
        }
    }
}

impl Iterator for NoiseFunctionIterator {
    type Item = NoiseFunction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.val {
            x if x < VALUES.len() => {
                let retval = VALUES[self.val];
                self.val += 1;
                Some(retval)
            },
            _ => None,
        }
    }
}

static FUNC_NAMES: [&'static str; 9] = [
    "1 : perlin noise       ",
    "2 : simplex noise      ",
    "3 : wavelet noise      ",
    "4 : perlin fbm         ",
    "5 : perlin turbulence  ",
    "6 : simplex fbm        ",
    "7 : simplex turbulence ",
    "8 : wavelet fbm        ",
    "9 : wavelet turbulence ",
];

struct NoiseSample {
    func: NoiseFunction,
    noise: Noise,
    dx: f32,
    dy: f32,
    octaves: u32,
    hurst: f32,
    lacunarity: f32,
    img: Image,
    zoom: f32
}

impl NoiseSample {
    fn new() -> Self {
        let noise = Noise::init_with_dimensions(2)
            .hurst(DEFAULT_HURST)
            .lacunarity(DEFAULT_LACUNARITY)
            .init();
        NoiseSample {
            func: NoiseFunction::Perlin,
            noise: noise,
            dx: 0.0,
            dy: 0.0,
            octaves: 4,
            hurst: DEFAULT_HURST,
            lacunarity: DEFAULT_LACUNARITY,
            img: Image::new(SAMPLE_SCREEN_WIDTH * 2, SAMPLE_SCREEN_HEIGHT * 2),
            zoom: 3.0
        }
    }

    fn new_noise(&self) -> Noise {
        Noise::init_with_dimensions(2)
            .hurst(self.hurst)
            .lacunarity(self.lacunarity)
            .init()
    }

    fn draw_noise(&mut self) {
        for y in 0..2*SAMPLE_SCREEN_HEIGHT {
            for x in 0..2*SAMPLE_SCREEN_WIDTH {
                let x0 = self.zoom * x as f32 / (2 * SAMPLE_SCREEN_WIDTH) as f32 + self.dx;
                let y0 = self.zoom * y as f32 / (2 * SAMPLE_SCREEN_HEIGHT) as f32 + self.dy;
                let mut coords = [x0, y0];
                let value = match self.func {
                    NoiseFunction::Perlin =>
                        self.noise.get_ex(&mut coords, NoiseType::Perlin),
                    NoiseFunction::Simplex =>
                        self.noise.get_ex(&mut coords, NoiseType::Simplex),
                    NoiseFunction::Wavelet =>
                        self.noise.get_ex(&mut coords, NoiseType::Wavelet),
                    NoiseFunction::FbmPerlin =>
                        self.noise.get_fbm_ex(&mut coords, self.octaves, NoiseType::Perlin),
                    NoiseFunction::TurbulencePerlin =>
                        self.noise.get_turbulence_ex(&mut coords, self.octaves, NoiseType::Perlin),
                    NoiseFunction::FbmSimplex =>
                        self.noise.get_fbm_ex(&mut coords, self.octaves, NoiseType::Simplex),
                    NoiseFunction::TurbulenceSimplex =>
                        self.noise.get_turbulence_ex(&mut coords, self.octaves, NoiseType::Simplex),
                    NoiseFunction::FbmWavelet =>
                        self.noise.get_fbm_ex(&mut coords, self.octaves, NoiseType::Wavelet),
                    NoiseFunction::TurbulenceWavelet =>
                        self.noise.get_turbulence_ex(&mut coords, self.octaves, NoiseType::Wavelet),
                };

                let c: u8 = ((value + 1.0) / 2.0 * 255.0) as u8;
                let color = colors::Color::new(c/2, c/2, c);
                self.img.put_pixel(x, y, color);
            }
        }
    }

    fn draw_rectangle(&self, console: &mut Offscreen) {
        console.set_default_background(colors::GREY);
        let height = if self.func as u32 <= NoiseType::Wavelet as u32 {10} else {13};
        console.rect(2, 2, 23, height, false, BackgroundFlag::Multiply);
        for y in 2..(2+height) {
            for x in 2..25 {
                let old_col = console.get_char_foreground(x, y);
                let color = old_col * colors::GREY;
                console.set_char_foreground(x, y, color);
            }
        }
    }

    fn draw_menu(&self, console: &mut Offscreen) {
        for cur_func in NoiseFunction::iter() {
            if self.func == cur_func {
                console.set_default_foreground(colors::WHITE);
                console.set_default_background(colors::LIGHT_BLUE);
                console.print_ex(2, 2 + cur_func as i32, BackgroundFlag::Set, TextAlignment::Left,
                                 FUNC_NAMES[cur_func as usize]);
            } else {
                console.set_default_foreground(colors::GREY);
                console.print(2, 2 + cur_func as i32, FUNC_NAMES[cur_func as usize]);
            }
        }

        console.set_default_foreground(colors::WHITE);
        console.print(2, 11, format!("Y/H : zoom({:2.1})", self.zoom));
        if self.func > NoiseFunction::Wavelet {
            console.print(2, 12, format!("E/D : hurst ({:2.1})", self.hurst));
            console.print(2, 13, format!("R/F : lacunarity ({:2.1})", self.lacunarity));
            console.print(2, 14, format!("T/G : octaves ({})", self.octaves));
        }

    }
}

impl Render for NoiseSample {
    fn initialize(&mut self, _console: &mut Offscreen) {
        system::set_fps(30);
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        self.dx += 0.01;
        self.dy += 0.01;

        self.draw_noise();
        blit_2x(&self.img, (0, 0), (-1, -1), console, (0, 0));

        self.draw_rectangle(console);
        self.draw_menu(console);

        if let Some((_, Event::Key(key))) = event {
            match key.printable {
                '1'...'9' =>
                    self.func = {
                        let number = key.printable.to_digit(10).unwrap() as u8;
                        NoiseFunction::from_value(number - 1)
                    },
                'e' | 'E' => {
                    self.hurst += 0.1;
                    self.noise = self.new_noise();
                },
                'd' | 'D' => {
                    self.hurst -= 0.1;
                    self.noise = self.new_noise();
                },
                'r' | 'R' => {
                    self.lacunarity += 0.5;
                    self.noise = self.new_noise();
                },
                'f' | 'F' => {
                    self.lacunarity -= 0.5;
                    self.noise = self.new_noise();
                },
                't' | 'T' => if self.octaves < MAX_OCTAVES - 1 {
                    self.octaves += 1;
                },
                'g' | 'G' => if self.octaves > 1 {
                    self.octaves -= 1
                },
                'y' | 'Y' => self.zoom += 0.2,
                'h' | 'H' => self.zoom -= 0.2,
                _ => {}
            }
        }
    }
}

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
    noise: Noise,
    light_walls: bool,
    algorithm: FovAlgorithm,
    torch_x: f32,
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
            noise: Noise::init_with_dimensions(1).init(),
            light_walls: true,
            algorithm: FovAlgorithm::Basic,
            torch_x: 0.0,
        }
    }

    fn display_help(&self, console: &mut Offscreen) {
        console.set_default_foreground(colors::WHITE);
        console.print(1, 0,
                      format!("IJKL : move around\nT : torch fx {}\nW : light walls {}\n+-: algo {:11?}",
                              if self.torch { "on " } else { "off" },
                              if self.light_walls { "on "  } else { "off" },
                              self.algorithm));
        console.set_default_foreground(colors::BLACK);
    }

    fn display_map(&mut self, console: &mut Offscreen, dx: f32, dy: f32, di: f32) {
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

    fn next_algorithm(&mut self) {
        match self.algorithm {
            FovAlgorithm::Restrictive => return,
            _ => self.algorithm = unsafe { std::mem::transmute(self.algorithm as i32 + 1) }
        };
    }

    fn previous_algorithm(&mut self) {
        match self.algorithm {
            FovAlgorithm::Basic => return,
            _ => self.algorithm = unsafe { std::mem::transmute(self.algorithm as i32 - 1) }
        };
    }
}

impl Render for FovSample {
    fn initialize(&mut self, console: &mut Offscreen) {
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

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        if self.recompute_fov {
            self.recompute_fov = false;
            let radius = if self.torch { TORCH_RADIUS as i32 } else { 0 };
            self.map.compute_fov(self.px, self.py, radius, self.light_walls, self.algorithm);
        }

        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut di = 0.0;
        if self.torch {
            self.torch_x += 0.2;

            let tdx = self.torch_x + 20.0;
            dx = self.noise.get(&mut [tdx]) * 1.5;
            dy = self.noise.get(&mut [tdx + 30.0]) * 1.5;
            di = self.noise.get(&mut [self.torch_x]) * 0.2;
        }

        self.display_map(console, dx, dy, di);

        if let Some((_, Event::Key(key))) = event {
            match key.printable {
                'i' | 'I' if self.map.is_walkable(self.px, self.py - 1) => {
                    self.handle_event(console, &mut |s| s.py -= 1);
                },
                'k' | 'K' if self.map.is_walkable(self.px, self.py + 1) => {
                    self.handle_event(console, &mut |s| s.py += 1);
                },
                'j' | 'J' if self.map.is_walkable(self.px - 1, self.py) => {
                    self.handle_event(console, &mut |s| s.px -= 1);
                },
                'l' | 'L' if self.map.is_walkable(self.px + 1, self.py) => {
                    self.handle_event(console, &mut |s| s.px += 1);
                },
                't' | 'T' => {
                    self.torch = !self.torch;
                    self.display_help(console);
                },
                'w' | 'W' => {
                    self.light_walls = !self.light_walls;
                    self.display_help(console);
                    self.recompute_fov = true;
                },
                '+' => {
                    self.next_algorithm();
                    self.display_help(console);
                    self.recompute_fov = true;
                },
                '-' => {
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

fn iterate_map<F>(closure: &mut F) where F: FnMut(i32, i32, char) -> () {
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
            dijkstra: Dijkstra::new_from_map(create_map(), 1.41),
            astar: AStar::new_from_map(create_map(), 1.41),
            recalculate_path: false,
            busy: 0.0,
            old_char: ' ',
        }
    }

    fn display_map(&mut self, console: &mut Offscreen) {
        iterate_map(&mut |x, y, c| {
            let wall = c == '#';
            let color = if wall { self.dark_wall } else { self.dark_ground };
            console.set_char_background(x, y, color, BackgroundFlag::Set);
        });
    }

    fn recalculate(&mut self) {
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

    fn draw_path(&mut self, console: &mut Offscreen) {
        if self.using_astar {
            for (x, y) in self.astar.iter() {
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
            for (x, y) in self.dijkstra.iter() {
                console.set_char_background(x, y, self.light_ground, BackgroundFlag::Set);
            }
        }
    }

    fn move_creature(&mut self, console: &mut Offscreen) {
        self.busy = 0.2;
        if self.using_astar {
            if !self.astar.is_empty() {
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
    fn initialize(&mut self, console: &mut Offscreen) {
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

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        if self.recalculate_path { self.recalculate() }

        self.display_map(console);
        self.draw_path(console);

        self.busy -= system::get_last_frame_length();
        if self.busy < 0.0 {
            self.move_creature(console);
        }

        if let Some((_, Event::Key(key))) = event {
            match key {
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

type CharMap = [[char; SAMPLE_SCREEN_WIDTH as usize]; SAMPLE_SCREEN_HEIGHT as usize];

struct BspSample<'a> {
    bsp: Bsp<'a>,
    generate: bool,
    refresh: bool,
    map: CharMap,
    dark_wall: colors::Color,
    dark_ground: colors::Color,
    bsp_depth: i32,
    min_room_size: i32,
    random_room: bool,
    room_walls: bool,
}

fn random_val(mut low: i32, high: i32) -> i32 {
    let mut rnd = rand::thread_rng();

    if low >= high { low -= 1 }
    rnd.gen_range(low, high)
}

impl<'a> BspSample<'a> {
    fn new() -> Self {
        BspSample {
            bsp: Bsp::new_with_size(0, 0, SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
            generate: true,
            refresh: false,
            map: [['#'; SAMPLE_SCREEN_WIDTH as usize]; SAMPLE_SCREEN_HEIGHT as usize],
            dark_wall: colors::Color::new(0, 0, 100),
            dark_ground: colors::Color::new(50, 50, 150),
            bsp_depth: 8,
            min_room_size: 4,
            random_room: false,
            room_walls: true,
        }
    }

    fn rebuild(&mut self) {
        self.bsp.resize(0, 0, SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT);
        for row in &mut self.map {
            for item in &mut row.iter_mut() {
                *item = '#';
            }
        }
        if self.generate {
            self.bsp.remove_sons();
            let rw = if self.room_walls { 1 } else { 0 };
            self.bsp.split_recursive(None,
                                     self.bsp_depth,
                                     self.min_room_size + rw,
                                     self.min_room_size + rw,
                                     1.5, 1.5);
        }
        let mut map = self.map;
        self.bsp.traverse(TraverseOrder::InvertedLevelOrder,
                          |mut node| self.visit(&mut node, &mut map));
        self.map = map;
        self.generate = false;
        self.refresh  = false;
    }

    fn visit(&self, node: &mut Bsp, map: &mut CharMap) -> bool {
        if node.is_leaf() {
            self.visit_leaf(node, map);
        } else {
            self.visit_node(node, map);
        }
        true
    }

    fn visit_leaf(&self, node: &mut Bsp, map: &mut CharMap) {
        let mut min_x = node.x + 1;
        let mut max_x = node.x + node.w - 1;
        let mut min_y = node.y + 1;
        let mut max_y = node.y + node.h - 1;

        if !self.room_walls {
            if min_x > 1 { min_x -= 1 }
            if min_y > 1 { min_y -= 1 }
        }
        if max_x == SAMPLE_SCREEN_WIDTH - 1  { max_x -= 1}
        if max_y == SAMPLE_SCREEN_HEIGHT - 1 { max_y -= 1}
        if self.random_room {
            min_x = random_val(min_x, max_x - self.min_room_size + 2);
            min_y = random_val(min_y, max_y - self.min_room_size + 2);
            max_x = random_val(min_x + self.min_room_size - 1, max_x + 1);
            max_y = random_val(min_y + self.min_room_size - 1, max_y + 1);
        }

        node.x = min_x;
        node.y = min_y;
        node.w = max_x - min_x + 1;
        node.h = max_y - min_y + 1;

        for x in min_x..(max_x+1) {
            for y in min_y..(max_y+1) {
                map[y as usize][x as usize] = ' ';
            }
        }
    }

    fn visit_node(&self, node: &mut Bsp, map: &mut CharMap) {
        let left = node.left().unwrap();
        let right = node.right().unwrap();

        node.x = min(left.x, right.x);
        node.y = min(left.y, right.y);
        node.w = max(left.x + left.w, right.x + right.w) - node.x;
        node.h = max(left.y + left.h, right.y + right.h) - node.y;

        if node.horizontal() {
            // vertical corridor
            if left.x + left.w - 1 < right.x || right.x + right.w - 1 < left.x {
                // no overlap
                let x1 = random_val(left.x, left.x + left.w);
                let x2 = random_val(right.x, right.x + right.w);
                let y  = random_val(left.y + left.h, right.y + 1);
                self.vline_up(map, x1, y - 1);
                self.hline(map, x1, y, x2);
                self.vline_down(map, x2, y + 1);
            } else {
                // straight vertical corridor
                let min_x = max(left.x, right.x);
                let max_x = min(left.x + left.w - 1, right.x + right.w - 1);
                let x = random_val(min_x, max_x + 1);
                self.vline_down(map, x, right.y);
                self.vline_up(map, x, right.y - 1);
            }
        } else {
            // horizontal corridor
            if left.y + left.h - 1 < right.y || right.y + right.h - 1 < left.y {
                let y1 = random_val(left.y, left.y + left.h);
                let y2 = random_val(right.y, right.y + right.h);
                let x  = random_val(left.x + left.w, right.x + 1);
                self.hline_left(map, x-1, y1);
                self.vline(map, x, y1, y2);
                self.hline_right(map, x+1, y2);
            } else {
                let min_y = max(left.y, right.y);
                let max_y = min(left.y + left.h - 1, right.y + right.h - 1);
                let y = random_val(min_y, max_y + 1);
                self.hline_left(map, right.x - 1, y);
                self.hline_right(map, right.x, y);
            }
        }
    }

    fn vline_up(&self, map: &mut CharMap, x: i32, mut y: i32) {
        while y >= 0 && map[y as usize][x as usize] != ' ' {
            map[y as usize][x as usize] = ' ';
            y -= 1;
        }
    }

    fn vline_down(&self, map: &mut CharMap, x: i32, mut y: i32) {
        while y < SAMPLE_SCREEN_HEIGHT && map[y as usize][x as usize] != ' ' {
            map[y as usize][x as usize] = ' ';
            y += 1;
        }
    }

    fn vline(&self, map: &mut CharMap, x: i32, y1: i32, y2: i32) {
        let mut y = y1;
        let dy = if y1 > y2 { -1 } else { 1 };
        map[y as usize][x as usize] = ' ';
        if y1 == y2 { return }
        loop {
            y += dy;
            map[y as usize][x as usize] = ' ';
            if y as i32 == y2 { break }
        }
    }

    fn hline(&self, map: &mut CharMap, x1: i32, y: i32, x2: i32) {
        let mut x = x1;
        let dx = if x1 > x2 { -1 } else { 1 };
        map[y as usize][x as usize] = ' ';
        if x1 == x2 { return };
        loop {
            x += dx;
            map[y as usize][x as usize] = ' ';
            if x as i32 == x2 { break }
        }
    }

    fn hline_left(&self, map: &mut CharMap, mut x: i32, y: i32) {
        while x >= 0 && map[y as usize][x as usize] != ' ' {
            map[y as usize][x as usize] = ' ';
            x -= 1;
        }
    }

    fn hline_right(&self, map: &mut CharMap, mut x: i32, y: i32) {
        while x < SAMPLE_SCREEN_WIDTH && map[y as usize][x as usize] != ' ' {
            map[y as usize][x as usize] = ' ';
            x += 1;
        }
    }
}

impl<'a> Render for BspSample<'a> {
    fn initialize(&mut self, _console: &mut Offscreen) {
    }

    fn render(&mut self, console: &mut Offscreen, _root: &Root,
              event: Option<(EventFlags, Event)>) {
        if self.generate || self.refresh {
            self.rebuild()
        }

        console.clear();
        console.set_default_foreground(colors::WHITE);
        console.print(1, 1, format!("ENTER : rebuild bsp\nSPACE : rebuild dungeon\n+-: bsp depth {}\n*/: room size {}\n1 : random room size {}",
                                    self.bsp_depth,
                                    self.min_room_size,
                                    if self.random_room { "ON" } else { "OFF" }));
        if self.random_room {
            console.print(1, 6, format!("2 : room walls {}",
                                        if self.room_walls { "ON" } else { "OFF" }));
        }

        for x in 0..SAMPLE_SCREEN_WIDTH {
            for y in 0..SAMPLE_SCREEN_HEIGHT {
                let wall = self.map[y as usize][x as usize] == '#';
                console.set_char_background(x, y,
                                            if wall { self.dark_wall } else {self.dark_ground },
                                            BackgroundFlag::Set);
            }
        }

        if let Some((_, Event::Key(key))) = event {
            match key {
                Key { code: KeyCode::Enter, .. } | Key { code: KeyCode::NumPadEnter, ..} =>
                    self.generate = true,
                Key { code: KeyCode::Spacebar, .. } =>
                    self.refresh = true,
                Key { printable: '+', .. } => {
                    self.bsp_depth += 1;
                    self.generate = true
                },
                Key { printable: '-', .. } if self.bsp_depth > 1 => {
                    self.bsp_depth -= 1;
                    self.generate = true
                },
                Key { printable: '*', .. } => {
                    self.min_room_size += 1;
                    self.generate = true
                },
                Key { printable: '/', .. } if self.min_room_size > 2 => {
                    self.min_room_size -= 1;
                    self.generate = true
                },
                Key { printable: '1', .. } | Key { code: KeyCode::Number1, .. } |
                Key { code: KeyCode::NumPad1, .. } => {
                    self.random_room = !self.random_room;
                    if !self.random_room { self.room_walls = true }
                    self.refresh = true
                },
                Key { printable: '2', .. } | Key { code: KeyCode::Number2, .. } |
                Key { code: KeyCode::NumPad2, .. } => {
                    self.room_walls = !self.room_walls;
                    self.refresh = true
                },
                _ => { /* ignore */ }
            }
        }
    }
}

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
    fn initialize(&mut self, _: &mut Offscreen) {
        system::set_fps(30)
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              _event: Option<(EventFlags, Event)>) {
        console.set_default_background(colors::BLACK);
        console.clear();

        let elapsed_seconds = seconds_from_duration(system::get_elapsed_time());
        let x = (SAMPLE_SCREEN_WIDTH/2) as f32 + (elapsed_seconds as f32).cos() * 10.0;
        let y = (SAMPLE_SCREEN_HEIGHT/2) as f32;
        let scale_x = 0.2 + 1.8 * (1.0 + (elapsed_seconds / 2.0).cos()) / 2.0;
        let scale_y = scale_x;
        let angle = elapsed_seconds;
        let duration = system::get_elapsed_time();
        let elapsed_milliseconds = duration.as_secs() as u32 * 1000 + duration.subsec_nanos() / 1_000_000;
        let elapsed = elapsed_milliseconds / 2000;

        if elapsed % 2 != 0 {
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
    mouse_state: Option<Mouse>,
}

impl MouseSample {
    fn new() -> Self {
        MouseSample {
            left_button: false,
            middle_button: false,
            right_button: false,
            mouse_state: None,
        }
    }

    fn format(&self, mouse: &Mouse, root: &Root) -> String {
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
}

impl Render for MouseSample {
    fn initialize(&mut self, console: &mut Offscreen) {
        system::set_fps(30);
        console.set_default_background(colors::GREY);
        console.set_default_foreground(colors::LIGHT_YELLOW);
        move_cursor(320, 200);
        show_cursor(true)
    }

    fn render(&mut self,
              console: &mut Offscreen,
              root: &Root,
              event: Option<(EventFlags, Event)>) {
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

        NameSample {
            sets: n.get_sets(),
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

    fn limit_names(&mut self) {
        while self.names.len() >= 15 {
            self.names.remove(0);
        }
    }
}

impl Render for NameSample {
    fn initialize(&mut self, _: &mut Offscreen) {
        system::set_fps(30);
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        self.limit_names();
        self.display_names(console);

        self.delay += system::get_last_frame_length();
        if self.delay >= 0.5 {
            let name = &self.sets[self.cur_set];
            self.delay -= 0.5;
            self.names.push(self.name_gen.generate(name).unwrap())
        }

        if let Some((_, Event::Key(key))) = event {
            match key.printable {
                '+' => {
                    self.cur_set = (self.cur_set + 1) % self.sets.len();
                    self.names.push("======".to_owned());
                },
                '-' => {
                    if self.cur_set == 0 {
                        self.cur_set = self.sets.len() - 1
                    } else {
                        self.cur_set -= 1;
                    }
                    self.names.push("======".to_owned());
                },
                _ => {},
            }
        }
    }
}

struct MenuItem<'a> {
    name: String,
    render: &'a mut Render
}

impl<'a> MenuItem<'a> {
    fn new(name: &str, render: &'a mut Render) -> Self {
        MenuItem { name: name.to_owned(), render: render }
    }
}

impl<'a> Render for MenuItem<'a> {
    fn initialize(&mut self, console: &mut Offscreen) {
        self.render.initialize(console);
    }

    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              event: Option<(EventFlags, Event)>) {
        self.render.render(console, _root, event);
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
        Options {
            fullscreen_width: 0,
            fullscreen_height: 0,
            font: "consolas10x10_gs_tc.png".to_owned(),
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
    let mut line = LineSample::new();
    let mut noise = NoiseSample::new();
    let mut bsp = BspSample::new();
    let mut samples = vec![MenuItem::new("  True colors      ", &mut colors),
                           MenuItem::new("  Offscreen console", &mut offscreen),
                           MenuItem::new("  Line drawing     ", &mut line),
                           MenuItem::new("  Noise            ", &mut noise),
                           MenuItem::new("  Field of view    ", &mut fov),
                           MenuItem::new("  Path finding     ", &mut path_sample),
                           MenuItem::new("  Bsp toolkit      ", &mut bsp),
                           MenuItem::new("  Image toolkit    ", &mut image_sample),
                           MenuItem::new("  Mouse support    ", &mut mouse),
                           MenuItem::new("  Name generator   ", &mut names),
                           // MenuItem::new("  SDL callback     ", &mut ),
                           ];
    let mut cur_sample = 0;
    let mut options = Options::new();
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
        samples[cur_sample].render(&mut console, &root, event);

        blit(&console, (0, 0), (SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT),
             &mut root, (SAMPLE_SCREEN_X, SAMPLE_SCREEN_Y), 1.0, 1.0);

        // erase the renderer in debug mode (needed because the root
        // console is not cleared each frame)
        root.print(1, 1, "        ");
        root.flush();

        if let Some((_, Event::Key(key))) = event {
            match key.code {
                KeyCode::Down => {
                    cur_sample = (cur_sample + 1) % samples.len();
                    samples[cur_sample].initialize(&mut console);
                },
                KeyCode::Up => {
                    if cur_sample == 0 {
                        cur_sample = samples.len() - 1;
                    } else {
                        cur_sample -= 1;
                    }
                    samples[cur_sample].initialize(&mut console);
                },
                KeyCode::Enter if key.left_alt => {
                    let fullscreen = root.is_fullscreen();
                    root.set_fullscreen(!fullscreen);
                },
                KeyCode::PrintScreen => {
                    // TODO
                },
                KeyCode::Escape => break,
                _ => {},
            }
        }
    }
}

fn print_samples(root: &mut Root, cur_sample: usize, samples: &[MenuItem]) {
    for (i, sample) in samples.iter().enumerate() {
        if i == cur_sample {
            root.set_default_foreground(colors::WHITE);
            root.set_default_background(colors::LIGHT_BLUE);
        } else {
            root.set_default_foreground(colors::GREY);
            root.set_default_background(colors::BLACK);
        }
        let y = 46 - (samples.len() - i);
        let fun = &sample.name;
        root.print_ex(2, y as i32, BackgroundFlag::Set, TextAlignment::Left, fun);
    }

}

fn print_help_message(root: &mut Root) {
    root.set_default_foreground(colors::GREY);
    root.print_ex(79, 46, BackgroundFlag::None, TextAlignment::Right,
                  format!("last frame : {:3.0} ms ({:3} fps)",
                          system::get_last_frame_length() * 1000.0,
                          system::get_fps()));

    let time = system::get_elapsed_time();
    let millis = time.as_secs() as u32 * 1000 + time.subsec_nanos() / 1_000_000;
    root.print_ex(79, 47, BackgroundFlag::None, TextAlignment::Right,
                  format!("elapsed {:8}ms {:4.2}s",
                          millis,
                          millis as f32/ 1000.0));

    root.print(2, 47, format!("{}{} : select a sample",
                              chars::ARROW_N, chars::ARROW_S));

    let fullscreen_text = if root.is_fullscreen() {
        "windowed mode"
    } else {
        "fullscren_mode"
    };

    root.print(2, 48, format!("ALT-ENTER : switch to {}", fullscreen_text));
}

fn parse_args(options: &mut Options) {
    let mut args = std::env::args();

    while let Some(opt) = args.next() {
        match opt.as_ref() {
            "-font" => {
                if let Some(font) = args.next() {
                    options.font = font;
                }
            },
            "-font-nb-char" => {
                if let (Some(h), Some(v)) = (args.next(), args.next()) {
                    options.nb_char_horiz = h.parse().ok().unwrap();
                    options.nb_char_vertic = v.parse().ok().unwrap();
                }
            },
            "-fullscreen-resolution" => {
                if let (Some(w), Some(h)) = (args.next(), args.next()) {
                    options.fullscreen_width = w.parse().ok().unwrap();
                    options.fullscreen_height = h.parse().ok().unwrap();
                }
            },
            "-fullscreen" => options.fullscreen = true,
            "-font-in-row" => options.font_layout = FontLayout::AsciiInRow,
            "-font-greyscale" => options.font_type = FontType::Greyscale,
            "-font-tcod" => options.font_layout = FontLayout::Tcod,
            "-renderer" => {
                if let Some(renderer) = args.next() {
                    match renderer.parse::<i32>().ok() {
                        Some(0) => options.renderer = Renderer::GLSL,
                        Some(1) => options.renderer = Renderer::OpenGL,
                        Some(2) => options.renderer = Renderer::SDL,
                        _ => {
                            println!("Invalid renderer");
                            std::process::exit(1)
                        }
                    }
                }
            },
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
            },
            _ => {},
        }
    }
}
