pub fn smooth_stop(t: f32, power: i32) -> f32 {
    1.0 - f32::powf(1.0 - t, power as f32)
}
