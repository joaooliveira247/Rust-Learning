#[allow(dead_code)]
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl Point<f32> {
    pub fn origin_distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
