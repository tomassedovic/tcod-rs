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
    noise: ffi::TCOD_noise_t
}

const TCOD_NOISE_DEFAULT_HURST: f32 = 0.5;
const TCOD_NOISE_DEFAULT_LACUNARITY: f32 = 2.0;

impl Noise {
    pub fn initializer() -> NoiseInitializer {
        NoiseInitializer::new()
    }
}

pub struct NoiseInitializer {
    dimensions: i32,
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

    pub fn dimensions(&mut self, dimensions: i32) -> &mut Self {
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
                noise: ffi::TCOD_noise_new(self.dimensions, self.hurst,
                                           self.lacunarity, *self.random.as_native())
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
    fn default_noise() {
        let noise = Noise::initializer().init();
        
    }

    #[test]
    fn custom_noise() {
        let noise = Noise::initializer()
            .dimensions(3)
            .hurst(0.7)
            .lacunarity(2.1)
            .noise_type(NoiseType::Perlin)
            .init();
    }
}
