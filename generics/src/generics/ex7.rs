#[allow(dead_code)]
#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U
}

#[allow(dead_code)]
impl<T, U> Point<T, U> {
    pub fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}