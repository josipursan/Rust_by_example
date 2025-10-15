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

    // Example of python-esque sum up operations : 
    let sum:i32 = (0..5).sum();
    println!("sum : {}", sum);

    let sum_2:i64 = [10 ,20 ,30].iter().sum();
    println!("sum_2  {}", sum_2);
    /*
     * Notice that you explicitly stated the data type for both sum and sum_2.
     * Why?
     * Because sum() function returns a generic type S. Rust doesn't know which numeric
     * type I want unless it can infer it from context
     * https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
     * */
    ///////////////////////
    
    let ints = [1,2,3,4,5];
    let my_slice = &ints;

    for s in my_slice.windows(2)
    {
        println!("s : {:?}", s);
    };
    /*
     * windows() method gives an iterator of slices.
     * It basically chunkifies the initial slice into as many subslices made up of N values, where
     * N is the value you pass to windows() function.
     * So our example above says "hey, create subslices, ie. windows, each made up of 2 elemtns
     * from the initiali slice".
     *
     * Each window's starting element is the end element of the previous window.
     *
     * e.g. W_0 = [0,1]
     *      W_1 = [1,2]
     *      W_2 = [2,3]
     * */

    for c in my_slice.chunks(2)
    {
        println!("c : {:?}", c);
    };
    /*
     * Very similar to windows, except that the starting element of chunk C is the element
     * after the ending element of chunk C-1
     * e.g. : C_0 = [0,1]
     *        C_1 = [2,3]
     *        C_2 = [4,5]
     * */
}
