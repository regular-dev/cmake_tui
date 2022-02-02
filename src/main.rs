mod util;
use std::error::Error;

use clap;

mod app;
mod parser;
mod tui_drawer;

use app::App;
use parser::Parser;
use tui_drawer::Tui;

fn main() -> Result<(), Box<dyn Error>> {
    // Args initialization
    let clap_app = clap::App::new("cmakeTui")
        .arg(clap::Arg::with_name("cmake_folder").index(1).required(true))
        .version("1.0")
        .about("Configure cmake parameters via TUI")
        .author("regular-dev.org")
        .get_matches();

    let folder = clap_app.value_of("cmake_folder").unwrap();

    println!("CMake project folder is {}", folder);
    println!("Launching...");

    // --- Parse cmake --- //

    let mut parser = Parser::new(folder);
    parser.parse_folder();

    // --- TUI --- //

    let mut tui = Tui::new();
    let mut app = App::new(&parser);

    loop {
        if let Err(_x) = tui.render(&mut app) {
            break;
        }

        if let Err(_x) = tui.event(&mut app) {
            break;
        }
    }

    Ok(())
}
