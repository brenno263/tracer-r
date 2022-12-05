pub fn lerp(x0: f32, x1: f32, t: f32) -> f32 {
    t * x0 + (1. - t) * x1
}
