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
-variable shadowing (name masking) is also available  
-declaration without definition is permitted  