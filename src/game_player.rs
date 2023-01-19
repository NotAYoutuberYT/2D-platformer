use super::map_loader::Map;

use minifb::Window;

use super::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

/// plays a game with a supplied map and window
pub fn play_game(map: &mut Map, window: &mut Window) {
    // this will be where we write out pixel values
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
}
