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
struct Rectangle    // This reminds me a bit of OOP...
{
    top_left: Point,
    bottom_right: Point,
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

    

}

