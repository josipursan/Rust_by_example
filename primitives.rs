use std::mem;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool, i32)
{
    let (int_param, bool_param) = pair; // let keyword can be used to bind the members of a tuple to variables
    (bool_param, int_param) // I assume this is considered as the return value, and -> (bool, i32) in function name defines the return data type
}


fn analyze_slice(slice: &[i32])
{
    println!("First element of slice : {}", slice[0]);
    println!("Length of slice : {}", slice.len());
}

fn main()
{
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular data annotation, ie. specifying data type
    let an_integer = 5i32;  // Suffix data annotation

    let default_float = 3.0; // f64 is default float
    let default_integer = 7; // default int is i32

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;   // mut keyword specfies a variable's value can be changed
    mutable = 21;
    //mutable = true; // as expected, throws error : expected integer, found bool; Very nice error handling where you are basically taken by your hand and pointed what error to look at : `rustc --explain E0308`

    let mutable = true; // Variables can be overwritten using shadowing

    println!("1 + 2 = {}", 1u32 + 2);   // Integer addition

    println!("1 - 2 = {}", 1i32 - 2); // Integer subtraction

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3); //Scientific notation!

    // Bool logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // Noooooice
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:08b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); // Nooooooooice
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("Improve readability using underscores : {}", 1_000_000u32);



    // Tuples

    // A tuple with data types mixed up
    let long_tple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

    //Example of how tuple values are accessed
    println!("Long tuple first value {}", long_tple.0);
    println!("Long tuple second value {}", long_tple.1);

    // Tuples can be tuple members - nesting
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple_of_tuples : {:?}", tuple_of_tuples);    // The whole tuple can be printed
    

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));


    println!("One element tuple : {:?}", (5u32,));  // One element tuples can be created by using the comma after our value
    println!("Just an integer : {:?}", (5u32));

    // Tuple destructuring
    let tuple = (1, "hello", 4.5, true);
    let(a,b,c,d) = tuple;   // Now values from tuple are positionally assigned to a,b,c,d
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("We can even create matrices! {:?}", matrix);

    //Arrays and slices
    //Fixed-size array
    let xs: [i32; 5] = [1,2,3,4,5];

    // initializing all 500 elements to value 0
    let ys: [i32; 500] = [0; 500];

    // Array length can be checked using .len() property
    println!("Numer of elements in array {}", xs.len());

    //Naturally, arrays are on stack
    println!("xs arrays occupies {} bytes of memory", mem::size_of_val(&xs));
    
    //Arrays can automatically be borrowed as slices
    println!("Borrowing the whole array as a slice");
    analyze_slice(&xs);

    println!("Borrowing a section of the array as a slice");
    analyze_slice(&ys[0 .. 5]);     //indices 0,1,2,3,4 get handled/printed, whatever was intended.

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);   //assertion to check whether empty_array is really empty

    
    // Explanation of this code block is in Notes.md
    for i in 0..xs.len() + 1
    {
        match xs.get(i)
        {
            Some(xval) => println!("{} : {}", i, xval),
            None => println!("Issue accessing xs with index {}", i),
        }
    }
}