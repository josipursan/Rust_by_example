fn main()
{
    'outer: loop
    {
        println!("Entered outer loop");

        'inner: loop
        {
            println!("Entered inner loop");

            break 'outer;   // With this label we break from the outer loop
        }

        println!("This println never prints because of break 'outer");
    }
    println!("Exited outer loop");


    // for in construct is used to iterate
    for i in 0..10 //a..b range notation yields values from a (inclusive) to b (exclusive)
    {
        println!("i : {:?}", i);
    }
    for j in 0..=10 // a..=b range notation includes both ends of given range
    {
        println!("j : {:?}", j);
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter()
    {
        match name
        {
            &"Ferris" => println!("Le potato"),
            _ => println!("Pumpikin pie"),
        }
    }
    println!("names vector : {:?}", names);

    for name in names.into_iter()
    {
        match name
        {
            "Ferris" => println!("Babababababanana potato aaaaaaa"),
            _ => println!("Spongebob"),
        }
    }
    //println!("names vector : {:?}", names);   // Compilation fails here because vector names has been consumed due to iter_loop usage

    let mut names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.iter_mut()
    {
        *name = match name
        {
            &mut "Ferris" => "Awwwww yiss",
            _ => "EoEo",
        }
    }
    println!("names : {:?}", names2);
    // iter_mut() enables us to modify the vector we've used to create iterator

    // Match
    let number = 13;
    println!("Now we will try to match!");
    match number 
    {
        1 => println!("Found 1\n"),
        2 | 3 | 5 | 7 | 11 => println!("2,3,5,7,11\n"), // Matching several values
        13..=19 => println!("Found value in range [13,19] : {:?}", number), // Matching done using a range of values
        _ => println!("Given value does not match anything in this match clause! {:?}", number),    // _ operator usage is similar to python
    }

    let boolean_match = true;
    let binary = match boolean_match  // match keyword also allows us to construct expressions, ie. callable blocks of code
    {
        false => 0, // Arms of a match clause must cover all possible values if we are using it as an expression (I guess this is because it is being used as an expression)
        true => 1,
    };
    /*
        If you comment out one of the arms of match expression above, compiler will complain that "pattern true is not covered" : `rustc --explain E0004`
    */
    println!("{} -> {}", boolean_match, binary);


    // Match can be used to destructure various data structures
    let three_element_tuple = (0, -2, 3);
    match three_element_tuple
    {
        (0, y, z) => println!("y : {:?}\tz : {:?}", y, z),
        (1, ..) => println!("First element is 1, the rest does not matter"),
        (.., 3) => println!("Last element is 3, other elements do not matter"),
        (2, .., 3) => println!("First element is 2, last element is 3, other elements do not matter"),
        _ => println!("No matches given the match conditions"),
    }

    let array_three_elements = [1, -2, 6];
    match array_three_elements
    {
        [zeroth, first, second] => println!("Three elements : {}, {}, {}", zeroth, first, second),
        [1, _, second] => println!("Zeroth element is 1, first is ignored, second is {}", second),  // ignoring valut at index 1
        [-1, first, ..] => println!("Zeroth el is -1, first element is {}, other elements are ignored", first),
        [zeroth, middle @ .., last] => println!("zeroth el {}\nmiddle values stored as another array {:?}\nlast el from original array : {}\n", zeroth, middle, last),
    }

    // Enum destructuring example
    enum Color
    {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    match color
    {
        Color::Red => println!("This is red"),
        Color::Blue => println!("This is blue"),
        Color::Green => println!("This is green"),
        Color::RGB(r, g, b) => println!("Red : {}, green : {}, blue : {}", r, g, b),
        Color::HSV(h, s, v) => println!("H : {}, S : {}, V : {}", h, s, v),
        Color::HSL(h, s, l) => println!("H : {}, S : {}, L : {}", h, s, l),
        Color::CMY(c, m , y) => println!("C : {}, M : {}, Y : {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("C : {}, M : {}, Y : {}, K : {}", c, m, y, k),
    }

    // Destructuring for pointers/references
    let value = 5;
    match value
    {
        ref r => println!("Reference to value {:?}", r),
    }


    // Structs can also be destructured
    struct Foo
    {
        x: (u32, u32),
        y: u32,
    }

    let foo_struct = Foo{x: (1,2), y: 3};

    match foo_struct
    {
        Foo {x: (1,b), y} => println!("x.0 is 1, b : {}, y : {}", b, y),
        Foo {y: 2, x: i} => println!("y is 2, i is {:?}", i),   // When destructuring we can reaname the variables easily - x now becomes variable i
        Foo {y, ..} => println!("y is {}, others are ignored", y),
    }

    // struct can be destructured without match block
    let faa = Foo{x: (5,6), y: 8};
    let Foo{x: x0, y: y0} = faa;
    println!("x0 : {:?}, y0 : {}", x0, y0);

    // Match guards can be used to further refine which match arm will be entered
    enum Temperature
    {
        Celsius(i32),
        Fahrenheit(i32),
    }
    let temperature = Temperature::Celsius(35);
    match temperature
    {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30!\n", t),
        Temperature::Celsius(t) => println!("{}C is below 30", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86", t),
    }
}   