fn main()
{
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();  // Because this variable is unused, and is not prefixed by underscore, compiler throws a warning.

    let copied_integer = an_integer;

    println!("An integer {:?}", copied_integer);
    println!("A boolean {:?}", a_boolean);

    // Why the underscore? underscore allows us to silence compiler's warning about unused variables - this is so neat!
    let _unused_variable = 3u32;

    //Mutability
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation : {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation : {}", mutable_binding);
    //_immutable_binding += 1;  This would cause the compiler to error out, because mut modifier has not been used on _immutable_binding

    

}