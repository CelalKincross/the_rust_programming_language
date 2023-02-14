// fn main() {
//     // Must state the type when you are changing types in the code cannot remove the u32 below because of the parse to integer and error handling
//     let guess: u32= "42".parse().expect("Not a number!");
//     println!("Guess: {guess}");

//     // Scalar types: represents a single value
//     // Integers
//     // Floating-point numbers
//     // Booleans
//     // Characters
// //     Length	    Signed	Unsigned
// //      8-bit	    i8	        u8
// //      16-bit	    i16	        u16
// //      32-bit	    i32	        u32
// //      64-bit	    i64	        u64
// //      128-bit	    i128       	u128
// //      arch	    isize   	usize
// // Number literals	Example
// // Decimal	98_222
// // Hex	0xff
// // Octal	0o77
// // Binary	0b1111_0000
// // Byte (u8 only)	b'A'

// // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. We’ll discuss this topic in detail in “Storing UTF-8 Encoded Text with Strings” in Chapter 8.
// ///// Commpound types //////
// // / Tupple
// // tuple groups a number of values with a variety of types together. 
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

// //destructuring a tuple
//     let (x, y, z) = tup;

//     println!("The value of y is {y}");

//  // access tuples using periods (.)

//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;

//     println!("The first value is {five_hundred}, the second is {six_point_four} and the last is {one}");

//  // Arrays
// //  / same types and fixed length
// //  / Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) 
// //  / vectors are allowed to grow or shrink
 
//     let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

//     let a: [i32; 5]= [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];
//     println!("Array values {first}, {second}");

// }
    use std::io;

    fn main() {
        let a = [1, 2, 3, 4, 5];
    
        println!("Please enter an array index.");
    
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index];
    
        println!("The value of the element at index {index} is: {element}");
    }
    
