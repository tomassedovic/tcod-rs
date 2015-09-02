extern crate libc;

use bindings::ffi;
use bindings::AsNative;

use random::Rng;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum NoiseType {
    Default = ffi::TCOD_NOISE_DEFAULT as isize,
    Perlin  = ffi::TCOD_NOISE_PERLIN as isize,
    Simplex = ffi::TCOD_NOISE_SIMPLEX as isize,
    Wavelet = ffi::TCOD_NOISE_WAVELET as isize,
}

#[derive(Debug)]
pub struct Noise {
    noise: ffi::TCOD_noise_t,
    dimensions: u32
}

const TCOD_NOISE_DEFAULT_HURST: f32 = 0.5;
const TCOD_NOISE_DEFAULT_LACUNARITY: f32 = 2.0;

impl Noise {
    pub fn initializer() -> NoiseInitializer {
        NoiseInitializer::new()
    }

    pub fn set_type(&self, noise_type: NoiseType) {
        unsafe {
            ffi::TCOD_noise_set_type(self.noise, noise_type as u32)
        }
    }

    pub fn get(&self, coords: &mut [f32]) -> f32 {
        assert!(self.dimensions as usize == coords.len());
        unsafe {
            ffi::TCOD_noise_get(self.noise, coords.as_mut_ptr())
        }
    }

    pub fn get_ex(&self, coords: &mut [f32], noise_type: NoiseType) -> f32 {
        assert!(self.dimensions as usize == coords.len());
        unsafe {
            ffi::TCOD_noise_get_ex(self.noise, coords.as_mut_ptr(), noise_type as u32)
        }
    }
}

impl Drop for Noise {
    fn drop(&mut self) {
        unsafe { ffi::TCOD_noise_delete(self.noise) }
    }
}

pub struct NoiseInitializer {
    dimensions: u32,
    hurst: f32,
    lacunarity: f32,
    noise_type: NoiseType,
    random: Rng
}

impl NoiseInitializer {
    fn new() -> Self {
        NoiseInitializer {
            dimensions: 2,
            hurst: TCOD_NOISE_DEFAULT_HURST,
            lacunarity: TCOD_NOISE_DEFAULT_LACUNARITY,
            noise_type: NoiseType::Default,
            random: Rng::get_instance()
        }
    }

    pub fn dimensions(&mut self, dimensions: u32) -> &mut Self {
        self.dimensions = dimensions;
        self
    }

    pub fn hurst(&mut self, hurst: f32) -> &mut Self {
        self.hurst = hurst;
        self
    }

    pub fn lacunarity(&mut self, lacunarity: f32) -> &mut Self {
        self.lacunarity = lacunarity;
        self
    }

    pub fn noise_type(&mut self, noise_type: NoiseType) -> &mut Self {
        self.noise_type = noise_type;
        self
    }

    pub fn random(&mut self, random: Rng) -> &mut Self {
        self.random = random;
        self
    }

    pub fn init(&self) -> Noise {
        unsafe {
            let noise = Noise {
                noise: ffi::TCOD_noise_new(self.dimensions as i32, self.hurst,
                                           self.lacunarity, *self.random.as_native()),
                dimensions: self.dimensions,
            };
            ffi::TCOD_noise_set_type(noise.noise, self.noise_type as u32);
            noise
        }
    }
}

#[cfg(test)]
mod test {
    use super::Noise;
    use super::NoiseType;

    #[test]
    fn get() {
        let noise1d = Noise::initializer().dimensions(1).init();
        let noise2d = Noise::initializer().dimensions(2).init();
        let noise3d = Noise::initializer().dimensions(3).init();

        let val1  = noise1d.get(&mut [1.0]);
        let val1a = noise1d.get(&mut [1.0]);
        assert!(val1 >= -1.0 && val1 <= 1.0);
        assert_eq!(val1, val1a);

        let val2  = noise2d.get(&mut [1.0, 2.0]);
        let val2a = noise2d.get(&mut [1.0, 2.0]);
        assert!(val2 >= -1.0 && val2 <= 1.0);
        assert_eq!(val2, val2a);

        let val3  = noise3d.get(&mut [1.0, 2.0, 3.0]);
        let val3a = noise3d.get(&mut [1.0, 2.0, 3.0]);
        assert!(val3 >= -1.0 && val3 <= 1.0);
        assert_eq!(val3, val3a);
    }

    #[test]
    #[should_panic]
    fn get_not_enough_args() {
        let noise2d = Noise::initializer().dimensions(2).init();
        noise2d.get(&mut [1.0]);
    }

    #[test]
    #[should_panic]
    fn get_too_many_args() {
        let noise2d = Noise::initializer().dimensions(2).init();
        noise2d.get(&mut [1.0, 2.0, 3.0]);
    }

    #[test]
    fn get_ex() {
        let noise2d = Noise::initializer().init();

        let val1  = noise2d.get_ex(&mut [1.0, 2.0], NoiseType::Perlin);
        let val1a = noise2d.get_ex(&mut [1.0, 2.0], NoiseType::Perlin);
        assert!(val1 >= -1.0 && val1 <= 1.0);
        assert_eq!(val1, val1a);

        let val2  = noise2d.get_ex(&mut [1.0, 2.0], NoiseType::Wavelet);
        let val2a = noise2d.get_ex(&mut [1.0, 2.0], NoiseType::Wavelet);
        assert!(val2 >= -1.0 && val2 <= 1.0);
        assert_eq!(val2, val2a);
    }
}
