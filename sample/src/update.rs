use std::process::Command;

use crate::list::List;

pub struct Update;

impl Update {
    pub fn replace(self, line_id: &str, content: &str) {
        let _cmd = Command::new("sed")
            .arg("-i")
            .arg(format!("{}c\\{}", line_id, content))
            .arg(content)
            .arg("rust.txt")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        println!("-----------Successfully replaced item {} with {}--------------", line_id,content);
        List::list_file();
    }
}
