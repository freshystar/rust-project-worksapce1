
use rectangle::Rectangle;
use square::Square;
use traits::area::Shape;



fn main() {
    let rectangle = Rectangle::new();
    let square = Square::new();

    println!("{:?} rectangle: {} ", rectangle, rectangle.area());
    println!("{:?} area: {} ", square, square.area());

}    

mod traits;
mod rectangle;
mod square;