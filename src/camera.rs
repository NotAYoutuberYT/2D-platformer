use crate::map_loader::Map;

use super::{
    constants::{
        CAMERA_MOVING_EASING_X, CAMERA_MOVING_EASING_Y, MAX_X_FROM_CAMERA_BOTTOM_LEFT,
        MAX_Y_FROM_CAMERA_BOTTOM_LEFT, MIN_X_FROM_CAMERA_BOTTOM_LEFT,
        MIN_Y_FROM_CAMERA_BOTTOM_LEFT,
    },
    objects::{RigidBody, Vector2},
};

pub struct Camera {
    pub bottom_left: Vector2,
}

impl Camera {
    /// creates a new camera with the x and why coordinates
    /// of the bottom left corner of the rendering area
    pub fn new(x: f64, y: f64) -> Camera {
        Camera {
            bottom_left: Vector2::new(x, y),
        }
    }

    /// returns the actual point in the game
    /// that world the pixel point represents
    pub fn get_game_position(&self, point: Vector2) -> Vector2 {
        Vector2::new(
            self.bottom_left.x + point.x as f64,
            self.bottom_left.y + (super::constants::WINDOW_HEIGHT - point.y as usize) as f64,
        )
    }

    /// keeps the camera centered on a player
    pub fn keep_centered_on_player(&mut self, player: &mut RigidBody, frame_time: f64) {
        if player.center.x - self.bottom_left.x < MIN_X_FROM_CAMERA_BOTTOM_LEFT {
            self.bottom_left.x -=
                (player.center.x - self.bottom_left.x - MIN_X_FROM_CAMERA_BOTTOM_LEFT)
                    * (player.center.x - self.bottom_left.x - MIN_X_FROM_CAMERA_BOTTOM_LEFT)
                    * frame_time
                    * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - self.bottom_left.x > MIN_X_FROM_CAMERA_BOTTOM_LEFT {
                self.bottom_left.x = player.center.x - MIN_X_FROM_CAMERA_BOTTOM_LEFT;
            }
        } else if player.center.x - self.bottom_left.x > MAX_X_FROM_CAMERA_BOTTOM_LEFT {
            self.bottom_left.x +=
                (player.center.x - self.bottom_left.x - MAX_X_FROM_CAMERA_BOTTOM_LEFT)
                    * (player.center.x - self.bottom_left.x - MAX_X_FROM_CAMERA_BOTTOM_LEFT)
                    * frame_time
                    * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - self.bottom_left.x < MAX_X_FROM_CAMERA_BOTTOM_LEFT {
                self.bottom_left.x = player.center.x - MAX_X_FROM_CAMERA_BOTTOM_LEFT;
            }
        }

        if player.center.y - self.bottom_left.y < MIN_Y_FROM_CAMERA_BOTTOM_LEFT {
            self.bottom_left.y -=
                (player.center.y - self.bottom_left.y - MIN_Y_FROM_CAMERA_BOTTOM_LEFT)
                    * (player.center.y - self.bottom_left.y - MIN_Y_FROM_CAMERA_BOTTOM_LEFT)
                    * frame_time
                    * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - self.bottom_left.y > MIN_Y_FROM_CAMERA_BOTTOM_LEFT {
                self.bottom_left.y = player.center.y - MIN_Y_FROM_CAMERA_BOTTOM_LEFT;
            }
        } else if player.center.y - self.bottom_left.y > MAX_Y_FROM_CAMERA_BOTTOM_LEFT {
            self.bottom_left.y +=
                (player.center.y - self.bottom_left.y - MAX_Y_FROM_CAMERA_BOTTOM_LEFT)
                    * (player.center.y - self.bottom_left.y - MAX_Y_FROM_CAMERA_BOTTOM_LEFT)
                    * frame_time
                    * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - self.bottom_left.y < MAX_Y_FROM_CAMERA_BOTTOM_LEFT {
                self.bottom_left.y = player.center.y - MAX_Y_FROM_CAMERA_BOTTOM_LEFT;
            }
        }
    }

    // renders the camera, using the inputted function to convert
    // pixels in the gamespace into rgb values

    // the function should take the following inputs:
    // the game point, the player bounds, if the player is sprinting,
    // the static object bounds, the moving object bounds
    pub fn render_frame(
        &self,
        render: &dyn Fn(
            Vector2, // the point in space to render
            &Map,    // the map to render
        ) -> u32,
        map: &Map,
        buffer: &mut Vec<u32>,
    ) {
        for x in 0..super::WINDOW_WIDTH {
            for y in 0..super::WINDOW_HEIGHT {
                // the coordinate in the world that this pixel is
                let world_point = self.get_game_position(Vector2::new(x as f64, y as f64));

                buffer[y * super::WINDOW_WIDTH + x] = render(world_point, &map);
            }
        }
    }
}
