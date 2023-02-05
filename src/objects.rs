use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    vec,
};

use crate::camera::Rgb;

// basic vector2 struct
#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    // makes set equal to passed vector
    pub fn set(&mut self, other: &Vector2) {
        self.x = other.x;
        self.y = other.y;
    }

    // returns the sum of self and other
    pub fn add(vector1: &Vector2, vector2: &Vector2) -> Vector2 {
        Vector2::new(vector1.x + vector2.x, vector1.y + vector2.y)
    }

    pub fn multiply(vector: &Vector2, scalar: f64) -> Vector2 {
        Vector2::new(vector.x * scalar, vector.y * scalar)
    }

    // linearly interpolates the two vectors
    // t should be between 0 and one
    pub fn lerp(vector1: &Vector2, vector2: &Vector2, t: f64) -> Vector2 {
        Vector2::add(
            &Vector2::multiply(vector1, 1.0 - t),
            &Vector2::multiply(vector2, t),
        )
    }

    // adds self to other
    pub fn add_to(&self, other: &mut Vector2) {
        other.set(&Vector2::add(self, other));
    }
}

/// checks if f1 is between f1 and f3
/// (f1 must be below f3 for a return of true)
fn sandwich(f1: f64, f2: f64, f3: f64) -> bool {
    f1 < f2 && f2 < f3
}

/// a struct for holding the bounds of an object
/// teach of the values represents the distance
/// of that side from the center of the object
pub struct Bounds {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl Bounds {
    /// determines if a Vector2 lies within an object's bounds
    pub fn contains_point(&self, point: &Vector2) -> bool {
        // if we detect the point too far to the left, too far the right, too far down, or too high up, return false
        !(point.x < self.left
            || self.right < point.x
            || point.y < self.bottom
            || self.top < point.y)
    }
}

//
// RectObject trait code
//

/// A basic rectangular object.
/// Two of its sides must be vertical,
/// and the other two must be horizontal.
pub trait RectObject {
    /// returns the center of the object
    fn center(&self) -> Vector2;

    /// returns the width of the object
    fn width(&self) -> f64;

    /// returns the height of the object
    fn height(&self) -> f64;

    /// returns the object's 4 points in clockwise
    /// order, starting at the top left
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width() / 2.0;
        let half_height: f64 = self.height() / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center().x - half_width,
            self.center().y + half_height,
        ));
        points.push(Vector2::new(
            self.center().x + half_width,
            self.center().y + half_height,
        ));
        points.push(Vector2::new(
            self.center().x + half_width,
            self.center().y - half_height,
        ));
        points.push(Vector2::new(
            self.center().x - half_width,
            self.center().y - half_height,
        ));

        points
    }

    /// returns a vector containing the leftmost
    /// x value, rightmost x value, lowest y
    /// value, and uppermost y value respectively
    fn bounds(&self) -> Bounds {
        let half_width: f64 = self.width() / 2.0;
        let half_height: f64 = self.height() / 2.0;

        Bounds {
            left: self.center().x - half_width,
            right: self.center().x + half_width,

            bottom: self.center().y - half_height,
            top: self.center().y + half_height,
        }
    }

    /// determines if a Vector2 lies within the object
    /// (being on an edge doesn't count as being inside)
    fn contains_point(&self, point: &Vector2) -> bool {
        let bounds = self.bounds();

        // if we detect the point outside of the
        // box at any time, set inside to false
        let outside_x = point.x < bounds.left || bounds.right < point.x;
        let outside_y = point.y < bounds.bottom || bounds.top < point.y;

        !(outside_x || outside_y)
    }

    /// Returns if it is possible to make
    /// a vertical line that passes through
    /// self and a passed in RectObject
    fn collides_with_y(&self, other: &dyn RectObject) -> bool {
        let self_bounds = self.bounds();
        let other_bounds = other.bounds();

        let self_left_in_other = sandwich(other_bounds.left, self_bounds.left, other_bounds.right);
        let self_right_in_other =
            sandwich(other_bounds.left, self_bounds.right, other_bounds.right);
        let self_contain_other =
            self_bounds.left < other_bounds.left && other_bounds.right < self_bounds.right;

        self_left_in_other || self_right_in_other || self_contain_other
    }

    /// Returns if it is possible to make
    /// a horizontal line that passes through
    /// self and a passed in RectObject
    fn collides_with_x(&self, other: &dyn RectObject) -> bool {
        let self_bounds = self.bounds();
        let other_bounds = other.bounds();

        let self_bottom_in_other =
            sandwich(other_bounds.bottom, self_bounds.bottom, other_bounds.top);
        let self_top_in_other = sandwich(other_bounds.bottom, self_bounds.top, other_bounds.top);
        let self_contain_other =
            self_bounds.bottom <= other_bounds.bottom && other_bounds.top < self_bounds.top;

        self_bottom_in_other || self_top_in_other || self_contain_other
    }

    /// detects collision with the other object
    // /(colliding edges counts as collision)
    fn collides_with(&self, other: &dyn RectObject) -> bool {
        self.collides_with_x(other) && self.collides_with_y(other)
    }
}

//
// RigidBody code
//

// when handling rigidbody collisions, it is useful to know
// how the rigidbody collided with objects
// note: the name refer to the rigidbody's position
// relative to the object it collides with
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CollisionTypes {
    Bottom,
    Top,
    Left,
    Right,
}

/// struct to represent an object with physics
/// * movement must be handled manually
/// * collision functions are provided
#[derive(Clone, Copy)]
pub struct RigidBody {
    pub center: Vector2,
    pub width: f64,
    pub height: f64,

    pub velocity: Vector2,
}

impl RigidBody {
    /// creates a new rigidbody
    pub fn new() -> RigidBody {
        RigidBody {
            center: Vector2::new(0.0, 0.0),
            width: 10.0,
            height: 10.0,
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    // movement with no physics
    pub fn move_by(&mut self, movement: &Vector2) {
        movement.add_to(&mut self.center);
    }

    /**
    handles the collisions with an array of rect objects,
    puts the collision type into active_collision,
    and returns the index of the object the player was
    on, if any
    */
    pub fn handle_collisions<T: RectObject + std::marker::Sync>(
        &mut self,
        objects: &[T],
        active_collisions: &mut Vec<CollisionTypes>,
    ) -> Option<usize> {
        let mut handles: Vec<JoinHandle<CollisionTypes>> = Vec::new();

        let platform_on: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
        let self_tracker = Arc::new(Mutex::new(*self));

        for (index, object) in objects.iter().enumerate() {
            // if the rigidbody doesn't collide with this object at all, move to the next object
            if !self.collides_with(object) {
                continue;
            }

            let self_bounds = self.bounds();
            let obj_bounds = object.bounds();

            let self_ptr = Arc::clone(&self_tracker);
            let platform_on_ptr = Arc::clone(&platform_on);

            handles.push(thread::spawn(move || {
                // determine the collision depth of each side of the object
                let right_depth: f64 = obj_bounds.right - self_bounds.left;
                let left_depth: f64 = self_bounds.right - obj_bounds.left;
                let top_depth: f64 = obj_bounds.top - self_bounds.bottom;
                let bottom_depth: f64 = self_bounds.top - obj_bounds.bottom;

                // creates an iterator of an enumeration of the depths
                let depths = [left_depth, right_depth, bottom_depth, top_depth];
                let iter = depths.iter().enumerate();

                // and findes the entry with the minimum value (I can unwrap because there is a 0% chance of finding a None)
                let min_index = iter
                    .reduce(|acc, item| match acc.1 < item.1 {
                        true => acc,
                        false => item,
                    })
                    .unwrap()
                    .0;

                // move the player outside of the platform
                let mut guard = self_ptr.lock().unwrap();
                match min_index {
                    0 => guard.center.x = obj_bounds.left - (guard.width / 2.0),
                    1 => guard.center.x = obj_bounds.right + (guard.width / 2.0),
                    2 => guard.center.y = obj_bounds.bottom - (guard.height / 2.0) - 1.0, // -1.0 stops physics bugs
                    3 => guard.center.y = obj_bounds.top + (guard.height / 2.0),

                    _ => panic!("Error: closest to no side handling rigidbody collisions"),
                }

                std::mem::drop(guard);

                // finds what kind of collision it was
                let current_collision = match min_index {
                    0 => CollisionTypes::Left,
                    1 => CollisionTypes::Right,
                    2 => CollisionTypes::Bottom,
                    3 => CollisionTypes::Top,
                    _ => panic!("Error: closest to no side when handling rigidbody collisions"),
                };

                // update the platform that the player is on if they're on top of one
                if current_collision == CollisionTypes::Top {
                    *platform_on_ptr.lock().unwrap() = Some(index);
                }

                current_collision
            }));
        }

        // we unwrap because collision handling shouldn't panic
        for handle in handles {
            active_collisions.push(handle.join().unwrap());
        }

        let final_self = *self_tracker.lock().unwrap();
        *self = final_self;

        let index = *platform_on.lock().unwrap();
        index
    }
}

impl RectObject for RigidBody {
    fn center(&self) -> Vector2 {
        self.center
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }
}

//
// MovingObject code
//

#[derive(Clone)]
/// a RectObject that moves between two fixed points
pub struct MovingObject {
    start_pos: Vector2,
    end_pos: Vector2,

    width: f64,
    height: f64,
    move_time: f64,

    /// describes how far the object has traveled (0-1 is going to end_pos, 1-2 is returning to start_pos)
    amount_traveled: f64,
    center: Vector2,

    /// the motion on the object's last update
    prev_move: Vector2,
}

impl MovingObject {
    /// returns the object's start position
    pub fn start_pos(&self) -> Vector2 {
        self.start_pos
    }

    /// returns the object's end position
    pub fn end_pos(&self) -> Vector2 {
        self.end_pos
    }

    /// returns the object's previous move
    pub fn prev_move(&self) -> Vector2 {
        self.prev_move
    }

    /// creates a new leaving moving platform at start point
    pub fn new(
        start_pos: Vector2,
        end_pos: Vector2,
        width: f64,
        height: f64,
        move_time: f64,
    ) -> MovingObject {
        let center: Vector2 = start_pos;

        MovingObject {
            start_pos,
            end_pos,
            width,
            height,
            move_time,

            amount_traveled: 0.0,
            center,

            prev_move: Vector2::new(0.0, 0.0),
        }
    }

    /**
     * moves the object across its path and stores the new position
     * will automatically change direction
     * will panic on negative amount values
     * returns the movement the object took
     */
    pub fn update(&mut self, amount: f64) {
        let pre_center: Vector2 = Vector2::clone(&self.center);

        // trying to move a moving platform backwards isn't unrecoverable
        // in itself, but it almost certainly means that something
        // somewhere else has gone completely wrong
        if amount < 0.0 {
            panic!("attempted to move moving platform negative ammount");
        }

        self.amount_traveled += amount / self.move_time;

        // this prevents overflow
        self.amount_traveled %= 2.0;

        // configures the lerp amount
        let lerp_amount: f64 = match self.amount_traveled < 1.0 {
            true => self.amount_traveled,
            false => 2.0 - self.amount_traveled,
        };

        // lerp between the two points to determine the center of the moving platform
        self.center
            .set(&Vector2::lerp(&self.start_pos, &self.end_pos, lerp_amount));

        // return the moved amount by subtracting previous position from new position
        self.prev_move = Vector2::add(&Vector2::multiply(&pre_center, -1.0), &self.center)
    }
}

impl RectObject for MovingObject {
    fn center(&self) -> Vector2 {
        self.center
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }
}

//
// StaticObject code
//

#[derive(Clone)]
/// a pure implementation of rect object
pub struct StaticObject {
    center: Vector2,
    width: f64,
    height: f64,
}

impl StaticObject {
    pub fn new(center: Vector2, width: f64, height: f64) -> StaticObject {
        StaticObject {
            center,
            width,
            height,
        }
    }
}

impl RectObject for StaticObject {
    fn center(&self) -> Vector2 {
        self.center
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }
}

//
// Circle code
//

/// a circle with no collisions used to indicate different things
pub struct Circle {
    center: Vector2,
    radius: f64,
    pub color: Rgb,
}

impl Circle {
    pub fn new(center: &Vector2, radius: f64, color: Rgb) -> Circle {
        Circle {
            center: *center,
            radius,
            color,
        }
    }

    pub fn contains_point(&self, point: &Vector2) -> bool {
        let vector_from_center = Vector2::new(point.x - self.center.x, point.y - self.center.y);
        let distance_from_center_squared = vector_from_center.x * vector_from_center.x
            + vector_from_center.y * vector_from_center.y;

        distance_from_center_squared < self.radius * self.radius
    }

    pub fn intersects_rigidbody(&self, rigidbody: &RigidBody) -> bool {
        let distance = Vector2::new(
            (self.center.x - rigidbody.center.x).abs(),
            (self.center.y - rigidbody.center.y).abs(),
        );

        if distance.x > rigidbody.width / 2.0 + self.radius {
            return false;
        }
        if distance.y > rigidbody.height / 2.0 + self.radius {
            return false;
        }

        if distance.x <= rigidbody.width / 2.0 {
            return true;
        }
        if distance.y <= rigidbody.height / 2.0 {
            return true;
        }

        let corner_distance_squared = (distance.x - rigidbody.width / 2.0)
            * (distance.x - rigidbody.width / 2.0)
            + (distance.y - rigidbody.height / 2.0) * (distance.y - rigidbody.height / 2.0);

        corner_distance_squared <= self.radius * self.radius
    }
}
