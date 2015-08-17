extern crate tcod;
use tcod::console::{Root, Console, BackgroundFlag, Offscreen, blit};
use tcod::console::{TextAlignment, Renderer};
use tcod::input::{Key, KeyCode, KEY_RELEASED};
use tcod::system::set_fps;
use tcod::colors;

fn render_colors(first: bool) -> () {}
fn render_offscreen(first:bool) -> () {}
fn render_lines(first: bool) -> () {}
fn render_noise(first: bool) -> () {}
fn render_fov(first: bool) -> () {}
fn render_path(first: bool) -> () {}
fn render_bsp(first: bool) -> () {}
fn render_image(first: bool) -> () {}
fn render_mouse(first: bool) -> () {}
fn render_name(first: bool) -> () {}
fn render_sdl(first: bool) -> () {}

fn main() {
    let samples : Vec<(String, fn(bool) -> ())> = vec![
        ("  True colors      ".to_string(), render_colors),
        ("  Offscreen console".to_string(), render_offscreen),
        ("  Line drawing     ".to_string(), render_lines),
        ("  Noise            ".to_string(), render_noise),
        ("  Field of view    ".to_string(), render_fov),
        ("  Path finding     ".to_string(), render_path),
        ("  Bsp toolkit      ".to_string(), render_bsp),
        ("  Image toolkit    ".to_string(), render_image),
        ("  Mouse support    ".to_string(), render_mouse),
        ("  Name generator   ".to_string(), render_name),
        ("  SDL callback     ".to_string(), render_sdl)
    ];
    let mut cur_sample = 0;
    
    let renderer = Renderer::SDL;
    let mut root = Root::initializer()
        .size(80, 50)
        .title("libtcod Rust sample")
        .fullscreen(false)
        .renderer(renderer)
        .init();
    while !root.window_closed() {
        // print the list of samples
        for i in 0..samples.len() {
            if i == cur_sample {
                root.set_default_foreground(colors::WHITE);
                root.set_default_background(colors::LIGHT_BLUE);
            } else {
                root.set_default_foreground(colors::GREY);
                root.set_default_background(colors::BLACK);
            }
            let y : i32 = 46 - (samples.len() as i32 - i as i32);
            let fun = &samples[i].0;
            root.print_ex(2, y, BackgroundFlag::Set, TextAlignment::Left, fun);
        }
        
        root.flush();
        root.check_for_keypress(KEY_RELEASED);
    }
}
