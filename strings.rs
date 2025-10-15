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


    // The notation used for slices works with strings as well: ...
}
