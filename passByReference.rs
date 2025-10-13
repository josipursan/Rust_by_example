fn pass_by_reference(x: &mut i32)
{
    *x+=1;
}

fn main()
{
    let mut value = 1;
    pass_by_reference(&mut value);
    println!("value after modification : {}", value);
}
