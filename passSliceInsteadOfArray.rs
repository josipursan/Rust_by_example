/*
 * https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#arrays-and-slices
 *
 * Arrays are fixed in size.
 * Arrays are either mutable or immutable - irregardless of their mutability they are still fixed
 * in size.
 * Representation of array includes its type.
 * Example :
 *      let arr = [1,2,3,4] --> this array is made up of 4 i32 elements.
 *      Its type is [i32, 4]
 *
 *      let arr2 = [1,2] --> this array is made up of 2 i32 elements
 *      Its type is [i32, 2]
 *
 * This makes passing arrays around problematic.
 * Because of this we use slices.
 *
 * Slices are subsets of some array - they are comparable to arrays in every respect, except in
 * size.
 * */


/*
 * Notice signature of sum() function - slice is declared using "&[i32]".
 * We literally say "hey, I'm gonna pass you address of a chunk of memory with i32 members in it".
 *
 * In C operator & represents "address of".
 * In Rust operator & is called "borrow".
 * Borrow represents every pass by reference. Anything borrowed is still owned by its original
 * owner.
 *
 * */

fn sum(values: &[i32]) -> i32
{
    let mut res = 0;
    for i in 0..values.len()
    {
        res += values[i];
    }
    res // return value
}

fn main()
{
    let arr = [10, 20, 30, 40];
    let sum_res = sum(&arr);
    println!("sum_res : {}", sum_res);
}
