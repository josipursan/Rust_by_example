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
}   