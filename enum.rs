enum WebEvent
{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64},
}

// inspect() function is used to return appropriate return value for the given enum value
fn inspect(event: WebEvent)
{
    match event
    {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click{x, y} => println!("clicked at {} {}", x, y),
    }
}

// Type aliases
enum aVeryLongNameJustLikeYouMakeThemAndThenHaveToChangeThemAfterReviews
{
    Add,
    Subtract,
}
type Operations = aVeryLongNameJustLikeYouMakeThemAndThenHaveToChangeThemAfterReviews;

enum Stage
{
    Beginner,
    Advanced,
}
enum Role
{
    Student,
    Teacher,
}

fn main()
{
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x: 20, y: 20};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // let x = Operations::Add; // example of how we can use type aliasing, ie. "Operations" now substitutes the long enum name, thus making code cleaner

    //Keyword "use" enables us to skip manual scoping, ie. to skip always doing Stage::Advanced or Stage::Beginner
    use crate::Stage::{Beginner, Advanced};
    // wildcard operator (*) is used to denote we want to use all names in Role
    use crate::Role::*;

    let stage = Beginner; // equivalent to doing Stage::Beginner
    let role = Student; // equivalent to doing Role::Student

    
    
}

