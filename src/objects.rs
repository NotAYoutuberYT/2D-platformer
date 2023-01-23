#![allow(dead_code)]

use std::vec;

// basic vector2 struct
#[derive(Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
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
            &Vector2::multiply(&vector1, 1.0 - t),
            &Vector2::multiply(&vector2, t),
        )
    }

    // adds self to other
    pub fn add_to(&self, other: &mut Vector2) {
        other.set(&Vector2::add(&self, &other));
    }
}

// ensures f2 is beteen f1 and f3
// (f1 must be below f3 for a return of true)
fn sandwich(f1: f64, f2: f64, f3: f64) -> bool {
    f1 < f2 && f2 < f3
}

//
// RectObject trait code
//

/// A basic rectangular object.
/// Two of its sides must be vertical,
/// and the other two must be horizontal.
pub trait RectObject {
    /// returns the object's 4 points in clockwise
    /// order, starting at the top left
    fn points(&self) -> Vec<Vector2>;

    /// returns a vector containing the leftmost
    /// x value, rightmost x value, lowest y
    /// value, and uppermost y value respectively
    fn bounds(&self) -> (f64, f64, f64, f64);

    /// determines if a Vector2 lies within the object
    /// (being on an edge doesn't count as being inside)
    fn contains_point(&self, point: &Vector2) -> bool {
        let bounds: (f64, f64, f64, f64) = self.bounds();

        let mut inside: bool = true;

        // if we detect the point outside of the
        // box at any time, set inside to false
        if point.x < bounds.0 || bounds.1 < point.x {
            inside = false;
        } else if point.y < bounds.2 || bounds.3 < point.y {
            inside = false;
        }

        inside
    }

    /// Returns if it is possible to make
    /// a vertical line that passes through
    /// self and a passed in RectObject
    fn collides_with_y(&self, other: &dyn RectObject) -> bool {
        let self_bounds: (f64, f64, f64, f64) = self.bounds();
        let other_bounds: (f64, f64, f64, f64) = other.bounds();

        let mut collides: bool = false;

        if sandwich(other_bounds.0, self_bounds.0, other_bounds.1) {
            // is self's leftmost side in other?
            collides = true;
        } else if sandwich(other_bounds.0, self_bounds.1, other_bounds.1) {
            // is self's rightmost side in other?
            collides = true;
        } else if self_bounds.0 <= other_bounds.0 && other_bounds.1 < self_bounds.1 {
            // do we completely contatain other on x axis?
            collides = true;
        }

        collides
    }

    /// Returns if it is possible to make
    /// a horizontal line that passes through
    /// self and a passed in RectObject
    fn collides_with_x(&self, other: &dyn RectObject) -> bool {
        let self_bounds: (f64, f64, f64, f64) = self.bounds();
        let other_bounds: (f64, f64, f64, f64) = other.bounds();

        let mut collides: bool = false;

        if sandwich(other_bounds.2, self_bounds.2, other_bounds.3) {
            // is self's bottom side in other?
            collides = true;
        } else if sandwich(other_bounds.2, self_bounds.3, other_bounds.3) {
            // is self's top side in other?
            collides = true;
        } else if self_bounds.2 <= other_bounds.2 && other_bounds.3 < self_bounds.3 {
            // do we completely contatain other on y axis?
            collides = true;
        }

        collides
    }

    /// detects collision with the other object
    // /(colliding edges counts as collision)
    fn collides_with(&self, other: &dyn RectObject) -> bool {
        self.collides_with_x(other) && self.collides_with_y(other)
    }
}

/// determines if a Vector2 lies within an object's bounds
pub fn bounds_contain_point(point: &Vector2, bounds: &(f64, f64, f64, f64)) -> bool {
    // if we detect the point too far to the left, too far the right, too far down, or too high up, return false
    !(point.x < bounds.0 || bounds.1 < point.x || point.y < bounds.2 || bounds.3 < point.y)
}

//
// RigidBody code
//

// when handling rigidbody collisions, it is useful to know
// how the rigidbody collided with objects
// note: the name refer to the rigidbody's position
// relative to the object it collides with
#[derive(PartialEq, Eq)]
pub enum CollisionTypes {
    OnBottom,
    OnSide,
    OnTop,
}

#[derive(Clone)]
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
    handles the collisions with an array of rectobjects,
    puts the collision type into active_collision,
    and returns the index of the object the player was
    on, if any
    */
    pub fn handle_collisions<T: RectObject>(
        &mut self,
        objects: &[T],
        active_collision: &mut Vec<CollisionTypes>,
    ) -> Option<usize> {
        let self_bounds = self.bounds();
        let mut platform_on: Option<usize> = None; // this stores an index, not an object

        for object in objects.iter().enumerate() {
            // if the rigidbody doesn't collide with this object at all, move to the next object
            if !self.collides_with(object.1) {
                continue;
            }

            let obj_bounds = object.1.bounds();

            // determine the collision depth of each side of the object
            let right_depth: f64 = obj_bounds.1 - self_bounds.0;
            let left_depth: f64 = self_bounds.1 - obj_bounds.0;
            let top_depth: f64 = obj_bounds.3 - self_bounds.2;
            let bottom_depth: f64 = self_bounds.3 - obj_bounds.2;

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

            // move the player ouside of the platform
            match min_index {
                0 => self.center.x = obj_bounds.0 - (self.width / 2.0),
                1 => self.center.x = obj_bounds.1 + (self.width / 2.0),
                2 => self.center.y = obj_bounds.2 - (self.height / 2.0) - 1.0, // this stops the player from sticking to the object
                3 => self.center.y = obj_bounds.3 + (self.height / 2.0),
                _ => panic!("Error: closest to no side when handling rigidbody collisions"),
            }

            // finds what kind of collision it was
            active_collision.push(match min_index {
                0 => CollisionTypes::OnSide,
                1 => CollisionTypes::OnSide,
                2 => CollisionTypes::OnBottom,
                3 => CollisionTypes::OnTop,
                _ => panic!("Error: closest to no side when handling rigidbody collisions"),
            });

            // update the platform that the player is on if they're on top of one
            if let Some(collision_state) = active_collision.last() {
                if collision_state == &CollisionTypes::OnTop {
                    platform_on = Some(object.0);
                }
            }
        }

        platform_on
    }
}

impl RectObject for RigidBody {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut points: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.0 = self.center.x - half_width;
        points.1 = self.center.x + half_width;

        // push bottom y, top y
        let half_height: f64 = self.height / 2.0;
        points.2 = self.center.y - half_height;
        points.3 = self.center.y + half_height;

        points
    }
}

//
// MovingObject code
//

#[derive(Clone)]
/// a RectObject that moves between two fixed points
pub struct MovingObject {
    pub start_pos: Vector2,
    pub end_pos: Vector2,
    pub width: f64,
    pub height: f64,
    pub move_time: f64,
    pub fallthrough: bool,

    /// describes how far the object has traveled (0-1 is going to end_pos, 1-2 is returning to start_pos)
    amount_traveled: f64,
    pub center: Vector2,

    /// the motion on the object's last update
    pub prev_move: Vector2,
}

impl MovingObject {
    /// creates a new leaving moving platform at start point
    pub fn new(
        start_pos: Vector2,
        end_pos: Vector2,
        width: f64,
        height: f64,
        move_time: f64,
        fallthrough: bool,
    ) -> MovingObject {
        let center: Vector2 = start_pos.clone();

        MovingObject {
            start_pos: start_pos,
            end_pos: end_pos,
            width: width,
            height: height,
            move_time: move_time,
            fallthrough: fallthrough,

            amount_traveled: 0.0,
            center: center,

            prev_move: Vector2::new(0.0, 0.0),
        }
    }

    /**
     * moves the object across its path and stores the new position
     * will automatically change direction
     * will panic on negative ammount values
     * returns the movement the object took
     */
    pub fn update(&mut self, ammount: f64) {
        let pre_center: Vector2 = Vector2::clone(&self.center);

        // trying to move a moving platform backwards isn't unrecoverable
        // in itself, but it almost certainly means that something
        // somewhere else has gone completely wrong
        if ammount < 0.0 {
            panic!("attempted to move moving platform negative ammount");
        }

        self.amount_traveled += ammount / self.move_time;

        // this prevents overflow
        self.amount_traveled %= 2.0;

        // configures the lerp ammount
        let lerp_ammount: f64;
        if self.amount_traveled < 1.0 {
            lerp_ammount = self.amount_traveled;
        } else {
            lerp_ammount = 2.0 - self.amount_traveled;
        }

        // lerp between the two points to determine the center of the moving platform
        self.center
            .set(&Vector2::lerp(&self.start_pos, &self.end_pos, lerp_ammount));

        // return the moved ammount by subtracting previous position from new position
        self.prev_move = Vector2::add(&Vector2::multiply(&pre_center, -1.0), &self.center)
    }
}

impl RectObject for MovingObject {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut points: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.0 = self.center.x - half_width;
        points.1 = self.center.x + half_width;

        // push bottom y, top y
        let half_height: f64 = self.height / 2.0;
        points.2 = self.center.y - half_height;
        points.3 = self.center.y + half_height;

        points
    }
}

//
// StaticObject code
//

#[derive(Clone)]
pub struct StaticObject {
    pub center: Vector2,
    pub width: f64,
    pub height: f64,
}

impl RectObject for StaticObject {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut points: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.0 = self.center.x - half_width;
        points.1 = self.center.x + half_width;

        // push bottom y, top y
        let half_height: f64 = self.height / 2.0;
        points.2 = self.center.y - half_height;
        points.3 = self.center.y + half_height;

        points
    }
}

//
// Circle code
//

// a circle with no collisions used to indicate sutff
pub struct Circle {
    center: Vector2,
    radius_squared: f64,
}

impl Circle {
    pub fn new(center: &Vector2, radius: f64) -> Circle {
        Circle {
            center: center.clone(),
            radius_squared: radius,
        }
    }

    pub fn contains_point(&self, point: &Vector2) -> bool {
        let vector_from_center = Vector2::new(point.x - self.center.x, point.y - self.center.y);
        let distance_from_center_squared = vector_from_center.x * vector_from_center.x
            + vector_from_center.y * vector_from_center.y;

        distance_from_center_squared < self.radius_squared
    }
}
