use application::Application;


fn main() {
    Application::run();

}

// println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

mod add;
mod list;
mod remove;
mod update;
mod application;
