use bindings::ffi;
use bindings::{AsNative, c_bool};

pub struct Map {
    tcod_map: ffi::TCOD_map_t,
}

impl AsNative<ffi::TCOD_map_t> for Map {
    unsafe fn as_native(&self) -> &ffi::TCOD_map_t {
        &self.tcod_map
    }
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        assert!(width > 0 && height > 0);
        unsafe {
            Map{tcod_map: ffi::TCOD_map_new(width, height)}
        }
    }

    pub fn size(&self) -> (i32, i32) {
        unsafe {
            (ffi::TCOD_map_get_width(self.tcod_map),
             ffi::TCOD_map_get_height(self.tcod_map))
        }
    }

    pub fn set(&mut self, x: i32, y: i32, transparent: bool, walkable: bool) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_set_properties(self.tcod_map, x, y,
                                         transparent as c_bool,
                                         walkable as c_bool);
        }
    }

    pub fn compute_fov(&mut self, origin_x: i32, origin_y: i32, max_radius: i32,
                       light_walls: bool, algo: FovAlgorithm) {
        assert!(origin_x >= 0 && origin_y >= 0);
        unsafe {
            ffi::TCOD_map_compute_fov(self.tcod_map, origin_x, origin_y, max_radius,
                                     light_walls as c_bool,
                                     algo as u32);
        }
    }

    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_is_in_fov(self.tcod_map, x, y) != 0
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_is_walkable(self.tcod_map, x, y) != 0
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_map_delete(self.tcod_map)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FovAlgorithm {
    Basic       = ffi::FOV_BASIC as isize,
    Diamond     = ffi::FOV_DIAMOND as isize,
    Shadow      = ffi::FOV_SHADOW as isize,
    Permissive0 = ffi::FOV_PERMISSIVE_0 as isize,
    Permissive1 = ffi::FOV_PERMISSIVE_1 as isize,
    Permissive2 = ffi::FOV_PERMISSIVE_2 as isize,
    Permissive3 = ffi::FOV_PERMISSIVE_3 as isize,
    Permissive4 = ffi::FOV_PERMISSIVE_4 as isize,
    Permissive5 = ffi::FOV_PERMISSIVE_5 as isize,
    Permissive6 = ffi::FOV_PERMISSIVE_6 as isize,
    Permissive7 = ffi::FOV_PERMISSIVE_7 as isize,
    Permissive8 = ffi::FOV_PERMISSIVE_8 as isize,
    Restrictive = ffi::FOV_RESTRICTIVE as isize,
}
