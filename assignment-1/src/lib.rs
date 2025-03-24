#[cfg(test)]

mod tests {
    use std::{
        fmt::format,
        fs::File,
        process::{Command, Stdio},
    };

    use super::*;

    #[test]

    /// TODO
    /// finding the number of occurences of an item in an array of items.
    /// the item should be copied<T, Copy>
    /// Get the complexity.
    /// optimise it(what can you do to optimise it)
    /// write a test case for it. Handle the case where the array is empry.
    /// use a dictionary to store the item and the occurences
    /// return the dictionary. Order the occurences
    ///
    /// TODO-2
    /// create a function that returns an object with two methods(two closures)
    /// the first method is going to increment the private counter by one
    /// the second one is going to return the value of the private counter.
    /// create a closure that can only be called once
    /// verify that the method can only be called once(writing a method for that)
    ///
    /// TODO-3
    /// Suppose you are given a tuple person which takes[name, age]
    /// you are asked to implement the sorting function of a vector of people in an alphabetical order and age, the priority is given to alphabetical order.
    ///
    /// TODO-4
    /// do the same thing(TODO-3) using the "sort_by" method of iterator and parse in a closure
    /// implement a closure that you will parse into the "sort_by" built in function to do the same thing as in [TODO-3]
    ///
    /// TODO-5
    ///
    fn get_ip() {
        let mut valid_ips = vec![];
        for i in 41..=231 {
            let ip = format!("192.168.1.{i}");
            let result = Command::new("nc")
                .arg(ip.as_str())
                .arg("5555")
                .stdin(Stdio::from(
                    File::open("/home/fresnelle/projects/Rust-projects/assignment-1/src/main.rs")
                        .expect("Could not open file"),
                ))
                .stdout(Stdio::null())
                .output();
            match result {
                Ok(_) => {
                    valid_ips.push(ip);
                    println!(" ✅ Message sent")
                }
                Err(_) => {
                    println!(" ❌ Could not execute command");
                }
            }
        }
        todo!()
    }
}
