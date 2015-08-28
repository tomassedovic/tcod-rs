extern crate libc;

use bindings::ffi;
use bindings::c_int;

#[derive(Default)]
pub struct Line {
    tcod_line: ffi::TCOD_bresenham_data_t,
}

pub trait Listener {
    fn put_point(x: i32, y: i32) -> bool;
}

impl Line {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let mut line: Line = Default::default();
        unsafe {
            ffi::TCOD_line_init_mt(start.0, start.1, end.0, end.1, &mut line.tcod_line)
        };
        line
    }
    
    // pub fn new_from_listener(start: (i32, i32), end: (i32, i32), listener: &Listener) -> Self {}
    // pub fn step(&self) -> (i32, i32) {}
}

#[cfg(test)]
mod test {
    use super::Line;

    #[test]
    fn line_created() {
        let line = Line::new((1, 1), (5, 5));

        assert_eq!(line.tcod_line.origx, 1);
        assert_eq!(line.tcod_line.origy, 1);
        assert_eq!(line.tcod_line.destx, 5);
        assert_eq!(line.tcod_line.desty, 5);
    }

    #[test]
    fn start_end_same() {
        let line = Line::new((1, 1), (1, 1));

        assert_eq!(line.tcod_line.origx, 1);
        assert_eq!(line.tcod_line.origy, 1);
        assert_eq!(line.tcod_line.destx, 1);
        assert_eq!(line.tcod_line.desty, 1);
    }

}
