/*
 * Vectors - resizeable arrays
 * Behaves very much like slices, except a vector can be added values.
 *
 * */

fn dump(arr: &[i32])
{
    println!("arr : {:?}", arr);
}

fn main()
{
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];
    let maybe_first = v.get(0);

    println!("v : {:?}", v);
    println!("first : {}", first);
    println!("maybe_first : {:?}", maybe_first);

    dump(&v);

    let slice = &v[1..]; //slices, ie. removes first element from vector
    println!("slice is : {:?}", slice);
}
