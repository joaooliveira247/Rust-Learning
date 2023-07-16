#[allow(dead_code)]
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

#[allow(dead_code)]
impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
    pub fn x(&self) -> &T {
        &self.x
    }
}