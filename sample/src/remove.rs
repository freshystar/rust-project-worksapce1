use std::process::Command;

use crate::list::List;

pub struct Remove;

impl Remove {
    pub fn remove_data(self, line_id: &str) {
        List::list_file();
        let _cmd = Command::new("sed")
            .arg("-i")
            .arg(format!("{}d", line_id))
            .arg("rust.txt")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        println!("------------Successfully removed item {}--------------", line_id);
        List::list_file();
    }
}
