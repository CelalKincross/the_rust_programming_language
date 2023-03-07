// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sing_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sing_in_count: 1, 
//     }
// }

/*
Shorthand -----> no need to write email, and username, twice.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

create a new user from user1 
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count:user1.sign_in_count,
};

'..' syntax means the fields not explicitly set should be the same as user1
let user2 = User {
    email: String::from("anotheremail@example.com"),
    ..user1
}; // user 1 is moved to user2 and is no longer valid
 */

// Tupple structs

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);



// fn main() {
//     let mut user1 = User{
//         email: String::from("abc@example.com"),
//         username: String::from("someuser"),
//         active: true,
//         sing_in_count: 1,
//     };

//     let black = Color(0,0,0);
//     let orgin = Point(0,0,0);
//     println!(orgin.1) // equals 0

//     user1.email = String::from("anotheremail@example.com");
// }

// let rect1 = (30, 40);

// fn area(dimensions: (u32, u32))-> {
//     dimensions.0 * dimensions.1
// }

// //use structs to make clear

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// let rect1 = Rectangle {
//     width: 30,
//     heigth: 50,
// };

// fn area(retangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

/*Ownership of Struct Data
In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:

Filename: src/main.rs

This code does not compile!
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // println!("rect1 is {:#?}", rect1);     //{:?} works too but :#? is a prettier
    dbg!(&rect1);
}