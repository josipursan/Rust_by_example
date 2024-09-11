#![allow(dead_code)]    // Attribute to hide warnings for unused code

#[derive(Debug)]
struct Person
{
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point
{
    x: f32,
    y: f32,
}

/*
    structs can be reused as fields of another struct
    Here we have defined a rectangle by top left and bottom right corners in space
*/
struct Rectangle
{
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32
{
    let mut a:f32 = rectangle.bottom_right.x - rectangle.top_left.x;
    let mut b:f32 = rectangle.bottom_right.y - rectangle.top_left.y;
    let mut area = a*b;
    area = area.abs();
    area    
    // return keyword is not needed when the returned value is the last expression in the function, like it is here
    // otherwise, good old c return area; work just fine
}

fn main()
{
    // lets create a Person struct
    let name = String::from("Spongebob");
    let age:u8 = 20;
    let spongebob = Person{name, age};
    println!("{:?}", spongebob);

    // lets instantiate Point struct
    let point: Point = Point{x: 10.3, y: 0.4};
    let another_point:Point = Point{x: 5.2, y: 0.2};
    println!("Accessing struct point elements : {} {}", point.x, point.y);
    println!("another_point elements : {} {}", another_point.x, another_point.y);

    // Creating a new point by using struct update syntax, enabling us to use the fields of our other, previously instantiated Point struct
    let bottom_right = Point{x: 3.8, ..another_point};
    println!("bottom_right point elements : {} {}", bottom_right.x, bottom_right.y);

    // Destructuring a struc, ie. assigning names to variables
    let Point {x: left_edge, y: top_edge} = point;

    // struct instantiation is an expression too
    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    // Accessing element of pair
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Example of destructuring - instead of accessing elements with 0 and 1, we have assigned them names integer and decimal
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let test_rectangle = Rectangle {
        top_left: Point {x: 1.0, y: 3.0},
        bottom_right: Point {x: 4.0, y: 1.0},
    };
    let mut rectangleArea = rect_area(&test_rectangle);
    println!("Rectangle area is : {:?}", rectangleArea);
}

