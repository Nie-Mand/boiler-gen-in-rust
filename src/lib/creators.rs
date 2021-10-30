use std::env::set_current_dir;
use std::fs::create_dir_all;
use std::process::Command;

// Helper Functions

pub fn create_express_boilerplate() {
    init_node_project();
}

pub fn create_react_boilerplate() {
    println!("Create a react boilerplate");
}

pub fn create_react_ts_boilerplate() {
    println!("Create a react as ts boilerplate");
}

pub fn place_me_at(location: &str) {
    if location != "." {
        create_dir_all(location).expect("error");
        set_current_dir(location).ok();
    }
}

pub fn init_node_project() {
    Command::new("npm").arg("init").arg("-y").output().ok();
}
