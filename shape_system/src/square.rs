use crate::traits::area::Shape;

#[derive(Debug)]
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powf(2.0)
    }
}
impl Square {
 pub   fn new() -> Self {
        Self { side: 6.4 }
    }
}
