//
// config constants
//

use crate::camera::Rgb;

// colors :)
pub const PLAYER_COLOR: Rgb = Rgb::from_u32(0xf00000);
pub const STATIC_OBJECT_COLOR: Rgb = Rgb::from_u32(0xff);
pub const MOVING_OBJECT_COLOR: Rgb = Rgb::from_u32(0x6cc06);
pub const MOVING_PLATFORM_INDICATOR_COLOR: Rgb = Rgb::from_u32(0xeeeeee);
pub const CHECKPOINT_COLOR: Rgb = Rgb::from_u32(0xff00);
pub const GOAL_COLOR: Rgb = Rgb::from_u32(0xf6f70b);
pub const BACKGROUND_COLOR: Rgb = Rgb::from_u32(0x200020);
pub const VOID_COLOR: Rgb = Rgb::from_u32(0x100010);

// sizes
pub const MOVING_PLATFORM_INDICATOR_RADIUS: f64 = 5.0;
pub const VOID_TRANSITION_SIZE: f64 = 60.0;

pub const PLAYER_WIDTH: f64 = 20.0;
pub const PLAYER_HEIGHT: f64 = 40.0;

// window stuff
pub const WINDOW_WIDTH: usize = 260 * 4;
pub const WINDOW_HEIGHT: usize = 260 * 3;
pub const FPS: f64 = 144.0;

// player stuff
pub const PLAYER_WALKING_ACCEL: f64 = 2.4;
pub const PLAYER_AIR_ACCELERATION_RATIO: f64 = 0.05;
pub const COYOTE_TIME_HUNDREDTH_SECONDS: f64 = 8.0;
pub const STUCK_PLATFORM_VELOCITY_ADD_MODIFIER: f64 = 0.6;

// jump stuff
pub const JUMP_FORCE: f64 = 5.0;
pub const JUMP_BUFFER_HUNDREDTH_SECONDS: f64 = 1.0;

// physics stuff
pub const FRICTION_GROUND: f64 = 0.7;
pub const FRICTION_AIR: f64 = 0.04;

// camera stuff
pub const PERCENT_SCREEN_PLAYER_ALLOWED_IN_X: f64 = 18.0;
pub const PERCENT_SCREEN_PLAYER_ALLOWED_IN_Y: f64 = 17.5;
pub const PLAYER_FOCUS_X_OFFSET: f64 = 0.0;
pub const PLAYER_FOCUS_Y_OFFSET: f64 = -230.0;
pub const CAMERA_MOVING_EASING_X: f64 = 1.0 / 750.0;
pub const CAMERA_MOVING_EASING_Y: f64 = 1.0 / 1300.0;

// gravity
pub const GRAVITY_MOVING_UP: f64 = -1.0 / 7.8;
pub const GRAVITY_MOVING_DOWN: f64 = -1.0 / 4.5;
pub const VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT: f64 = -1.0 / 2.25;

//
// don't touch these constants
//

pub const MIN_X_FROM_CAMERA_BOTTOM_LEFT: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_ALLOWED_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
pub const MAX_X_FROM_CAMERA_BOTTOM_LEFT: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_ALLOWED_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
pub const MIN_Y_FROM_CAMERA_BOTTOM_LEFT: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_ALLOWED_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;
pub const MAX_Y_FROM_CAMERA_BOTTOM_LEFT: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_ALLOWED_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;

pub const FRAME_LIMIT_MILLIS: u64 = (1000.0 / FPS) as u64;
