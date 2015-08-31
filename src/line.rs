extern crate libc;

use bindings::ffi;
use bindings::c_int;

#[derive(Default)]
pub struct Line {
    tcod_line: ffi::TCOD_bresenham_data_t,
}

pub trait Listener {
    fn put_point(&self, x: i32, y: i32) -> bool;
}

impl Line {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let mut line: Line = Default::default();
        unsafe {
            ffi::TCOD_line_init_mt(start.0, start.1, end.0, end.1, &mut line.tcod_line)
        };
        line
    }
    
    pub fn new_with_callback<F>(start: (i32, i32), end: (i32, i32), callback: F) -> Self
        where F: FnMut(i32, i32) -> bool
    {
        let mut line: Line = Line::new(start, end);
        line.step_with_callback(callback);
        line
    }

    pub fn new_with_listener(start: (i32, i32), end: (i32, i32), listener: &Listener) -> Self {
        let mut line: Line = Line::new(start, end);
        line.step_with_listener(listener);
        line
    }
    pub fn step(&mut self) -> Option<(i32, i32)> {
        let mut x: c_int = 0;
        let mut y: c_int = 0;
        let end = unsafe {
            ffi::TCOD_line_step_mt(&mut x, &mut y, &mut self.tcod_line)
        };

        if end == 0 {
            Some((x, y))
        } else {
            None
        }
    }

    fn step_with_callback<F>(&mut self, mut callback: F) -> bool
        where F: FnMut(i32, i32) -> bool
    {
        let mut x: c_int = self.tcod_line.origx;
        let mut y: c_int = self.tcod_line.origy;
        loop {
		    if !callback(x, y) {
                return false
            }
            let step = unsafe {
                ffi::TCOD_line_step_mt(&mut x, &mut y, &mut self.tcod_line)
            };
            if step != 0 {
                break
            }
	    }
	    true
    }

    fn step_with_listener(&mut self, listener: &Listener) -> bool {
        let mut x: c_int = self.tcod_line.origx;
        let mut y: c_int = self.tcod_line.origy;
        loop {
		    if !listener.put_point(x, y) {
                return false
            }
            let step = unsafe {
                ffi::TCOD_line_step_mt(&mut x, &mut y, &mut self.tcod_line)
            };
            if step != 0 {
                break
            }
	    }
	    true
    }
}

impl Iterator for Line {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

#[cfg(test)]
mod test {
    use super::Line;
    use super::Listener;

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

    #[test]
    fn step_line() {
        let mut line = Line::new((1, 1), (5, 5));

        assert_eq!(Some((2, 2)), line.step());
        assert_eq!(Some((3, 3)), line.step());
        assert_eq!(Some((4, 4)), line.step());
        assert_eq!(Some((5, 5)), line.step());
        assert_eq!(None, line.step());
    }

    #[test]
    fn step_two_lines() {
        let mut line1 = Line::new((1, 1), (5, 5));
        let mut line2 = Line::new((10, 10), (14, 14));

        assert_eq!(Some((2, 2)), line1.step());
        assert_eq!(Some((11, 11)), line2.step());
        assert_eq!(Some((3, 3)), line1.step());
        assert_eq!(Some((12, 12)), line2.step());
        assert_eq!(Some((4, 4)), line1.step());
        assert_eq!(Some((13, 13)), line2.step());
        assert_eq!(Some((5, 5)), line1.step());
        assert_eq!(Some((14, 14)), line2.step());
        assert_eq!(None, line1.step());
        assert_eq!(None, line2.step());
    }

    #[test]
    fn iterate_over_line() {
        let mut line = Line::new((1, 1), (5, 5));

        assert_eq!(Some((2, 2)), line.next());
        assert_eq!(Some((3, 3)), line.next());
        assert_eq!(Some((4, 4)), line.next());
        assert_eq!(Some((5, 5)), line.next());
        assert_eq!(None, line.next());
    }

    #[test]
    fn line_with_callback() {
        let mut line = Line::new_with_callback((1, 1), (5, 5), |x, _y| {
            assert!(x <= 4);
            x < 4
        });
        assert_eq!(Some((5, 5)), line.next());
        assert_eq!(None, line.next());
    }

    struct MyListener;
    impl Listener for MyListener {
        fn put_point(&self, x: i32, _y:i32) -> bool {
            assert!(x <= 4);
            x < 4
        }
    }

    #[test]
    fn line_with_listener() {
        let listener = MyListener;
        let mut line = Line::new_with_listener((1, 1), (5, 5), &listener);
        assert_eq!(Some((5, 5)), line.next());
        assert_eq!(None, line.next());
    }
}
