use std::{env, process};

fn usage() {
    println!("Usage: ./main <plugin_path>");
}

fn main() {
    let mut args = env::args();
    let path = match args.nth(1) {
        Some(path) => path,
        None => {
            usage();
            process::exit(1);
        }
    };

    env_logger::init();
    if let Err(e) = plugin_system::run_plugin(&path) {
        log::error!("Error: {}", e);
        process::exit(1);
    }
}
