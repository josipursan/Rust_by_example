/*
 * Vectors - resizeable arrays
 * Behaves very much like slices, except a vector can be added values.
 *
 * Vectors have size and capacity.
 * If you run clear operation on vector its size becomes 0.
 * However, it still has a capacity to hold e.g. 5 values.
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

    let mut v1 = vec![10, 20, 30, 40];
    v1.pop(); //remove last element from v1
    println!("v1 : {:?}", v1);

    let mut v2 = Vec::new();
    v2.push(10); //adding elements to v2
    v2.push(20);
    v2.push(30);
    println!("v2 : {:?}", v2);

    assert_eq!(v1, v2); //compare v1 and v2

    v2.extend(0..2);    // add elements to v2 by defining a range of values to be added
    println!("v2 after extend operation : {:?}", v2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]); //compare v2 and locally craeted slice


    let mut v3 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    println!("original v3 : {:?}", v3);
    v3.sort();
    v3.dedup(); //removes duplicates from given vector
    println!("v3 after sort and dedup : {:?}", v3);
    assert_eq!(v3, &[1, 2, 5, 10, 11, 40]);

}
