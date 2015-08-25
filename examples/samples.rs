extern crate tcod;
extern crate tcod_sys as ffi;
extern crate rand;

use tcod::console::*;
use tcod::input::*;
use tcod::system;
use tcod::colors;
use tcod::colors::Color;
use tcod::chars;
use rand::Rng;
use rand::ThreadRng;
use std::char::from_u32;

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
    cols : [Color; 4],
    dirr : [i8; 4],
    dirg : [i8; 4],
    dirb : [i8; 4],
}

impl ColorsSample {
    fn new() -> ColorsSample {
        ColorsSample {
            cols : [Color {r:50,  g:40, b:150},
                    Color {r:240, g:85, b:5},
                    Color {r:50,  g:35, b:240},
                    Color {r:10,  g:200, b:130}],
            dirr : [1, -1, 1, 1],
            dirg : [1, -1, -1, 1],
            dirb : [1, 1, 1, -1]
        }
    }
}

impl Render for ColorsSample {
    fn render(&mut self,
              console: &mut Offscreen,
              _root: &Root,
              first: bool,
              _event: Option<(EventFlags, Event)>) -> () {
        enum Dir {
            TopLeft = 0,
            TopRight,
            BottomLeft,
            BottomRight,
        };

        let rng : &mut ThreadRng = &mut rand::thread_rng();

        if first {
            system::set_fps(0);
            console.clear()
        }

        for c in 0..4 {
            let component = rng.gen_range(0, 3);
            match component {
                0 => {
                    let delta : i16 = (5 * self.dirr[c]) as i16;
                    self.cols[c].r = (self.cols[c].r as i16 + delta) as u8;

                    if self.cols[c].r == 255 {
                        self.dirr[c] = -1
                    } else if self.cols[c].r == 0 {
                        self.dirr[c] = 1
                    }
                },
                1 => {
                    let delta : i16 = (5 * self.dirg[c]) as i16;
                    self.cols[c].g = (self.cols[c].g as i16 + delta) as u8;

                    if self.cols[c].g == 255 {
                        self.dirg[c] = -1
                    } else if self.cols[c].g == 0 {
                        self.dirg[c] = 1
                    }
                },
                2 => {
                    let delta : i16 = (5 * self.dirb[c]) as i16;
                    self.cols[c].b = (self.cols[c].b as i16 + delta) as u8;

                    if self.cols[c].b == 255 {
                        self.dirb[c] = -1
                    } else if self.cols[c].b == 0 {
                        self.dirb[c] = 1
                    }
                },
                _ => panic!("Random number generator is broken!")
            }
        }

        // ==== scan the whole screen, interpolating corner colors ====
	    for x in 0..SAMPLE_SCREEN_WIDTH {
		    let xcoef = (x as f32) / ((SAMPLE_SCREEN_WIDTH-1) as f32);
            
		    // get the current column top and bottom colors
		    let top = colors::lerp(self.cols[Dir::TopLeft as usize], self.cols[Dir::TopRight as usize], xcoef);
		    let bottom = colors::lerp(self.cols[Dir::BottomLeft as usize], self.cols[Dir::BottomRight as usize], xcoef);
		    for y in 0..SAMPLE_SCREEN_HEIGHT {
			    let ycoef = (y as f32) / ((SAMPLE_SCREEN_HEIGHT-1) as f32);
                
			    // get the current cell color
			    let cur_color = colors::lerp(top, bottom, ycoef);
			    console.set_char_background(x, y, cur_color, BackgroundFlag::Set);
		    }
	    }

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
                    let r = rng.gen_range('a' as u32, 'z' as u32);
				    c = from_u32(r).unwrap();
			    }

			    console.set_default_foreground(col);
			    console.put_char(x, y, c, BackgroundFlag::None);
		    }
	    }

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
fn render_fov(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
fn render_path(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
fn render_bsp(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
fn render_image(_console: &mut Offscreen, _root: &Root, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
 */

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
}

impl Render for MouseSample {
    fn render(&mut self,
              console: &mut Offscreen,
              root: &Root,
              first: bool,
              event: Option<(EventFlags, Event)>) -> () {
        if first {
            system::set_fps(30);
            console.set_default_background(colors::GREY);
            console.set_default_foreground(colors::LIGHT_YELLOW);
            move_cursor(320, 200);
            show_cursor(true)
        }

        console.clear();
        match event {
            Some((_, Event::Mouse(mouse))) => {
                if mouse.lbutton_pressed {self.left_button = !self.left_button;}
                if mouse.mbutton_pressed {self.middle_button = !self.middle_button;}
                if mouse.rbutton_pressed {self.right_button = !self.right_button;}

                self.mouse_state = Some(mouse);
            },
            Some((_, Event::Key(state))) if state.key == Key::Special(KeyCode::Number1) => {
                show_cursor(false);
            },
            Some((_, Event::Key(state))) if state.key == Key::Special(KeyCode::Number2) => {
                show_cursor(true)
            },
            _ => {} // Ignore other events
        }

        if self.mouse_state.is_some() {
            let mouse = self.mouse_state.unwrap();
            console.print(1, 1,
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
                                  if mouse.wheel_up { "UP" } else if mouse.wheel_down { "DOWN" } else { "" }
                                  ));

        }

        console.print(1, 10, "1 : Hide cursor\n2 : Show cursor");
    }
}

/*
fn render_name(_console: &mut Offscreen, _first: bool, _event: Option<(EventFlags, Event)>) -> () {}
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

static RENDERER_NAME : [&'static str; 3] = ["F1 GLSL   ", "F2 OPENGL ", "F3 SDL    "];

struct Options {
    fullscreen_width: i32,
    fullscreen_height: i32,
    font: String,
    font_type: FontType,
    font_layout: FontLayout,
    nb_char_horiz: i32,
    nb_char_vertic: i32,
    fullscreen: bool,
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
        }
    }
}

fn main() {
    let mut colors = ColorsSample::new();
    let mut offscreen = OffscreenSample::new();
    let mut mouse = MouseSample::new();
    let mut samples = vec![MenuItem::new("  True colors      ", &mut colors),
                           MenuItem::new("  Offscreen console", &mut offscreen),
                           MenuItem::new("  Mouse support    ", &mut mouse),
                           ];
    // let samples = vec!["  True colors      ".to_string(),
    //     "  Offscreen console".to_string(),
    //     "  Line drawing     ".to_string(),
    //     "  Noise            ".to_string(),
    //     "  Field of view    ".to_string(),
    //     "  Path finding     ".to_string(),
    //     "  Bsp toolkit      ".to_string(),
    //     "  Image toolkit    ".to_string(),
    //     "  Mouse support    ".to_string(),
    //     "  Name generator   ".to_string(),
    //     "  SDL callback     ".to_string()];
    let mut cur_sample = 0;
    let mut options = Options::new();
    let mut first = true;
    let mut console = Offscreen::new(SAMPLE_SCREEN_WIDTH, SAMPLE_SCREEN_HEIGHT);
    let renderer = Renderer::SDL;

    parse_args(&mut options);

    if options.fullscreen_width > 0 {
		system::force_fullscreen_resolution(options.fullscreen_width,
                                            options.fullscreen_height);
    }
    let mut root = Root::initializer()
        .size(80, 50)
        .title("libtcod Rust sample")
        .fullscreen(options.fullscreen)
        .renderer(renderer)
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

        print_renderers(&mut root);
        
        root.flush();
        match event {
            None => {continue;}
            Some((_flag, Event::Key(state))) => {
                match state.key {
                    Key::Special(KeyCode::Down) => {
                        cur_sample = (cur_sample + 1) % samples.len();
                        first = true
                    }
                    Key::Special(KeyCode::Up) => {
                        if cur_sample == 0 { cur_sample = samples.len() - 1; }
                        else { cur_sample -= 1; }
                        first = true
                    }
                    Key::Special(KeyCode::Enter) if state.left_alt => {
                        let fullscreen = root.is_fullscreen();
                        root.set_fullscreen(!fullscreen)
                    }
                    Key::Special(KeyCode::PrintScreen) => {
                        // TODO
                    }
                    Key::Special(KeyCode::Escape) => { break }
                    Key::Special(KeyCode::F1) => {
                        system::set_renderer(Renderer::GLSL)
                    }
                    Key::Special(KeyCode::F2) => {
                        system::set_renderer(Renderer::OpenGL)
                    }
                    Key::Special(KeyCode::F3) => {
                        system::set_renderer(Renderer::SDL)
                    }
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

fn print_renderers(root: &mut Root) -> () {
    root.set_default_foreground(colors::GREY);
    root.set_default_background(colors::BLACK);
    root.print_ex(42, 46-(ffi::TCOD_NB_RENDERERS as i32 + 1),
                  BackgroundFlag::Set, TextAlignment::Left,
                  "Renderer :");
    for i in 0..(ffi::TCOD_NB_RENDERERS as i32) {
        if i == system::get_renderer() as i32{
            root.set_default_foreground(colors::WHITE);
            root.set_default_background(colors::LIGHT_BLUE);
        } else {
            root.set_default_foreground(colors::GREY);
            root.set_default_background(colors::BLACK);
        }
        root.print_ex(42, 46 - (ffi::TCOD_NB_RENDERERS as i32 - i),
                      BackgroundFlag::Set, TextAlignment::Left,
                      RENDERER_NAME[i as usize]);
    }
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
