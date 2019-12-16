use bindings::{ffi, c_int};
use bindings::{AsNative};
use noise::Noise;
use random::Rng;
use std::slice;

#[derive(Debug)]
pub struct HeightMap {
    height_map: *mut ffi::TCOD_heightmap_t,
    width: i32,
    height: i32,
}

impl AsNative<ffi::TCOD_heightmap_t> for HeightMap {
    unsafe fn as_native(&self) -> &ffi::TCOD_heightmap_t {
        &*self.height_map
    }

    unsafe fn as_native_mut(&mut self) -> &mut ffi::TCOD_heightmap_t {
        &mut *self.height_map
    }
}

impl HeightMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            height_map: unsafe { ffi::TCOD_heightmap_new(width, height) },
            width,
            height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn values(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                (*self.height_map).values,
                (self.width * self.height) as usize,
            )
        }
    }

    pub fn get_value(&self, x: i32, y: i32) -> f32 {
        unsafe { ffi::TCOD_heightmap_get_value(self.height_map, x, y) }
    }

    pub fn get_interpolated_value(&self, x: f32, y: f32) -> f32 {
        unsafe { ffi::TCOD_heightmap_get_interpolated_value(self.height_map, x, y) }
    }

    pub fn set_value(&mut self, x: i32, y: i32, value: f32) {
        unsafe { ffi::TCOD_heightmap_set_value(self.height_map, x, y, value) }
    }

    pub fn get_slope(&self, x: i32, y: i32) -> f32 {
        unsafe { ffi::TCOD_heightmap_get_slope(self.height_map, x, y) }
    }

    pub fn get_normal(&self, x: f32, y: f32, water_level: f32) -> [f32; 3] {
        let mut n = [0.0f32; 3];
        unsafe {
            ffi::TCOD_heightmap_get_normal(self.height_map, x, y, n.as_mut_ptr(), water_level)
        }

        n
    }

    pub fn count_cells(&self, min: f32, max: f32) -> i32 {
        unsafe { ffi::TCOD_heightmap_count_cells(self.height_map, min, max) }
    }

    pub fn has_land_on_border(&self, water_level: f32) -> bool {
        unsafe { ffi::TCOD_heightmap_has_land_on_border(self.height_map, water_level) != 0 }
    }

    pub fn get_minmax(&self) -> MinMax {
        let mut m = MinMax { min: 0.0, max: 0.0 };
        unsafe {
            ffi::TCOD_heightmap_get_minmax(
                self.height_map,
                &mut m.min as *mut f32,
                &mut m.max as *mut f32,
            )
        }

        m
    }

    pub fn copy(&self, dest: &mut HeightMap) {
        unsafe { ffi::TCOD_heightmap_copy(self.height_map, dest.height_map) }
    }

    pub fn add(&mut self, value: f32) {
        unsafe { ffi::TCOD_heightmap_add(self.height_map, value) }
    }

    pub fn scale(&mut self, value: f32) {
        unsafe { ffi::TCOD_heightmap_scale(self.height_map, value) }
    }

    pub fn clamp(&mut self, min: f32, max: f32) {
        unsafe { ffi::TCOD_heightmap_clamp(self.height_map, min, max) }
    }

    pub fn normalize(&mut self, min: f32, max: f32) {
        unsafe { ffi::TCOD_heightmap_normalize(self.height_map, min, max) }
    }

    pub fn clear(&mut self) {
        unsafe { ffi::TCOD_heightmap_clear(self.height_map) }
    }

    pub fn lerp_hm(&self, other: &HeightMap, coefficient: f32) -> HeightMap {
        let res = HeightMap::new(self.width, self.height);
        unsafe {
            ffi::TCOD_heightmap_lerp_hm(
                self.height_map,
                other.height_map,
                res.height_map,
                coefficient,
            )
        }

        res
    }

    pub fn add_hm(&self, other: &HeightMap) -> HeightMap {
        let res = HeightMap::new(self.width, self.height);
        unsafe { ffi::TCOD_heightmap_add_hm(self.height_map, other.height_map, res.height_map) }

        res
    }

    pub fn multiply_hm(&self, other: &HeightMap) -> HeightMap {
        let res = HeightMap::new(self.width, self.height);
        unsafe {
            ffi::TCOD_heightmap_multiply_hm(self.height_map, other.height_map, res.height_map)
        }

        res
    }

    pub fn add_hill(&mut self, x: f32, y: f32, radius: f32, height: f32) {
        unsafe { ffi::TCOD_heightmap_add_hill(self.height_map, x, y, radius, height) }
    }

    pub fn dig_hill(&mut self, x: f32, y: f32, radius: f32, height: f32) {
        unsafe { ffi::TCOD_heightmap_dig_hill(self.height_map, x, y, radius, height) }
    }

    pub fn dig_bezier(
        &mut self,
        mut px: [i32; 4],
        mut py: [i32; 4],
        start_radius: f32,
        start_depth: f32,
        end_radius: f32,
        end_depth: f32,
    ) {
        unsafe {
            ffi::TCOD_heightmap_dig_bezier(
                self.height_map,
                px.as_mut_ptr(),
                py.as_mut_ptr(),
                start_radius,
                start_depth,
                end_radius,
                end_depth,
            )
        }
    }

    pub fn rain_erosion(
        &mut self,
        drops: i32,
        erosion_coefficient: f32,
        sedimentation_coefficient: f32,
        rnd: &Rng,
    ) {
        unsafe {
            ffi::TCOD_heightmap_rain_erosion(
                self.height_map,
                drops,
                erosion_coefficient,
                sedimentation_coefficient,
                *rnd.as_native(),
            )
        }
    }

    pub fn kernel_transform(
        &mut self,
        dx: &[i32],
        dy: &[i32],
        weight: &[f32],
        min_level: f32,
        max_level: f32,
    ) {
        assert_eq!(dx.len(), dy.len());
        assert_eq!(dx.len(), weight.len());

        let kernel_size = dx.len() as c_int;

        unsafe {
            ffi::TCOD_heightmap_kernel_transform(
                self.height_map,
                kernel_size,
                dx.as_ptr(),
                dy.as_ptr(),
                weight.as_ptr(),
                min_level,
                max_level,
            )
        }
    }

    pub fn add_voronoi(&mut self, points: i32, coefficients: &[f32], rnd: &Rng) {
        unsafe {
            ffi::TCOD_heightmap_add_voronoi(
                self.height_map,
                points,
                coefficients.len() as c_int,
                coefficients.as_ptr(),
                *rnd.as_native(),
            )
        }
    }

    pub fn mid_point_displacement(&mut self, rnd: &Rng, roughness: f32) {
        unsafe {
            ffi::TCOD_heightmap_mid_point_displacement(self.height_map, *rnd.as_native(), roughness)
        }
    }

    pub fn add_fbm(
        &mut self,
        noise: &Noise,
        mul_x: f32,
        mul_y: f32,
        add_x: f32,
        add_y: f32,
        octaves: f32,
        delta: f32,
        scale: f32,
    ) {
        unsafe {
            ffi::TCOD_heightmap_add_fbm(
                self.height_map,
                *noise.as_native(),
                mul_x,
                mul_y,
                add_x,
                add_y,
                octaves,
                delta,
                scale,
            )
        }
    }

    pub fn scale_fbm(
        &mut self,
        noise: &Noise,
        mul_x: f32,
        mul_y: f32,
        add_x: f32,
        add_y: f32,
        octaves: f32,
        delta: f32,
        scale: f32,
    ) {
        unsafe {
            ffi::TCOD_heightmap_scale_fbm(
                self.height_map,
                *noise.as_native(),
                mul_x,
                mul_y,
                add_x,
                add_y,
                octaves,
                delta,
                scale,
            )
        }
    }

    pub fn islandify(&mut self, sea_level: f32, rnd: &Rng) {
        unsafe { ffi::TCOD_heightmap_islandify(self.height_map, sea_level, *rnd.as_native()) }
    }
}

impl Drop for HeightMap {
    fn drop(&mut self) {
        unsafe { ffi::TCOD_heightmap_delete(self.height_map) }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct MinMax {
    min: f32,
    max: f32,
}

impl From<MinMax> for (f32, f32) {
    fn from(m: MinMax) -> Self {
        (m.min, m.max)
    }
}
