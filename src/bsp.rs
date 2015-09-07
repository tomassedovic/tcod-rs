use bindings::ffi;

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
