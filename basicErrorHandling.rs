fn good_or_bad(good: bool) -> Result<i32,String>
{
    if good
    {
        Ok(42)
    }
    else
    {
        Err("bad".to_string())
    }

}

fn main()
{
    println!("{:?}", good_or_bad(true));
    println!("{:?}", good_or_bad(false));


    match good_or_bad(true)
    {
        Ok(n) => println!("No problem : {}", n),
        Err(e) => println!("A problem : {}", e)
    }
}

/*
    Here we implement very basic error handling leveraging Rust's built in mechanisms.

    File::open (https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open) has return type Result<File>, which aliases Result<T> that returns either Ok(T) or Err(Error).

    This means, when we are doing IO operations, we can either expect Ok(T) or Err(Error) as return from the run IO operation.

    Instead of leaving it to chance, we need to handle these ourselves.
*/
