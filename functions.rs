fn foo() -> !
{
    panic!("Example of diverging function");
}

fn main()
{
    do_fizzbuzz_to(15);

    // Closures - basically lambdas in python, enabling us to create functions without names
    let print_text = || println!("Example of closure");
    print_text();

    let increment = |x:i32| x+1;
    println!("increment closure : {:?}", increment(2));

    foo();
    println!("This will not get printed");

}

fn is_divisible(left_digit:u32, right_digit:u32) -> bool
{
    if(right_digit == 0)
    {
        println!("Right digit is zero! Division not possible!");
        return false;
    }

    left_digit % right_digit == 0   // last expression in function without ; is considered return value
}

fn fizzbuzz(val:u32)
{
    println!("val : {:?}", val);
    if(is_divisible(val, 15))
    {
        println!("FizzBuzz");
    }
    else if(is_divisible(val, 5))
    {
        println!("Buzz");
    }
    else if(is_divisible(val, 3))
    {
        println!("Fizz");
    }
}

fn do_fizzbuzz_to(n:u32)
{
    for n in 1..=n
    {
        fizzbuzz(n);
    }
}