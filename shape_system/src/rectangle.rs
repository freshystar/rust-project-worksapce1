use crate::traits::area::Shape;

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    length: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

impl Rectangle {
  pub  fn new() -> Self {
        Self { length: 12.4, width: 7.5}
    }
}









