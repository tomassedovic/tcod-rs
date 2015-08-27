use bindings::ffi;
use bindings::{AsNative, c_bool, c_float, c_int, c_void};
use map::Map;

enum PathInnerData<'a> {
    Map(Map),
    Callback(Box<FnMut((i32, i32), (i32, i32)) -> f32+'a>),
}

pub struct AStar<'a>{
    tcod_path: ffi::TCOD_path_t,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: i32,
    height: i32,
}

impl<'a> AsNative<ffi::TCOD_path_t> for AStar<'a> {
    unsafe fn as_native(&self) -> &ffi::TCOD_path_t {
        &self.tcod_path
    }
}

impl<'a> Drop for AStar<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_path_delete(self.tcod_path);
        }
    }
}

extern "C" fn c_path_callback<T: FnMut((i32, i32), (i32, i32)) -> f32>(xf: c_int, yf: c_int,
                          xt: c_int, yt: c_int,
                          user_data: *mut c_void) -> c_float {
    let callback_ptr = user_data as *mut T;
    let cb: &mut T = unsafe {
        &mut *callback_ptr
    };
    cb((xf, yf), (xt, yt))
}

impl<'a> AStar<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32, path_callback: T,
        diagonal_cost: f32) -> AStar<'a> {
        let callback = Box::new(path_callback);
        let user_data = &*callback as *const T as *mut c_void;
        unsafe {
            let tcod_path = ffi::TCOD_path_new_using_function(width, height,
                                                              Some(c_path_callback::<T>),
                                                              user_data,
                                                              diagonal_cost);
            AStar {
                tcod_path: tcod_path,
                // We need to keep user_closure around, otherwise it
                // would get deallocated at the end of this function.
                inner: PathInnerData::Callback(callback),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> AStar<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_path_new_using_map(*map.as_native(), diagonal_cost)
        };
        let (w, h) = map.size();
        AStar {
            tcod_path: tcod_path,
            inner: PathInnerData::Map(map),
            width: w,
            height: h,
        }
    }

    pub fn find(&mut self,
                from: (i32, i32),
                to: (i32, i32)) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        assert!(from_x >= 0 && from_y >= 0 && to_x >= 0 && to_y >= 0);
        assert!(from_x < self.width && from_y < self.height && to_x < self.width && to_y < self.height);
        unsafe {
            ffi::TCOD_path_compute(self.tcod_path,
                                   from_x, from_y,
                                   to_x, to_y) != 0
        }
    }

    pub fn iter(&'a self) -> AStarPathIter<'a> {
        AStarPathIter { current: -1, path: self }
    }

    pub fn walk(&mut self) -> AStarIterator {
        AStarIterator{tcod_path: self.tcod_path, recalculate: false}
    }

    pub fn walk_recalculate(&mut self) -> AStarIterator {
        AStarIterator{tcod_path: self.tcod_path, recalculate: true}
    }

    pub fn walk_one_step(&mut self, recalculate_when_needed: bool) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_path_reverse(self.tcod_path)
        }
    }

    pub fn origin(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_origin(self.tcod_path, &mut x, &mut y);
            (x as isize, y as isize)
        }
    }

    pub fn destination(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_destination(self.tcod_path, &mut x, &mut y);
            (x as isize, y as isize)
        }
    }

    pub fn get(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get(self.tcod_path, index, &mut x, &mut y);
            (Some((x, y)))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path)
        }
    }
}

pub struct Dijkstra<'a> {
    tcod_path: ffi::TCOD_dijkstra_t,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: i32,
    height: i32,
}

impl<'a> AsNative<ffi::TCOD_path_t> for Dijkstra<'a> {
    unsafe fn as_native(&self) -> &ffi::TCOD_dijkstra_t {
        &self.tcod_path
    }
}

impl<'a> Drop for Dijkstra<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_delete(self.tcod_path);
        }
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32,
        path_callback: T,
        diagonal_cost: f32) -> Dijkstra<'a> {
        let callback = Box::new(path_callback);
        let user_data = &*callback as *const T as *mut c_void;
        unsafe {
            let tcod_path = ffi::TCOD_dijkstra_new_using_function(width,
                                                                  height,
                                                                  Some(c_path_callback::<T>),
                                                                  user_data,
                                                                  diagonal_cost);
            Dijkstra {
                tcod_path: tcod_path,
                inner: PathInnerData::Callback(callback),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> Dijkstra<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_dijkstra_new(*map.as_native(), diagonal_cost)
        };
        let (w, h) = map.size();
        Dijkstra {
            tcod_path: tcod_path,
            inner: PathInnerData::Map(map),
            width: w,
            height: h,
        }
    }

    pub fn compute_grid(&mut self, root: (i32, i32)) {
        let (x, y) = root;
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_dijkstra_compute(self.tcod_path, x, y);
        }
    }

    pub fn find(&mut self, destination: (i32, i32)) -> bool {
        let (x, y) = destination;
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            unsafe {
                ffi::TCOD_dijkstra_path_set(self.tcod_path, x, y) != 0
            }
        } else {
            false
        }
    }

    pub fn iter(&'a self) -> DijkstraPathIter<'a> {
        DijkstraPathIter { current: -1, path: self }
    }

    pub fn walk(&mut self) -> DijkstraIterator {
        DijkstraIterator{tcod_path: self.tcod_path}
    }

    pub fn walk_one_step(&mut self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path, &mut x, &mut y) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }


    pub fn distance_from_root(&self, point: (i32, i32)) -> Option<f32> {
        let (x, y) = point;
        let result = unsafe {
            ffi::TCOD_dijkstra_get_distance(self.tcod_path, x, y)
        };
        if result == -1.0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_reverse(self.tcod_path);
        }
    }

    pub fn get(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_dijkstra_get(self.tcod_path, index, &mut x, &mut y);
            Some((x, y))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_dijkstra_is_empty(self.tcod_path) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_dijkstra_size(self.tcod_path)
        }
    }
}

pub struct AStarIterator {
    tcod_path: ffi::TCOD_path_t,
    recalculate: bool,
}

impl Iterator for AStarIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      self.recalculate as c_bool) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }
}

pub struct DijkstraIterator {
    tcod_path: ffi::TCOD_path_t,
}

impl Iterator for DijkstraIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path, &mut x, &mut y) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }
}

pub struct AStarPathIter<'a> {
    current: i32,
    path: &'a AStar<'a>,
}

impl<'a> Iterator for AStarPathIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.path.len() - 1 {
            None
        } else {
            self.current += 1;
            unsafe {
                let mut x: c_int = 0;
                let mut y: c_int = 0;
                ffi::TCOD_path_get(*self.path.as_native(), self.current, &mut x, &mut y);
                Some((x, y))
            }
        }
    }
}

pub struct DijkstraPathIter<'a> {
    current: i32,
    path: &'a Dijkstra<'a>,
}

impl<'a> Iterator for DijkstraPathIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.path.len() - 1 {
            None
        } else {
            self.current += 1;
            unsafe {
                let mut x: c_int = 0;
                let mut y: c_int = 0;
                ffi::TCOD_dijkstra_get(*self.path.as_native(), self.current, &mut x, &mut y);
                Some((x, y))
            }
        }
    }
}
