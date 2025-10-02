#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

//Structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    let peter = Person { name, age };

    //Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    //Make a new point by using struct update syntax to use the fields of our
    // other one

    let bottom_right = Point { x: 10.3, ..another_point };

    //`bottom_right.y` will be the same as `another_point.y` because we used that
    // from `another_point`

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
}
