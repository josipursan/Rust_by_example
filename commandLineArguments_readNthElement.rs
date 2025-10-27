/*
https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#interlude-getting-command-line-arguments
*/

use std::env;
fn main()
{
    // Flavor 1
/*    let first_element = env::args().nth(1).expect("Supply an argument");
    let n: i32 = first_element.parse().expect("Not an integer");
    println!("n : {}", n);*/
    
    //  Flavor 2
    let collectedArgument = env::args().nth(1);
    match collectedArgument
    {
        Some(val) => println!("Collected argument : {}", val),
        None => println!("Naughty, naughty, you must pass an argument!"),
    }

    /*
    Flavor 1 - collecting command line arguments without checking whether something
    was actually passed will cause a panic and dirty exit.

    .nth(1) is actually used to grab first element from command line args - if they were empty there wouldn't be nth(1) element.

    ============================================

    Flavor 2 - handles incorrect usage of command line arguments
    If user does not supply command line argument in position nth(1) we handle it
    in match{} block using Some() and None.
    If Some(val) is found (ie. the user has passed some value), we print the value.
        NOTE : data type should also somehow be checked.
    If None is found we cleanly exit avoiding the panic.
    */
}
