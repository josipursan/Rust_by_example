fn main()
{
    let a = 5;
	let b = &a;

	println!("a : {}\t&a : {:p}\n", a, &a);
	println!("b : {:p}\t*b : {}\t&b : {:p}\n", b, *b, &b);


	println!("a : {}\t&a : {}\n", a, &a);
	println!("b : {}\t*b : {}\t&b : {}\n", b, *b, &b);
}

/*
a : 5   &a : 0x7ffce9213f3c                                                                b : 0x7ffce9213f3c      *b : 5  &b : 0x7ffce9213f40                                        

a : 5   &a : 5
b : 5   *b : 5  &b : 5
___________________

What's the difference between the top println! pair and the bottom one?
Both pairs use same operators on variables (&, *) in same places.

However, the difference lies in format specifiers - where we wanted to grab the address
we used {:p} format specifier.
Where we wanted the value, we used the normal {} format specifier.

This is the power of Rust - it figures out context on its own, and actually relies
more on it than what operator you chose.

Such smart behaviour might take some time getting used to, especially moving on from C
where you have to be mindful of what you do.
*/
