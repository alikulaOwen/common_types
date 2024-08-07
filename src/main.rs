#![allow(dead_code)]


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//unit
struct Unit;

//Tuple struct

struct Pair(i32, f32);

//struct with two fields
struct Point {
    x: f32,
    y: f32,
}

//struct with two fields
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


// A function which takes a custom Type `name, age` as an argument and returns struct `Person`
fn create_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
    }
}

fn rect_area(rectangle: Rectangle) -> f32 {
    // Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;

    let length = x2 - x1;
    let width = y2 - y1;

    length * width
}

fn square(point: &Point, size: f32) -> Rectangle {
      Rectangle {
        top_left: Point { x: point.x, y: point.y },
        bottom_right: Point {
            x: point.x + size,
            y: point.y + size,
        },
    }
}

fn main() {
    println!("Hello, world!");

    let owen = create_person(String::from("Owen"), 25);

    println!("{:?}", owen);
    println!("{} is {} years old", owen.name, owen.age);

    // Instantiate a Point

    let point: Point = Point { x: 10.3, y: 0.4 };
    

    println!("Point coordinates: x: {}, y: {}", point.x, point.y);

    let another_point = Point { x: 5.2, ..point };

    let bottom_right = Point { x: 5.2, ..another_point };

    println!("Bottom right coordinates: x: {}, y: {}", bottom_right.x, bottom_right.y);


    // Destructure the point using a `let` binding

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: my_x, y: my_y },
        bottom_right: bottom_right,
    };
    let area = rect_area(_rectangle);

    println!("Area of rectangle is: {}", area);

    let point = Point { x: 0.0, y: 0.0 };
    let size = 5.0;
    let rect = square(&point, size);

    println!("Rectangle Top Left: ({}, {})", rect.top_left.x, rect.top_left.y);
    println!("Rectangle Bottom Right: ({}, {})", rect.bottom_right.x, rect.bottom_right.y);

}
