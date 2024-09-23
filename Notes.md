# Primitives

-*let* - keyword used to introduce new variables into the current scope  
-*mut* - keyword used to make the variable, reference or pointer mutable  
  
-arrays - contiguous memory chunks holding a number of objects, each of the same type *T*  
    &nbsp;&nbsp;&nbsp;&nbsp;-length must be known at compile time  
    &nbsp;&nbsp;&nbsp;&nbsp;-*let xs: [i32; 5]* ---> declaration of *xs* array, holding 5 members, each being i32 data type  
  
-slices - similar to arrays  
    &nbsp;&nbsp;&nbsp;&nbsp;-their length is NOT known at compile time
    &nbsp;&nbsp;&nbsp;&nbsp;-it is a 2 word object : the first word is a pointer to the data, and the second word is length of the data slice  
    &nbsp;&nbsp;&nbsp;&nbsp;-keep in mind that when you are slicing an array, *starting index* includes thah index value in the slicing process, while *end index* value does not include it  
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-example : let xs: [i32; 5] = [1,2,3,4,5];  
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;starting index = 0; end index = 3  
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Because of the chosen indices, values 1,2,3 are printed because their indices are 0,1,2. end index values does NOT get printed  


```Rust
for i in 0..xs.len() + 1
    {
        match xs.get(i)
        {
            Some(xval) => println!("{} : {}", i, xval),
            None => println!("Issue accessing xs with index {}", i),

        }
    }
```  
-what does the code block above do?  
  
```Rust
for i in 0..xs.len() + 1
```  
-beautiful for loop implementation thanks to `xs.len()` property  
  
```Rust
match xs.get(i)
```  
-`xs.get(i)` allows us to safely access members of `xs` array with the goal of avoiding unsage accesses (ie. accessing array index not exising in the array)  
-`match` allows us to check whether `xs.get(i)` returns something useful  
  
```Rust
Some(xval) => println!("{} : {}", i, xval),
None => println!("Issue accessing xs with index {}", i);
```  
-keywords `Some(xval)` and `None` work in conjunction with `match xs.get(i)` to check whether the value we've gotten back by doing `xs.get(i)` is something useful, or `None`  

# Custom types
-two keywords are used to create custom structures : `struct` and `enum`  
  
-rust supports 3 types of structs :   
&nbsp;&nbsp;&nbsp;-tuple structs (basically named tuples)  
&nbsp;&nbsp;&nbsp;-conventional C structs  
&nbsp;&nbsp;&nbsp;-unit structs (useful for generics)  
 
-what is this? It can be seen before the `Person` struct definition. `#[derive(Debug)]`  
&nbsp;&nbsp;&nbsp;-the compiler is capable of providing a basic implementation for some traits  
&nbsp;&nbsp;&nbsp;-these traits can be overriden, and implemented manually to exercise more control over it  
&nbsp;&nbsp;&nbsp;-in our example, `#[derive(Debug)]` preceeds `struct Person` definition, which allows us to gain basic info regarding the struct once we have an instance of it; in other words, it allows us to do simply `println!("{:?}", spongebog)`, and have the compiler print this :  
`Person { name: "Spongebob", age: 20 }`  
  
## Enums  
-just like in C, allows creation of an unchangeable set of values  

-rust has two different types of `constants` which can be declared in any scope (including global)  
-`const` and `static`  
-`const` - unchangeable value  
-`static` - static keyword can be observed through two different lenses  
&nbsp;&nbsp;&nbsp;-as a reference lifetime `'static` indicates that the data pointed by the reference lives for the remaining lifetime of the running program  
&nbsp;&nbsp;&nbsp;-as a trait bound `static` means that the type does not contain any non-static references  

# Variable bindings  
-rust is statically typed, thus providing type safety  
-however, in most cases, compiler will be able to properly infer the type of variable from the context  
-variable bindings have a scope - they are constrained to blocks, blocks being collections of statements between curly braces  
-variable shadowing (name masking) is also available (https://en.wikipedia.org/wiki/Variable_shadowing) 
-mutablity is not transferable via variable shadowing  
&nbsp;&nbsp;&nbsp;&nbsp;-if for some reason you have variable `a` both before and in the `for` loop block, and the variable `a` before `for` loop is `mut`, the `a` variable in the `for` loop will not be mutable if it hasn't been declared as being mutable - **THIS IS CALLED FREEZING**  
-declaration without definition is permitted  
  
# Types  
-rust provides no implicit type conversion between primitive data types  
-explicit conversion is done using `as` keyword  
-since v1.45, using `as` keyword performs a *saturating cast*  
&nbsp;&nbsp;&nbsp;-saturating cast means that when we are casting from float to int, if the floating point exceeds the upper bound, or is less than the lower bound, the returned value will be equal to the bound crossed  
-Rust also provides us with type aliasing (`UpperCamelCase` must be used)  
```Rust
type NanoSecond = u64;  // now instead of declaring a variable being u64 data type, we will decalare it as NanoSecond data type because in our project we often use variables to store soem nanosecond scale times
type Inch = u64;

fn main()
{
    let nanoseconds: NanoSecond = 5 as u64;
}
```
  
# Conversion  
-`From` and `Into` keywords allow us to perform data type conversions using some variable as a data type example  
-`From` allows us to define how a variable will create itself based on some other variable's data type
```Rust
let my_str = "hello";
let my_String = String::From(my_str);   // my_str is a str - it is a temporary, read-only object | my_String is String - it is a growable, heap based data structure used to store a sequence of UTF-8 characters
```  
-`Into` enables us to convert a type into another type  
  
-`TryFrom` and `TryInto` are used for fallible conversions  
  
# Expressions  
-besided ordinary expressions such as declaring a variable, calling functions, and many more, blocks of code are also expressions, meaning the can also be used as values in assignments  
```Rust
fn main()
{
    let x = 5u32;

    let y = 
    {
        let x_squared = x*x;
        let x_cube = x_squared*x;

        x_cube + x_squared + x  // This is the return value 
        // Remember, if the last expression of block ends with a semicolon, the return value will be ()
    };

    let z = 
    {
        2*x;
        // Because you've used semicolon here, z will always return ()
    };
}
```

# Flow of control  
-rust enables creating infinite loops using a simple `loop` keyword  
-when nesting loops, it is possible to `breka` or `continue` outer loops from within inner loops (check out `flow_control.rs`)  
&nbsp;&nbsp;&nbsp;-to do this, loops must be annotated with some chosen label  
  
-pay attention to line 32 in `flow_control.rs` - the `match name` line  
&nbsp;&nbsp;&nbsp;-in the `match name` block the matching is done using `&'Ferris'` - why is that?  
&nbsp;&nbsp;&nbsp;-because `match name` clause expects `&&str` variable, meaning that you have to pass `&var` to it  
&nbsp;&nbsp;&nbsp;-a bit more about the *borrowing* operator (&) can be found here : https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html  
  
-by default, `for` will apply the `into_iter` function to the collection we want to iterate  
-what is the difference between `iter()` and `into_iter()` when creating iterable objects from collections/vectors/whatever?  
&nbsp;&nbsp;&nbsp;-`iter` borrows each element of the collection through each iteration; therefore, the collection we are iterating is left untouched and is available in its original form for reuse after the loop; It is exactly this *borrowing* that makes us have to do `&var` when doing `match name`  
&nbsp;&nbsp;&nbsp;-`iter_loop` consumes the collection so that on each iteration the exact data is provided; Once the collection has been consumed in the for loop, it is no longer available for reuse as it has been "moved" to the for loop  
  
-rust provides pattern matching via the `match` keyword, which can be used like `switch` is used in C  
  
-binding allows us, via `@` operator, to do the pattern-matching for a value, and also the usage of that value as well  
  
## if let  
-`if let` permits patterns matching within the condition of an `if` statement  
-for example, let's say we have some sort of `Option <T>`, and we want to call some function if it is `Some<T>`, but do nothing if it's None - using `match` this would look like this :  
```Rust
match option{
    Some(x) => { foo(x) },
    None => {},
}
```  
-`if let` allows us to make this prettier :  
```Rust
if let Some(x) = option{
    foo(x);
} else{
    bar();
}
```  
  
# Functions  
-*associated functions* and *methods* will be skipped for the time being - they introduce OOP like behaviour  
  
-*closures* - functions without names; basically what lambdas are in python  
&nbsp;&nbsp;&nbsp;-`let increment = |x:i32| x+1`  
&nbsp;&nbsp;&nbsp;-`increment` - name of the closure which is called  
&nbsp;&nbsp;&nbsp;-`|x:i32|` - parameter and its type that we pass to the closure, ie. the value on which we want the closure to execute on  
&nbsp;&nbsp;&nbsp;-`x+1` - body of the closure; what the closure does  
  
-*diverging* functions never return, and are marked using `!`  
