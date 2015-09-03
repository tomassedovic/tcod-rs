//! Noise toolkit.
//!
//! Rust bindings follow the original API pretty closely.

use bindings::ffi;
use bindings::AsNative;

use random::Rng;

/// Available noise types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum NoiseType {
    Default = ffi::TCOD_NOISE_DEFAULT as isize,
    Perlin  = ffi::TCOD_NOISE_PERLIN as isize,
    Simplex = ffi::TCOD_NOISE_SIMPLEX as isize,
    Wavelet = ffi::TCOD_NOISE_WAVELET as isize,
}

/// Noise object encapsulates a noise generator.
#[derive(Debug)]
pub struct Noise {
    noise: ffi::TCOD_noise_t,
    dimensions: u32
}

/// Default hurst value.
pub const DEFAULT_HURST: f32 = 0.5;

/// Default lacunarity value.
pub const DEFAULT_LACUNARITY: f32 = 2.0;

/// Maximum number of octaves for turbulence and fbm noise functions.
pub const MAX_OCTAVES: u32 = 128;

impl Noise {
    /// Return an instance of [NoiseInitializer](./struct.NoiseInitializer.html)
    /// which is used to customize the creation of Noise object.
    pub fn init_with_dimensions(dimensions: u32) -> NoiseInitializer {
        NoiseInitializer::new_init_with_dimensions(dimensions)
    }

    pub fn set_type(&self, noise_type: NoiseType) {
        unsafe {
            ffi::TCOD_noise_set_type(self.noise, noise_type as u32)
        }
    }

    pub fn get<T: AsMut<[f32]>>(&self, mut coords: T) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        unsafe {
            ffi::TCOD_noise_get(self.noise, coords.as_mut().as_mut_ptr())
        }
    }

    pub fn get_ex<T: AsMut<[f32]>>(&self, mut coords: T, noise_type: NoiseType) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        unsafe {
            ffi::TCOD_noise_get_ex(self.noise,
                                   coords.as_mut().as_mut_ptr(),
                                   noise_type as u32)
        }
    }

    pub fn get_fbm<T: AsMut<[f32]>>(&self, mut coords: T, octaves: u32) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        assert!(octaves > 0);
        assert!(octaves < MAX_OCTAVES);
        unsafe {
            ffi::TCOD_noise_get_fbm(self.noise, coords.as_mut().as_mut_ptr(), octaves as f32)
        }
    }

    pub fn get_fbm_ex<T: AsMut<[f32]>>(&self, mut coords: T, octaves: u32, noise_type: NoiseType) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        assert!(octaves > 0);
        assert!(octaves < MAX_OCTAVES);
        unsafe {
            ffi::TCOD_noise_get_fbm_ex(self.noise,
                                       coords.as_mut().as_mut_ptr(),
                                       octaves as f32,
                                       noise_type as u32)
        }
    }

    pub fn get_turbulence<T: AsMut<[f32]>>(&self, mut coords: T, octaves: u32) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        assert!(octaves > 0);
        assert!(octaves < MAX_OCTAVES);
        unsafe {
            ffi::TCOD_noise_get_turbulence(self.noise,
                                           coords.as_mut().as_mut_ptr(),
                                           octaves as f32)
        }
    }

    pub fn get_turbulence_ex<T: AsMut<[f32]>>(&self, mut coords: T, octaves: u32, noise_type: NoiseType) -> f32 {
        assert!(self.dimensions as usize == coords.as_mut().len());
        assert!(octaves > 0);
        assert!(octaves < MAX_OCTAVES);
        unsafe {
            ffi::TCOD_noise_get_turbulence_ex(self.noise,
                                              coords.as_mut().as_mut_ptr(),
                                              octaves as f32,
                                              noise_type as u32)
        }
    }
}

impl Drop for Noise {
    fn drop(&mut self) {
        unsafe { ffi::TCOD_noise_delete(self.noise) }
    }
}

/// An initializer is used to customize creation of a `Noise` object.
pub struct NoiseInitializer {
    dimensions: u32,
    hurst: f32,
    lacunarity: f32,
    noise_type: NoiseType,
    random: Rng
}

impl NoiseInitializer {
    fn new_init_with_dimensions(dimensions: u32) -> Self {
        assert!(dimensions > 0 && dimensions <= 4);
        NoiseInitializer {
            dimensions: dimensions,
            hurst: DEFAULT_HURST,
            lacunarity: DEFAULT_LACUNARITY,
            noise_type: NoiseType::Default,
            random: Rng::get_instance()
        }
    }

    /// Sets the hurst value of the noise generator.
    pub fn hurst(&mut self, hurst: f32) -> &mut Self {
        self.hurst = hurst;
        self
    }

    /// Sets the lacunarity value of the noise generator.
    pub fn lacunarity(&mut self, lacunarity: f32) -> &mut Self {
        self.lacunarity = lacunarity;
        self
    }

    /// Sets the noise type the generator produces.
    pub fn noise_type(&mut self, noise_type: NoiseType) -> &mut Self {
        self.noise_type = noise_type;
        self
    }

    /// Sets a custom random number generator. Use
    /// [tcod::random::Rng](../random/struct.Rng.html) instance.
    pub fn random(&mut self, random: Rng) -> &mut Self {
        self.random = random;
        self
    }

    /// Finalizes creation and returns a new `Noise` object.
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
        let noise1d = Noise::init_with_dimensions(1).init();
        let noise2d = Noise::init_with_dimensions(2).init();
        let noise3d = Noise::init_with_dimensions(3).init();

        let val1  = noise1d.get([1.0]);
        let val1a = noise1d.get([1.0]);
        assert!(val1 >= -1.0 && val1 <= 1.0);
        assert_eq!(val1, val1a);

        let val2  = noise2d.get([1.0, 2.0]);
        let val2a = noise2d.get([1.0, 2.0]);
        assert!(val2 >= -1.0 && val2 <= 1.0);
        assert_eq!(val2, val2a);

        let val3  = noise3d.get([1.0, 2.0, 3.0]);
        let val3a = noise3d.get([1.0, 2.0, 3.0]);
        assert!(val3 >= -1.0 && val3 <= 1.0);
        assert_eq!(val3, val3a);
    }

    #[test]
    #[should_panic]
    fn init_with_wrong_dimensions() {
        Noise::init_with_dimensions(5).init();
    }

    #[test]
    #[should_panic]
    fn init_with_zero_dimensions() {
        Noise::init_with_dimensions(0).init();
    }

    #[test]
    #[should_panic]
    fn get_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get([1.0]);
    }

    #[test]
    #[should_panic]
    fn get_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get([1.0, 2.0, 3.0]);
    }

    #[test]
    fn get_ex() {
        let noise2d = Noise::init_with_dimensions(2).init();

        let val1  = noise2d.get_ex([1.0, 2.0], NoiseType::Perlin);
        let val1a = noise2d.get_ex([1.0, 2.0], NoiseType::Perlin);
        assert!(val1 >= -1.0 && val1 <= 1.0);
        assert_eq!(val1, val1a);

        let val2  = noise2d.get_ex([1.0, 2.0], NoiseType::Wavelet);
        let val2a = noise2d.get_ex([1.0, 2.0], NoiseType::Wavelet);
        assert!(val2 >= -1.0 && val2 <= 1.0);
        assert_eq!(val2, val2a);
    }

    #[test]
    #[should_panic]
    fn get_ex_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_ex([1.0], NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_ex_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_ex([1.0, 2.0, 3.0], NoiseType::Perlin);
    }

    #[test]
    fn get_fbm() {
        let noise1d = Noise::init_with_dimensions(1).init();
        let noise2d = Noise::init_with_dimensions(2).init();
        let noise3d = Noise::init_with_dimensions(3).init();

        let val1  = noise1d.get_fbm([1.0], 32);
        let val1a = noise1d.get_fbm([1.0], 32);
        assert!(val1.is_nan() || val1 >= -1.0 && val1 <= 1.0);
        if !val1.is_nan() {
            assert_eq!(val1, val1a);
        }

        let val2  = noise2d.get_fbm([1.0, 2.0], 32);
        let val2a = noise2d.get_fbm([1.0, 2.0], 32);
        assert!(val2.is_nan() || val2 >= -1.0 && val2 <= 1.0);
        if !val2.is_nan() {
            assert_eq!(val2, val2a);
        }

        let val3  = noise3d.get_fbm([1.0, 2.0, 3.0], 32);
        let val3a = noise3d.get_fbm([1.0, 2.0, 3.0], 32);
        assert!(val3.is_nan() || val3 >= -1.0 && val3 <= 1.0);
        if !val3.is_nan() {
            assert_eq!(val3, val3a);
        }
    }

    #[test]
    #[should_panic]
    fn get_fbm_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm([1.0], 32);
    }

    #[test]
    #[should_panic]
    fn get_fbm_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm([1.0, 2.0, 3.0], 32);
    }

    #[test]
    #[should_panic]
    fn get_fbm_octaves_zero() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm([1.0, 2.0, 3.0], 0);
    }

    #[test]
    #[should_panic]
    fn get_fbm_octaves_too_big() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm([1.0, 2.0, 3.0], 128);
    }

    #[test]
    fn get_fbm_ex() {
        let noise2d = Noise::init_with_dimensions(2).init();

        let val1  = noise2d.get_fbm_ex([1.0, 2.0], 32, NoiseType::Perlin);
        let val1a = noise2d.get_fbm_ex([1.0, 2.0], 32, NoiseType::Perlin);
        assert!(val1.is_nan() || val1 >= -1.0 && val1 <= 1.0);
        if !val1.is_nan() {
            assert_eq!(val1, val1a);
        }

        let val2  = noise2d.get_fbm_ex([1.0, 2.0], 64, NoiseType::Wavelet);
        let val2a = noise2d.get_fbm_ex([1.0, 2.0], 64, NoiseType::Wavelet);
        assert!(val2.is_nan() || val2 >= -1.0 && val2 <= 1.0);
        if !val2.is_nan() {
            assert_eq!(val2, val2a);
        }
    }

    #[test]
    #[should_panic]
    fn get_fbm_ex_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm_ex([1.0], 32, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_fbm_ex_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm_ex([1.0, 2.0, 3.0], 32, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_fbm_ex_octaves_zero() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm_ex([1.0, 2.0, 3.0], 0, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_fbm_ex_octaves_too_big() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_fbm_ex([1.0, 2.0, 3.0], 128, NoiseType::Perlin);
    }

    #[test]
    fn get_turbulence() {
        let noise1d = Noise::init_with_dimensions(1).init();
        let noise2d = Noise::init_with_dimensions(2).init();
        let noise3d = Noise::init_with_dimensions(3).init();

        let val1  = noise1d.get_turbulence([1.0], 32);
        let val1a = noise1d.get_turbulence([1.0], 32);
        assert!(val1.is_nan() || val1 >= -1.0 && val1 <= 1.0);
        if !val1.is_nan() {
            assert_eq!(val1, val1a);
        }

        let val2  = noise2d.get_turbulence([1.0, 2.0], 32);
        let val2a = noise2d.get_turbulence([1.0, 2.0], 32);
        assert!(val2.is_nan() || val2 >= -1.0 && val2 <= 1.0);
        if !val2.is_nan() {
            assert_eq!(val2, val2a);
        }

        let val3  = noise3d.get_turbulence([1.0, 2.0, 3.0], 32);
        let val3a = noise3d.get_turbulence([1.0, 2.0, 3.0], 32);
        assert!(val3.is_nan() || val3 >= -1.0 && val3 <= 1.0);
        if !val3.is_nan() {
            assert_eq!(val3, val3a);
        }
    }

    #[test]
    #[should_panic]
    fn get_turbulence_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence([1.0], 32);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence([1.0, 2.0, 3.0], 32);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_octaves_zero() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence([1.0, 2.0, 3.0], 0);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_octaves_too_big() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence([1.0, 2.0, 3.0], 128);
    }

    #[test]
    fn get_turbulence_ex() {
        let noise2d = Noise::init_with_dimensions(2).init();

        let val1  = noise2d.get_turbulence_ex([1.0, 2.0], 32, NoiseType::Perlin);
        let val1a = noise2d.get_turbulence_ex([1.0, 2.0], 32, NoiseType::Perlin);
        assert!(val1.is_nan() || val1 >= -1.0 && val1 <= 1.0);
        if !val1.is_nan() {
            assert_eq!(val1, val1a);
        }

        let val2  = noise2d.get_turbulence_ex([1.0, 2.0], 64, NoiseType::Wavelet);
        let val2a = noise2d.get_turbulence_ex([1.0, 2.0], 64, NoiseType::Wavelet);
        assert!(val2.is_nan() || val2 >= -1.0 && val2 <= 1.0);
        if !val2.is_nan() {
            assert_eq!(val2, val2a);
        }
    }

    #[test]
    #[should_panic]
    fn get_turbulence_ex_not_enough_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence_ex([1.0], 32, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_ex_too_many_args() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence_ex([1.0, 2.0, 3.0], 32, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_ex_octaves_zero() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence_ex([1.0, 2.0, 3.0], 0, NoiseType::Perlin);
    }

    #[test]
    #[should_panic]
    fn get_turbulence_ex_octaves_too_big() {
        let noise2d = Noise::init_with_dimensions(2).init();
        noise2d.get_turbulence_ex([1.0, 2.0, 3.0], 128, NoiseType::Perlin);
    }
}
