use bindings::ffi;
use bindings::AsNative;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Distribution {
    Linear = ffi::TCOD_DISTRIBUTION_LINEAR as isize,
    Gaussian = ffi::TCOD_DISTRIBUTION_GAUSSIAN as isize,
    GaussianRange = ffi::TCOD_DISTRIBUTION_GAUSSIAN_RANGE as isize,
    GaussianInverse = ffi::TCOD_DISTRIBUTION_GAUSSIAN_INVERSE as isize,
    GaussianRangeInverse = ffi::TCOD_DISTRIBUTION_GAUSSIAN_RANGE_INVERSE as isize,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Algo {
    MT = ffi::TCOD_RNG_MT as isize,
    CMWC = ffi::TCOD_RNG_CMWC as isize
}

pub struct Rng {
    tcod_random: ffi::TCOD_random_t,
    default: bool
}

impl Rng {
    pub fn get_instance() -> Rng {
        unsafe {
            Rng { tcod_random: ffi::TCOD_random_get_instance(), default: true }
        }
    }

    pub fn new(algo: Algo) -> Rng {
        unsafe {
            Rng { tcod_random: ffi::TCOD_random_new(algo as u32), default: false }
        }
    }

    pub fn new_with_seed(algo: Algo, seed: u32) -> Rng {
        unsafe {
            Rng { tcod_random: ffi::TCOD_random_new_from_seed(algo as u32, seed), default: false }
        }
    }

    pub fn save(&self) -> Rng {
        unsafe {
            Rng { tcod_random: ffi::TCOD_random_save(self.tcod_random), default: false }
        }
    }

    pub fn restore(&mut self, backup: &Rng) {
        unsafe {
            ffi::TCOD_random_restore(self.tcod_random, backup.tcod_random);
        }
    }

    pub fn set_distribution(&self, distribution: Distribution) {
        unsafe {
            ffi::TCOD_random_set_distribution(self.tcod_random, distribution as u32);
        }
    }

    pub fn get_int(&self, min: i32, max: i32) -> i32 {
        unsafe {
            ffi::TCOD_random_get_int(self.tcod_random, min, max)
        }
    }

    pub fn get_int_mean(&self, min: i32, max: i32, mean: i32) -> i32 {
        unsafe {
            ffi::TCOD_random_get_int_mean(self.tcod_random,  min, max, mean)
        }
    }

    pub fn get_float(&self, min: f32, max: f32) -> f32 {
        unsafe {
            ffi::TCOD_random_get_float(self.tcod_random, min, max)
        }
    }

    pub fn get_float_mean(&self, min: f32, max: f32, mean: f32) -> f32 {
        unsafe {
            ffi::TCOD_random_get_float_mean(self.tcod_random, min, max, mean)
        }
    }

    pub fn get_double(&self, min: f64, max: f64) -> f64 {
        unsafe {
            ffi::TCOD_random_get_double(self.tcod_random, min, max)
        }
    }

    pub fn get_double_mean(&self, min: f64, max: f64, mean: f64) -> f64 {
        unsafe {
            ffi::TCOD_random_get_double_mean(self.tcod_random, min, max, mean)
        }
    }
}

impl AsNative<ffi::TCOD_random_t> for Rng {
    unsafe fn as_native(&self) -> &ffi::TCOD_random_t {
        &self.tcod_random
    }
}

impl Drop for Rng {
    fn drop(&mut self) {
        if !self.default {
            unsafe {
                ffi::TCOD_random_delete(self.tcod_random);
            }
        }
    }
}


