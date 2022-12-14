// basic point struct
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }

    // makes set equal to passed
    // in point
    fn set(&mut self, other: &Point) {
        self.x = other.x;
        self.y = other.y;
    }

    // returns the sum of self and other
    fn add(point1: &Point, point2: &Point) -> Point {
        Point::new(point1.x + point2.x, point1.y + point2.y)
    }

    // adds self to other
    fn add_to(&self, other: &mut Point) {
        other.set(&Point::add(&self, &other));
    }
}

// ensures f2 is beteen f1 and f3
// (f1 must be below f3)
fn sandwitch(f1: f64, f2: f64, f3: f64) -> bool {
    f1 <= f2 && f2 <= f3
}

// a rectangle object that has two
// flat and two vertical lines
pub trait RectObject {
    // returns the object's 4 points
    fn points(&self) -> Vec<Point>;

    // returns a vector containing the leftmost
    // x value, rightmost x value, lowest y
    // value, and uppermost y value respectively
    fn bounds(&self) -> Vec<f64>;

    // detects if the there is collion on
    // the horizontal axis
    fn collides_with_y(&self, other: &dyn RectObject) -> bool {
        let self_bounds: Vec<f64> = self.bounds();
        let other_bounds: Vec<f64> = other.bounds();

        let mut collides: bool = false;

        if sandwitch(other_bounds[0], self_bounds[0], other_bounds[1]) {
            // is self's leftmost side in other?
            collides = true;
        } else if sandwitch(other_bounds[0], self_bounds[1], other_bounds[1]) {
            // is self's rightmost side in other?
            collides = true;
        } else if self_bounds[0] <= other_bounds[0] && other_bounds[1] < self_bounds[1] {
            // do we completely contatain other on x axis?
            collides = true;
        }

        collides
    }

    // detects if the there is collion on
    // the vertical axis
    fn collides_with_x(&self, other: &dyn RectObject) -> bool {
        let self_bounds: Vec<f64> = self.bounds();
        let other_bounds: Vec<f64> = other.bounds();

        let mut collides: bool = false;

        if sandwitch(other_bounds[2], self_bounds[2], other_bounds[3]) {
            // is self's bottom side in other?
            collides = true;
        } else if sandwitch(other_bounds[2], self_bounds[3], other_bounds[3]) {
            // is self's top side in other?
            collides = true;
        } else if self_bounds[2] <= other_bounds[2] && other_bounds[3] < self_bounds[3] {
            // do we completely contatain other on y axis?
            collides = true;
        }

        collides
    }

    // detects collision with the other object
    // (colliding edges counts as collision)
    fn collides_with(&self, other: &dyn RectObject) -> bool {
        self.collides_with_x(other) && self.collides_with_y(other)
    }
}

pub struct RigidBody {
    pub center: Point,
    pub width: f64,
    pub height: f64,
    pub density: f64,
}

impl RigidBody {
    fn move_by(&mut self, movement: &Point) {
        movement.add_to(&mut self.center);
    }
}

impl RectObject for RigidBody {
    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Point::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Point::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Point::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Point::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> Vec<f64> {
        let mut points: Vec<f64> = vec![];

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.push(self.center.x - half_width);
        points.push(self.center.x + half_width);

        // push bottom y, top y
        let half_height: f64 = self.width / 2.0;
        points.push(self.center.y - half_height);
        points.push(self.center.y + half_height);

        points
    }
}
