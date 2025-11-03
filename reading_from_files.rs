use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

// Flavor 1 - no error handling
/*
fn main()
{
    let first = env::args().nth(1).expect("Please supply a filename");

    let mut file = File::open(&first).expect("Can't open this file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("Cant read the file");

    println!("Given files has {} bytes", text.len());
}*/

// Flavor 2 - with error handling
fn read_file_to_string(filename: &str) -> Result<String,io::Error>
{
    let mut file = match File::open(&filename)
    {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text)
    {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn main()
{
    /*let file = env::args().nth(1);
    match file
    {
        Some(val) => println!("Argument present"),
        None => return Ok(-1),
    }
    let text = read_file_to_string(&file).expect("Not a filename!");
    println!("Passed file has {} bytes", text.len());*/


    // Not a bad way, but too C-like
    /*
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0
    {
        println!("Passed filename : {:?}", args[0]);
        //let args_slice = &args;
        let text = read_file_to_string(&args[0]).expect("Not a filename");
        println!("Text : {:?}", text);
    }
    else
    {
        println!("No args passed");
    }*/

    let filename = env::args().nth(1).expect("No filename provided");
    let text = std::fs::read_to_string(&filename).unwrap_or_else(|_| panic!("Could not read file : {}", filename));
    println!("text : {:?}", text);

    /*
    I still dislike using built-in stuff such as .expect() or .unwrap_or_else().
    When something fails (e.g. because a command line param wasn't passed) I want a clean exit showing only the error message/code.
    */
}
