#[macro_export]
macro_rules! remap {
    (value: $value: expr, from: $min1: expr, $max1: expr, to: $min2: expr, $max2: expr) => {{
        let value = $value;
        let min1 = $min1;
        let max1 = $max1;
        let min2 = $min2;
        let max2 = $max2;

        min2 + (value - min1) * (max2 - min2) / (max1 - min1)
    }};
}

/// <https://www.youtube.com/watch?v=LSNQuFEDOyQ>
pub const DECAY: f32 = 16.0;
/// <https://www.youtube.com/watch?v=LSNQuFEDOyQ>
pub fn exponential_decay(current: f32, goal: f32, decay: f32, dt: f32) -> f32 {
    (current - goal).mul_add(f32::exp(-decay * dt), goal)
}
