
use std::f32::consts::PI;

#[inline] pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

#[inline] pub fn rad_to_deg(radians: f32) -> f32 {
    radians * (180.0 / PI)
}
