// Part one

// fn main() {
//     {// literal sting not valid yet
//         let literal_string = "hello";
//         // immutable
//         // do stuff with literal string
//     } // out of scope, literal sting no longer valid

//     {
//         let mut a_string = String::from("hello");
//         // mutable
//         a_string.push_str(", world!");
//         println!("{a_string}");

//         let another_string = a_string;
//         // a_string is a pointer and another string is also a
//         // pointer. when the both go out of scope they are 
//         // both dropped causing a double free error where 
//         // the computer is trying to free the same memory
//         // twice
//         // Rust avoids this by considering a_string no longer
//         // valid at line 14
//         // below would be an error
//         // let s1 = String::from("hello");
//         // let s2 = s1;
    
//         // println!("{}, world!", s1);

//         // If you’ve heard the terms shallow copy and deep copy 
//         // while working with other languages, the concept of 
//         // copying the pointer, length, and capacity without 
//         // copying the data probably sounds like making a shallow 
//         // copy. But because Rust also invalidates the first 
//         // variable, instead of calling it a shallow copy, it’s 
//         // known as a move. In this example, we would say that s1 
//         // was moved into s2. So what actually happens is shown 
//         // in Figure 4-4.
//     }

//     {

//         let s1 = String::from("hello");
//         let s2 = s1.clone();
    
//         println!("s1 = {}, s2 = {}", s1, s2);   
//     }
//     // deep copy using .clone() and is valid code
// }


// Part 2

// fn main() {
//     let s = String::from("hello");  // s comes in this scope

//     takes_ownership(s);   // s 's value moves into the function ..
//                         // ... and is no longer valid here

//     let x = 5;           // x comes into scope

//     makes_copy(x);       // x would move into the function,
//                         // but i32 is Copy, so it's oke to still 
//                         // use afterward
//     // println!("{}", s); does not compile
//     println!("{}", x);
// } // Here, x goes out of scope (dropped? nope, its on the stack so it is 
//     //automatically popped), then s. But because s's value was moved, 
//     // nothing special happens. 

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } //here, some_string goes out of scope and 'drop' is called. the backing 
//   // memory is freed

// fn makes_copy(some_int: i32) {
//     println!("{}", some_int);
// } // Here, some_int goes out of scope. Nothing special happens.

// If we tried to use s after the call to takes_ownership, 
// Rust would throw a compile-time error. These static checks protect us 
// from mistakes. Try adding code to main that uses s and x to see where 
// you can use them and where the ownership rules prevent you from doing so.



// Part 3: More Examples
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


// transferring ownership can be tedious if you need to use the orginal
// data after it is moved to a function the example below uses 
// tuples to transfer the data back. 
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// "References" are a better solution and is the next section


//Notes
// The Compiler Is Not Perfect
// A common issue in statically-typed languages is that the compiler can reject programs which would execute without a problem. For example, take this program:

// This code does not compile!
// fn main() {
//   let x = if true { 1 } else { "hello" }; //error because "hello" is a string
//   assert_eq!(x + 1, 2);
// }
// If this program were allowed to execute, x would always be 1, and so computing x + 1 would always work without issue. That's because the if condition is true. But the compiler ignores the actual value of the condition, and assumes either branch could be executed. Therefore this program is rejected because x cannot be both a number and a string.

// Similarly, the compiler may reject programs that seem acceptable under the rules of ownership. For example, like this program:

// This code does not compile!
// fn move_two(s1: String, s2: String) -> String {
//   s1
// }



// fn main() {
//   let (s1, s2) = (String::from("a"), String::from("b"));
//   let s3 = move_two(s1, s2);  // both moved so both not accessible later even though only s1 is used
//   println!("{} {}", s2, s3);
// }
// Even though s2 is never used or returned in move_two, the compiler still considers s2 to be moved when calling move_two.

// So as you're learning about ownership and Rust, keep in mind that the compiler's safety checks often assume the worst case about your code. Part of learning to write Rust is convincing the compiler that the worst case is still safe!