use bindings::ffi;
use bindings::{c_bool, c_float, c_int, c_void};

use map::{Map, TcodMap};

enum PathInnerData<'a> {
    Map(Map),
    Callback(Box<FnMut((i32, i32), (i32, i32)) -> f32+'a>, Box<(usize, usize)>),
}

// We need to wrap the pointer in a struct so that we can implement a
// destructor. But we can't put ffi::TCOD_path_t directly inside AStarPath
// because it includes the boxed closure which has a 'static lifetime so we
// can't attach a destructor to it.
//
// Unless we use #[unsafe_destructor] and I've no idea what could go wrong there.
struct TCODPath {
    ptr: ffi::TCOD_path_t,
}

impl Drop for TCODPath {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_path_delete(self.ptr);
        }
    }
}

pub struct AStarPath<'a>{
    tcod_path: TCODPath,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: i32,
    height: i32,
}

extern "C" fn c_path_callback(xf: c_int, yf: c_int,
                          xt: c_int, yt: c_int,
                          user_data: *mut c_void) -> c_float {
    unsafe {
        let ptr: &(usize, usize) = &*(user_data as *const (usize, usize));
        let cb: &mut FnMut((i32, i32), (i32, i32)) -> f32 = ::std::mem::transmute(*ptr);
        cb((xf, yf), (xt, yt))
    }
}

type TcodPathCb = extern "C" fn(c_int, c_int, c_int, c_int, *mut c_void) -> c_float;

impl<'a> AStarPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32, path_callback: T,
        diagonal_cost: f32) -> AStarPath<'a> {
        // Convert the closure to a trait object. This will turn it into a fat pointer:
        let user_closure: Box<FnMut((i32, i32), (i32, i32)) -> f32> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            // Allocate the fat pointer on the heap:
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            // Create a pointer to the fat pointer. This well be passed as *void user_data:
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;

            let tcod_path = ffi::TCOD_path_new_using_function(width, height,
                                                              Some(c_path_callback),
                                                              user_data_ptr as *mut c_void,
                                                              diagonal_cost);
            AStarPath {
                tcod_path: TCODPath{ptr: tcod_path},
                // Keep track of everything we've allocated on the heap. Both
                // `user_closure` and `ptr` will be deallocated when AStarPath
                // is dropped:
                inner: PathInnerData::Callback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> AStarPath<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_path_new_using_map(*map.as_tcod_map(), diagonal_cost)
        };
        let (w, h) = map.size();
        AStarPath {
            tcod_path: TCODPath{ptr: tcod_path},
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
            ffi::TCOD_path_compute(self.tcod_path.ptr,
                                   from_x, from_y,
                                   to_x, to_y) != 0
        }
    }

    pub fn walk(&mut self) -> AStarPathIterator {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: false}
    }

    pub fn walk_recalculate(&mut self) -> AStarPathIterator {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: true}
    }

    pub fn walk_one_step(&mut self, recalculate_when_needed: bool) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path.ptr, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_path_reverse(self.tcod_path.ptr)
        }
    }

    pub fn origin(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_origin(self.tcod_path.ptr, &mut x, &mut y);
            (x as isize, y as isize)
        }
    }

    pub fn destination(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_destination(self.tcod_path.ptr, &mut x, &mut y);
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
            ffi::TCOD_path_get(self.tcod_path.ptr, index, &mut x, &mut y);
            (Some((x, y)))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path.ptr)
        }
    }
}

struct TCODDijkstraPath {
    ptr: ffi::TCOD_dijkstra_t,
}

impl Drop for TCODDijkstraPath {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_delete(self.ptr);
        }
    }
}

pub struct DijkstraPath<'a> {
    tcod_path: TCODDijkstraPath,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: i32,
    height: i32,
}

impl<'a> DijkstraPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32,
        path_callback: T,
        diagonal_cost: f32) -> DijkstraPath<'a> {
        // NOTE: this is might be a bit confusing. See the
        // AStarPath::new_from_callback implementation comments.
        let user_closure: Box<FnMut((i32, i32), (i32, i32)) -> f32> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;
            let tcod_path = ffi::TCOD_dijkstra_new_using_function(width,
                                                                  height,
                                                                  Some(c_path_callback),
                                                                  user_data_ptr as *mut c_void,
                                                                  diagonal_cost);
            DijkstraPath {
                tcod_path: TCODDijkstraPath{ptr: tcod_path},
                inner: PathInnerData::Callback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> DijkstraPath<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_dijkstra_new(*map.as_tcod_map(), diagonal_cost)
        };
        let (w, h) = map.size();
        DijkstraPath {
            tcod_path: TCODDijkstraPath{ptr: tcod_path},
            inner: PathInnerData::Map(map),
            width: w,
            height: h,
        }
    }

    pub fn compute_grid(&mut self, root: (i32, i32)) {
        let (x, y) = root;
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_dijkstra_compute(self.tcod_path.ptr, x, y);
        }
    }

    pub fn find(&mut self, destination: (i32, i32)) -> bool {
        let (x, y) = destination;
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            unsafe {
                ffi::TCOD_dijkstra_path_set(self.tcod_path.ptr, x, y) != 0
            }
        } else {
            false
        }
    }

    pub fn walk(&mut self) -> DijkstraPathIterator {
        DijkstraPathIterator{tcod_path: self.tcod_path.ptr}
    }

    pub fn walk_one_step(&mut self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path.ptr, &mut x, &mut y) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }


    pub fn distance_from_root(&self, point: (i32, i32)) -> Option<f32> {
        let (x, y) = point;
        let result = unsafe {
            ffi::TCOD_dijkstra_get_distance(self.tcod_path.ptr, x, y)
        };
        if result == -1.0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_reverse(self.tcod_path.ptr);
        }
    }

    pub fn get(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_dijkstra_get(self.tcod_path.ptr, index, &mut x, &mut y);
            Some((x, y))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_dijkstra_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_dijkstra_size(self.tcod_path.ptr)
        }
    }
}

pub struct AStarPathIterator {
    tcod_path: ffi::TCOD_path_t,
    recalculate: bool,
}

impl Iterator for AStarPathIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      self.recalculate as c_bool) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }
}

pub struct DijkstraPathIterator {
    tcod_path: ffi::TCOD_path_t,
}

impl Iterator for DijkstraPathIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path, &mut x, &mut y) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }
}
