
use std::f32::consts::PI;

#[inline] pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

#[inline] pub fn rad_to_deg(radians: f32) -> f32 {
    radians * (180.0 / PI)
}

use std::ops::*;
#[inline] pub fn normalize<T>(x: T, min_x: T, max_x: T, min: T, max: T) -> T
where T: Copy + Add<Output = T> + Div<Output = T> + Sub<Output = T> + Mul<Output = T>
{
    (max - min) * ((x - min_x) / (max_x - min_x)) + min
}

#[inline] pub fn deadzone(x: f32, delta: f32) -> f32 {
    if x.abs() < delta {0.0}
    else {x}
}
