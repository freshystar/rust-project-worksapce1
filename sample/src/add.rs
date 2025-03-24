use std::{fs::OpenOptions, io::{self, Write}, path::Path};

#[derive(Clone)]
pub struct Add;

impl Add {
    pub fn add_to_file(self, string: &str) -> io::Result<()> {
        let path = Path::new("rust.txt");
        let display = path.display();
        let file_content = String::new();
    
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("rust.txt")
            .expect("Cannot open file");
        let line_count = file_content.lines().count();
        writeln!(file, "{}: {}", line_count + 1, string)?;
        let input = format!("{}\n", string);
       // file.write_all(input.as_bytes())
           // .expect("Cannot write to file");

            
            println!("Successfully wrote into {}: {}", display, input);
            Ok(())
    }
}


 