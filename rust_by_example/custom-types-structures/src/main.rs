// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

struct Number

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right corners are in space.
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> (f32, f32, f32) {
  let Rectangle { top_left, bottom_right } = rectangle;
  let width: f32 = (bottom_right.x - top_left.x).abs();
  let height: f32 = (bottom_right.y - top_left.y).abs();

  return (width, height, width * height);
}

fn square(top_left: Point, side: f32) -> Rectangle {
  let bottom_right = Point { x: (top_left.x + side), y: (top_left.y - side) };
  let rect = Rectangle { top_left, bottom_right };
  return rect;
}

fn main() {
  // Create struct with field init shorthand
  let name = String::from("Peter");
  let age = 27;
  let peter = Person { name, age };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point = Point { x: 5.2, y: 0.4 };
  let another_point = Point { x: 10.3, y: 0.2 };

  // Access the fields of the point
  println!("Point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our other one
  let bottom_right = Point { x: 10.3, ..another_point };

  // `bottom_right.y` will be the same as `another_point.y` because we used the that from
  // `another_point`
  println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using a `let` binding
  let Point { x: left_edge, y: top_edge } = point;

  let rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right,
  };
  let (rect_width, rect_height, rect_area_abs) = rect_area(&rectangle);
  println!(
    "Rectangle Dimensions: Length: {} | Width: {}  | Area: {}",
    rect_width, rect_height, rect_area_abs
  );

  let square = square(Point { x: 2.3, y: 3.6 }, 2.0);
  let (sq_width, sq_height, sq_area_abs) = rect_area(&square);
  println!(
    "Square Dimensions: Length: {} | Width: {}  | Area: {}",
    sq_width, sq_height, sq_area_abs
  );

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("Pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("Pair contains {:?} and {:?}", integer, decimal);
}
