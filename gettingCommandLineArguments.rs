/*
https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#interlude-getting-command-line-arguments
*/
fn main()
{
    for arg in std::env::args()
    {
        println!("'{}'", arg);
    }

    //collect all arguments into a vector
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0
    {
        println!("args present : {:?}", args);
        let args_slice = &args; // we turn the Vec<String> object into a slice string, which allows us simple iterative access of each element
        for i in args_slice // so pythonic, much wow
        {
            println!("{}", i);
        }
        println!("args_slice[0] : {}", args_slice[0]); //example of direct access for string slice
        println!("args[0] : {:?}", args[0]); //example of direct access for String vector
    }
}
