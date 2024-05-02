use rand::Rng;

pub fn cap(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn rnd(mult: i32) -> f32 {
    let mut rng = rand::thread_rng();
    return (mult as f32) * rng.gen_range(0..10) as f32 / 10.0;
}
