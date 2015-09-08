use bindings::ffi;
use bindings::AsNative;
use random::Rng;

pub struct BSP {
    bsp: *mut ffi::TCOD_bsp_t,
}

impl BSP {
    pub fn new_with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        let bsp = unsafe {
            ffi::TCOD_bsp_new_with_size(x, y, w,  h)
        };
        BSP { bsp: bsp }
    }

    pub fn remove_sons(&self) {
        unsafe { ffi::TCOD_bsp_remove_sons(self.bsp) }
    }

    pub fn split_once(&self, horizontal: bool, position: i32) {
        unsafe { ffi::TCOD_bsp_split_once(self.bsp, horizontal as u8, position) }
    }

    pub fn split_recursive(&self,
                           randomizer: Rng,
                           nb: i32,
                           min_h_size: i32,
                           min_v_size: i32,
                           max_h_ratio: f32,
                           max_v_ratio: f32) {
        unsafe {
            ffi::TCOD_bsp_split_recursive(self.bsp,
                                          *randomizer.as_native(),
                                          nb,
                                          min_h_size,
                                          min_v_size,
                                          max_h_ratio,
                                          max_v_ratio)
        }
    }

    pub fn resize(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::TCOD_bsp_resize(self.bsp, x, y, w, h) }
    }

    pub fn left(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_left(self.bsp) }
        }
    }

    pub fn right(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_right(self.bsp) }
        }
    }

    pub fn father(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_father(self.bsp) }
        }
    }

    pub fn is_leaf(&self) -> bool {
        unsafe { ffi::TCOD_bsp_is_leaf(self.bsp) != 0 }
    }

    pub fn contains(&self, cx: i32, cy: i32) -> bool {
        unsafe { ffi::TCOD_bsp_contains(self.bsp, cx, cy) != 0 }
    }
}

impl Drop for BSP {
    fn drop(&mut self) {
        unsafe { ffi::TCOD_bsp_delete(self.bsp) }
    }
}

#[cfg(test)]
mod test {
    use super::BSP;

    #[test]
    #[allow(unused_variables)]
    fn created_destroyed_no_panic() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
    }
}
