use std::env;

use crate::{add::Add, list::List, remove::Remove, update::Update};

pub struct Application;

impl Application {
    pub fn run() {
        let args: Vec<String> = env::args().collect();

        // if args.len() < 3 || args[3] != "task" {
        //     println!("Usage:");
        //     println!("Enter: 'task' 'add' <data>, if you want to add");
        //     println!("Enter: 'task' 'remove' <index_of_data>, to remove");
        //     println!("Enter: 'task' 'update' <index_of_data>, <data>, to update");
        //     println!("Enter: 'task' 'list', to list data in file. ")
        // }
        let var = Add;

        if args[1] == "task" {
            if args[2] == "add" {
                if args.len() > 4 || args.len() < 3 {
                    println!("Enter the right number of arguments: '3'");
                } else {
                    let _ = var.add_to_file(&args[3]);
                }
            } else if args[2] == "list" {
                if args.len() > 3 {
                    println!("Enter the right number of arguments: '2'");
                } else {
                    List::list_file();
                }
            } else if args[2] == "remove" {
                if args.len() > 4 {
                    println!("Enter the right number of arguments: '3'");
                } else {
                    let var = Remove;
                    var.remove_data(&args[3])
                }
            } else if args[2] == "update" {
                if args.len() > 5 {
                    println!("Enter the right number of arguments: '3'");
                } else {
                    let var = Update;
                    var.replace(&args[3], &args[4])
                }
            } else {
                println!("Enter : 'add', 'list', 'remove', 'update' ")
            }
        } else {
            println!("Enter: 'task'...followed by either: 'add', 'list', 'remove' 'update' ");
        }
    }
}
