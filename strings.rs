/*
	Using &str as function parameter is comarable to using pointers in C.
	Both work with addresses/references, except that &str in Rust is considered
	a fat pointer because it carries both the address of the first byte and the
	length of the slice.

	
*/
fn dump(s: &str)
{
    println!("str '{}'", s);
}

fn main()
{
    let text = "Hello there"; // string slice example (&str)
    let s = text.to_string(); // this is now an allocated string
    
    dump(text);
    dump(&s);

    /*
     *  text --> give me a reference to this literal that already exists in the program image. This
     *  value lives in ROM, it is baked in, no heap manipulation. It is a reference to a pre‑existing, immutable chunk of memory.
     *
     *  to_string() function literally tells to Rust to create a new, owned String data type. This
     *  means it allocates memory on heap, copies passed string to that chunk, and returns a String
     *  object which owns the buffer. 
     *
     * */

	let mut s = String::new();
	s.push('H'); //notice we are using '', not ""
	s.push_str("ello"); //used to push a string, not just one char
	s.push(' ');
	s += "World";	// short-hand way for push_str()
	s.pop();	//removes last char
	dump(&s);


    // Example of breaking up strings : 
    let text = "A system of cells interlinked within";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("text split by whitespace : {:?}", words);

    /*
        1.Why does "words" have to be defined as Vec<&str> data type?
        2. Why is string slice used (&str) instead of plain string?

        1. It must be explicitly defined as Vec<&str> data type because
            function split_whitespace() yields items of type &str, ie. a slice
            representing each subunit from the orignal string.
            Because we call collect() Rust must know what kind of collection type we
            want to build. Since split_whitespace() returns string slice we use it as
            the collection type.

        2. split_whitespace() does not allocate new strings - it instead gives us slices
            (&str). This is why we must use Vec<&str> - we basically have a borrowed view
            into the original string.
    */

    // Different way of doing the same thing as above - this approach allocates space to keep the slice created by split_whitespace()
    let mut newWords = Vec::new();
    newWords.extend(text.split_whitespace());
    println!("newWords : {:?}", newWords);
}
