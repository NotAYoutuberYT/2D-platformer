/* This project uses a library (known as a crate in Rust) called minifb. It is
owned by Daniel Collin. It can be found on crates.io at https://crates.io/crates/minifb
or on GitHub at https://github.com/emoon/rust_minifb. To use in a project, put the line
of code "minifb = "0.23" into the cargo.toml file of a cargo-initialized project. This
crate is what allows me to open a window and write rgb values to each pixel of the window.
The exact things I use from the crate will either be in a use statement (see below) or
be explicitly prefaced with the library name (e.g. minifb::Window) */
extern crate minifb;
use minifb::{Window, WindowOptions};

//
// my modules
//

mod objects;

mod camera;
use constants::{FRAME_LIMIT_MILLIS, WINDOW_HEIGHT, WINDOW_WIDTH};

mod map;
use map::Map;

mod constants;

mod game_player;
use game_player::play_game;

//
// main
//

fn main() {
    let mut map: Map = Map::new();
    let mut current_level = 1;
    map.load_map(current_level);

    // our window :)
    let mut window = Window::new(
        "Platformer - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|error| {
        panic!("Error opening window: {}", error);
    });

    // configure the window
    window.limit_update_rate(Some(std::time::Duration::from_millis(FRAME_LIMIT_MILLIS)));
    window.set_position(20, 20);

    while !play_game(&mut map, &mut window) && current_level != 7 {
        current_level += 1;
        map.load_map(current_level);
    }
}
