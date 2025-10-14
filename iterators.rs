fn main()
{
    let arr = [10, 20, 30];
    for i in arr.iter() //more efficient than doing for i in 0..slice.len(){}
    {
        println!("i : {}", i);
    };

    let slice = &arr;
    for i in slice  // slices get implicitly converted to iterators
    {
        println!("{}", i);
    };

}
