/*
 * https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#optional-values
 *
 * Array size - known at compile time
 * Slice size - run time info
 *
 * Using slice index out of bounds causes an out-of-bounds error and panic.
 * RUST DOES NOT HAVE EXCEPTION HANDLING - therefore simple try/catch blocks are not possible.
 *
 *
*/

// slice method get() does not panic
fn main()
{
    let ints = [1,2,3,4];
    let slice = &ints;
    let first_el = slice.get(0);    // slice[0] = 1
    let last_el = slice.get(5); // slice[5] = out-of-bounds
    println!("first {:?}", first_el);
    println!("last {:?}", last_el);

    /*
     * Output when compiled and run : 
     *  first Some(1)
     *  last None  
     * */
/*
 * Notice that for index == 0 we got "Some(1)", and for index == 5 we got "None".
 * Values "Some" and "None" are Option type.
 *
 * Option type has some useful actions, such as checking X.is_some() or X.is_none()
 * */
    println!("first_El Some or None? : {} {}", first_el.is_some(), first_el.is_none());
    println!("last_el Some or None? : {} {}", last_el.is_some(), last_el.is_none());

    /*
     * Some and None are wrapped - think of this as a sort of protection agains panic
     * Only when you unwrap it will the panic unfold.
     * */
    println!("first_el unwrapped : {}", first_el.unwrap());
    //println!("last_el unwrapped : {}", last_el.unwrap()); LEAVE COMMENTED OUT - CAUSES PANIC
    //BECAUSE INDEX 5 IS ACCESSING SLICE OUT OF BOUNDS

    /*
     * is_some() can be used to check whether we are dealing with a normal, non-panic causing
     * value, or if we are dealing with None
     *
     * Notice that slice.get(3) returns Option<&i32> - this is a reference, not the value!
     * This is why you must use deref operator when unwrapping - unwrap() would simply return you
     * the reference of the unwrapped value, and we want the value itself.
     *
     * */
    let check_last_for_none = slice.get(3);
    let _ = if check_last_for_none.is_some()
    {
        println!("Aaa-ha : {}", *check_last_for_none.unwrap());
    }
    else
    {
        println!("-1");
    };

    // Alternative ways for the snippet above : 
    /*
     * copied() turns the reference (Option<&i32>) into value (Option<i32>) by copying the integer
     * out
     * Main drawback is the copying.
     * */
    let last = slice.get(5).copied().unwrap_or(-1);
    println!("last value with copied : {}", last);
    /**/
    let _ = if let Some(&value) = slice.get(5)
    {
        println!("value : {}", value);
    }
    else
    {
        println!("-1");
    };

    /*
     * Another pretty way of doing this.
     * 
     * get() returns you a reference
     * To get value from reference you use deref operator *
     *
     * And now for the problematic part : unwrap_or(&-1)
     * Why &-1?
     * Because unwrap_or() takes the same data type as Option's inner type - in this case this is
     * return from get(), which is Option<&i32>, ie. a reference.
     * Think of it as this : & creates the address into which you put the default value (-1) for
     * output if unwrap is unsuccessful.
     *
     * Deref operataor (*) is applied to both get() or unwrap_or() outputs.
     * */
    let last_val = *slice.get(5).unwrap_or(&-1);
    println!("last_val : {}", last_val);
}
