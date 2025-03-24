use container::Container;

fn main() {
let array1 =  Container::max_area([1,8,6,2,5,4,8,3,7].to_vec() );

println!("Enter an array of integers");


println!("The container formed is: {}", array1);
}

mod container;