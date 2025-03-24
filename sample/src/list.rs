use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub struct List;

impl List {
    pub fn list_file() {
        if let Ok(lines) = read_lines("./rust.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}