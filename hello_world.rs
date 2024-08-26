// Beautiful guides and docs by Rust team.

fn main()
{
    println!("Henlo world!\n");

    // To check out all kinds of printing options/functions/macros, look here : https://doc.rust-lang.org/std/fmt/

    println!("It's a {}", "trap");
    println!("Welcome to Section {}", 9);   // Nice, very python-like

    println!("{0} {2} {1} {1}", "Potato", "Spongebob", "Kalamarko");    // Positional arguments - int in curly braces indicates which of the arguments we want to print

    println!("{data_list1} {data_list2} {data_list3}", data_list1 = "Star Wars", data_list2 = "mucho", data_list3 = "gusto");

    // Format specifiers - messing around with numeral systems looks very nice
    println!("Base 10 : {}", 69420);    // Rust team hehehe 69420
    println!("Base 2 : {:b}", 69420);
    println!("Base 8 : {:o}", 69420);
    println!("Base 16 : {:x}", 69420);

    // Rust also supports a lot of other formatting options, such as controlling padding, right-justifying, etc. Check the docs.
}

// Regular one line comment like in C
/*Regular block style comment like in C*/


// /// This is a DOC comment which gets parsed to HTML libarary documentation; triple forward slash means "generate library docs for the following item"
// //! This means "generate libarary docs for the enclosing item" --> if left without line comment demarcation, an error will be thrown by the compiler because //! must appear before items we want to document
